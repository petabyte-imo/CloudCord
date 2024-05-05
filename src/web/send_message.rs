use crate::{
    errors::uh_oh,
    secrets::get_secret,
    web::{
        db::upload::UploadDatabase,
        encryption_helper::{encrypt_file, string_to_bytes},
    },
};
use std::{env::current_dir, fs::File, io::Read, path::PathBuf};

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::web::filefunctions::*;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};

pub async fn send_message(
    payload: MessagePayload,
) -> core::result::Result<String, impl IntoResponse> {
    let encrypt = |path: &str| {
        let key = string_to_bytes(&get_secret("ENCRYPTION_KEY"));
        let nonce = [0u8; 12];
        encrypt_file(&key, &nonce, path)
    };
    let encryption = get_secret("ENCRYPTION");

    //Handler print
    println!("->>  Starting to Send Messages To Discord");
    //Initialize database to store the uploaded files info
    let upload_db = match UploadDatabase::new().await {
        Ok(db) => db,
        Err(_) => return Err(uh_oh()),
    };
    //Initialize file info to send to the split_file_into_chunks function
    let file_name = &payload.file_name;
    let chunk_size = 25 * 1024 * 1024;
    // 10 MB

    //Get the chunk filenames from the split_file_into_chunks function
    let chunk_filenames = match split_file_into_chunks(file_name.as_str(), chunk_size) {
        Ok(chunk_filenames) => chunk_filenames,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(json!({"result": "File not found"})),
                ));
            }
            return Err(uh_oh());
        }
    };
    //Define reqwest client to send requests with
    let client = Client::new();

    for chunk_filename in chunk_filenames.clone().iter() {
        //Make sure the chunk file doesn't already exist
        let exists = match upload_db
            .chunk_filename_exist(chunk_filename.as_str())
            .await
        {
            Ok(exists) => exists,
            Err(_) => return Err(uh_oh()),
        };
        // Make sure that the file doesn't already exist, and that it isn't a file, that is under 25MB
        if exists.0 && exists.1 > 1 {
            println!("Chunk file {} already exists", chunk_filename);
            std::fs::remove_file(format!(
                "{}/uploads/chunks/{}",
                current_dir().unwrap().display(),
                chunk_filename
            ))
            .unwrap();
            continue;
        } else if exists.0 && exists.1 == 1 {
            std::fs::remove_file(format!(
                "{}/uploads/{}",
                current_dir().unwrap().display(),
                file_name
            ))
            .unwrap();
            continue;
        }
        //Define the path to the file that will be uploaded
        //And making sure the file isnt a file under 25MB

        let path = if chunk_filenames.len() == 1 {
            PathBuf::from(format!(
                "{}/uploads/{}",
                current_dir().unwrap().display(),
                file_name
            ))
        } else {
            PathBuf::from(format!(
                "{}/uploads/chunks/{}",
                current_dir().unwrap().display(),
                chunk_filename
            ))
        };

        let contents = if encryption.trim().to_lowercase() == "true" {
            encrypt(path.to_str().unwrap())
        } else {
            std::fs::read(path.to_str().unwrap()).unwrap()
        };
        std::fs::write(&path, contents).unwrap();
        //Open the file from the path and read the data in it
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => return Err(uh_oh()),
        };

        let mut bytes = Vec::new();
        match file.read_to_end(&mut bytes) {
            Ok(_) => (),
            Err(_) => return Err(uh_oh()),
        }
        //Send the file to discord
        println!("Uploading to discord");
        let res = match client
            .post(format!(
                "https://discord.com/api/v9/channels/{}/attachments",
                get_secret("CHANNEL_ID")
            ))
            .json(&json!({

                "files": [{
                    "filename": chunk_filename,
                    "file_size": bytes.len()
                }
                    ]
            }))
            .header("Authorization", format!("Bot {}", get_secret("BOT_TOKEN")))
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => {
                println!("Error: {}", e);
                return Err(uh_oh());
            }
        };

        let res_json = match res.json::<Value>().await {
            Ok(res_json) => res_json,
            Err(_) => return Err(uh_oh()),
        };

        let attachments = res_json["attachments"].as_array().unwrap();
        let upload_url = match attachments[0]["upload_url"].as_str() {
            Some(upload_url) => upload_url,
            None => return Err(uh_oh()),
        };
        let upload_filename = match attachments[0]["upload_filename"].as_str() {
            Some(upload_filename) => upload_filename,
            None => return Err(uh_oh()),
        };
        match client.put(upload_url).body(bytes).send().await {
            Ok(res) => res,
            Err(_) => return Err(uh_oh()),
        };

        let res = match client
            .post(format!(
                "https://discord.com/api/v9/channels/{}/messages",
                get_secret("CHANNEL_ID")
            ))
            .json(&json!({
                "content": "",
                "attachments" :[{
                    "filename": chunk_filename,
                    "id": "0",
                    "uploaded_filename": upload_filename
                }],
                "channel_id": "1230975819849924771"
            }))
            .header("Authorization", format!("Bot {}", get_secret("BOT_TOKEN")))
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => {
                println!("Error: {}", e);
                return Err(uh_oh());
            }
        };
        let res_json = match res.json::<Value>().await {
            Ok(res_json) => res_json,
            Err(_) => return Err(uh_oh()),
        };
        let attachments = res_json["attachments"].as_array().unwrap();
        let url = match attachments[0]["url"].as_str() {
            Some(upload_url) => upload_url,
            None => return Err(uh_oh()),
        };

        let size = match attachments[0]["size"].as_i64() {
            Some(upload_filename) => upload_filename,
            None => return Err(uh_oh()),
        };
        //Add the url to the database, to be able to download it later
        match upload_db
            .add_url(
                url,
                &payload.file_name,
                chunk_filename,
                size.to_string().as_str(),
                &encryption.trim().to_lowercase(),
            )
            .await
        {
            Ok(_) => (),
            Err(_) => {
                return Err(uh_oh());
            }
        };
        std::fs::remove_file(path).unwrap();
    }
    //Remove the file from the uploads directory

    upload_db.close().await;
    if file_exists(&format!(
        "{}/uploads/{}",
        current_dir().unwrap().display(),
        file_name
    )) {
        std::fs::remove_file(format!(
            "{}/uploads/{}",
            current_dir().unwrap().display(),
            file_name
        ))
        .unwrap();
    }
    Ok("Successfully sent files".to_string())
}

#[derive(Debug, Deserialize)]
pub struct MessagePayload {
    pub file_name: String,
}
