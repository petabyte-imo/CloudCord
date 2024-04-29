use std::{env::current_dir, fs::File, io::Write};

use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

use crate::errors::uh_oh;

use super::{db::upload::UploadDatabase, filefunctions::reassemble_file_from_chunks};

pub fn routes() -> Router {
    Router::new().route("/files", get(get_files))
}

pub async fn get_file_from_db(file_name: &str) -> Result<String, String> {
    let db = match UploadDatabase::new().await {
        Ok(db) => db,
        Err(_) => return Err("5321".to_string()),
    };

    let files = match db.get_urls_by_filename(file_name).await {
        Ok(files) => files,
        Err(_) => return Err("5321".to_string()),
    };
    let client = reqwest::Client::new();
    let mut file_info_to_return = Vec::new();
    for file in files.iter() {
        file_info_to_return.push((file.clone().file_name, file.clone().url));
        let res = match client.get(file.url.as_str()).send().await {
            Ok(res) => res,
            Err(_) => return Err("5321".to_string()),
        };
        let res_bytes = match res.bytes().await {
            Ok(res_json) => res_json,
            Err(_) => return Err("5321".to_string()),
        };
        let mut file = File::create(format!(
            "{}/{}",
            current_dir().unwrap().display(),
            file.chunk_filename
        ))
        .unwrap();
        file.write_all(&res_bytes).unwrap();
    }
    if !files.len() == 1 {
        reassemble_file_from_chunks(file_name).unwrap();
    }

    Ok(String::from("hello"))
}

async fn get_files() -> Result<impl IntoResponse, impl IntoResponse> {
    let db = match UploadDatabase::new().await {
        Ok(db) => db,
        Err(_) => return Err(uh_oh()),
    };
    let files = match db.get_urls().await {
        Ok(files) => files,
        Err(_) => return Err(uh_oh()),
    };
    let json_response = Json(json!({ "result": files}));
    Ok::<
        (axum::http::StatusCode, Json<serde_json::Value>),
        (axum::http::StatusCode, Json<serde_json::Value>),
    >((axum::http::StatusCode::OK, json_response))
}
