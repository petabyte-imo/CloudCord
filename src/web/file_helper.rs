use std::{env::current_dir, fs::File, io::Write};

use axum::Extension;

use crate::{
    web::encryption_helper::{decrypt_bytes, string_to_bytes},
    States,
};

use super::{db::upload::UploadDatabase, filefunctions::reassemble_file_from_chunks};

// Gets file from the database
pub async fn get_file_from_db(
    file_name: &str,
    state: &Extension<States>,
) -> Result<String, String> {
    let decrypt = |bytes| {
        let key = string_to_bytes(state.key.lock().unwrap().as_str());
        let nonce = [0u8; 12];
        decrypt_bytes(&key, &nonce, bytes)
    };
    //Initialize the database
    let db = match UploadDatabase::new().await {
        Ok(db) => db,
        Err(_) => return Err("5321".to_string()),
    };
    //Get the urls from the database by the file name
    let files = match db.get_urls_by_filename(file_name).await {
        Ok(files) => files,
        Err(_) => return Err("5321".to_string()),
    };

    //Initialize the reqwest client
    let client = reqwest::Client::new();

    for file in files.iter() {
        //This just defines the file info to return
        //The file will be downloaded and saved to the current directory
        let res = match client.get(file.url.as_str()).send().await {
            Ok(res) => res,
            Err(_) => return Err("5321".to_string()),
        };

        let res_bytes = match res.bytes().await {
            Ok(res_json) => res_json,
            Err(_) => return Err("5321".to_string()),
        };
        let bytes = if state.encrypted.load(std::sync::atomic::Ordering::Relaxed) {
            decrypt(res_bytes.to_vec())
        } else {
            res_bytes.to_vec()
        };

        let mut file = File::create(format!(
            "{}/{}",
            current_dir().unwrap().display(),
            file.chunk_filename
        ))
        .unwrap();
        file.write_all(&bytes).unwrap();
    }

    //This will reassemble the file from chunk, only if its multiple chunks
    if files.len() != 1 {
        reassemble_file_from_chunks(file_name).unwrap();
    }
    db.close().await;

    Ok(String::from("hello"))
}
