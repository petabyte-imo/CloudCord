use crate::{errors::uh_oh, secrets::get_secret, web::db::upload::UploadDatabase};
use std::{env::current_dir, fs::File, io::Read, path::PathBuf};

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::web::filefunctions::*;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};

pub async fn send_message(
    payload: MessagePayload,
) -> core::result::Result<String, impl IntoResponse> {
    println!("->>  Starting to Send Messages To Discord",);
    let upload_db = match UploadDatabase::new().await {
        Ok(db) => db,
        Err(_) => return Err(uh_oh()),
    };
    let file_name = &payload.file_name;
    let chunk_size = 25 * 1024 * 1024; // 10 MB

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
    let client = Client::new();

    for chunk_filename in chunk_filenames.clone().iter() {
        let exists = match upload_db
            .chunk_filename_exist(chunk_filename.as_str())
            .await
        {
            Ok(exists) => exists,
            Err(_) => return Err(uh_oh()),
        };

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
                chunk_filename
            ))
            .unwrap();
            continue;
        }
        let mut file: File;
        let path;
        if chunk_filenames.len() == 1 {
            path = PathBuf::from(format!(
                "{}/uploads/{}",
                current_dir().unwrap().display(),
                file_name
            ));
        } else {
            path = PathBuf::from(format!(
                "{}/uploads/chunks/{}",
                current_dir().unwrap().display(),
                chunk_filename
            ));
        }
        file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => return Err(uh_oh()),
        };

        let mut bytes = Vec::new();
        match file.read_to_end(&mut bytes) {
            Ok(_) => (),
            Err(_) => return Err(uh_oh()),
        }

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
        match upload_db
            .add_url(
                url,
                &payload.file_name,
                chunk_filename,
                size.to_string().as_str(),
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

    Ok("Successfully sent files".to_string())
}

#[derive(Debug, Deserialize)]
pub struct MessagePayload {
    pub file_name: String,
}
