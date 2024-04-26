use axum::{
    body::Body,
    extract::{
        multipart::{Field, MultipartError},
        Multipart, Path,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use serde_json::json;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Read, Write},
    path::PathBuf,
};

use crate::{errors::uh_oh, web::send_message::send_message};

use super::send_message::MessagePayload;

// Define a struct to store uploaded file information
#[derive(Debug, Clone)]
struct UploadedFile {
    filename: String,
    content_type: Option<String>,
}

pub fn routes() -> axum::Router {
    Router::new()
        .route("/upload", post(upload))
        .route("/uploads/:dir", get(get_uploads))
        .route("/download/:filename", get(download_file))
}
async fn save_file(mut field: Field<'_>, path: PathBuf) -> Result<UploadedFile, MultipartError> {
    let filename = field.file_name().unwrap_or("unknown.bin").to_string();
    let content_type = field.content_type().map(|ct| ct.to_string());

    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path.join(filename.clone()))
        .unwrap();
    let mut writer = BufWriter::new(file);

    while let Some(chunk) = field.chunk().await.unwrap() {
        writer.write_all(&chunk).unwrap();
    }

    writer.flush().unwrap();

    Ok(UploadedFile {
        filename,
        content_type,
    })
}

async fn upload(mut multipart: Multipart) -> Result<impl IntoResponse, impl IntoResponse> {
    println!("->> {:12} - upload", "HANDLER");
    let mut files = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                format!("Error processing field: {}", err),
            )
        })
        .unwrap()
    {
        // Validate filename or content type here if needed

        let upload_path = PathBuf::from("uploads"); // Customize upload directory
        let filename = match field.file_name() {
            Some(filename) => filename,
            None => {
                return Err(uh_oh());
            }
        }
        .to_string();
        let uploaded_file = save_file(field, upload_path).await.unwrap();
        let payload = MessagePayload {
            file_name: filename,
        };
        files.push(uploaded_file.clone());

        // region:  ---SendMessage
        match send_message(payload).await {
            Ok(o) => {
                println!("upload.rs {o}");
                o
            }
            Err(_) => {
                return Err(uh_oh());
            }
        };

        // endregion:  ---SendMessage
    }

    Ok(format!(
        "Successfully uploaded {} files: {:?}",
        files.len(),
        files.iter().map(|f| f.filename.clone()).collect::<Vec<_>>()
    ))
}

async fn get_uploads() -> Result<impl IntoResponse, impl IntoResponse> {
    println!("->> {:12} - get_uploads", "HANDLER");
    let dir = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        + "/"
        + "uploads";

    let upload_path = PathBuf::from(dir);
    if !upload_path.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"result": "Directory not found"})),
        ));
    }

    let mut entries = tokio::fs::read_dir(upload_path)
        .await
        .map_err(|err| {
            eprintln!("Error reading upload directory: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
        .unwrap();

    let mut file_list = Vec::new();
    while let Ok(Some(entry)) = entries.next_entry().await {
        let file_type = entry.file_type().await.unwrap();
        if file_type.is_file() {
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap().to_string();
            file_list.push(filename);
        }
    }
    if file_list.is_empty() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"result": "No files found"})),
        ));
    }

    Ok(Json(
        json!({ "result": "File(s) Found", "files": file_list }),
    ))
}
async fn download_file(Path(filename): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let file_path = PathBuf::from("uploads/chunks").join(filename.clone()); // Customize upload directory

    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let file = OpenOptions::new()
        .read(true)
        .open(file_path.clone())
        .unwrap();
    let file_len = match file.metadata() {
        Ok(metadata) => metadata,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
    .len();

    let mut data = Vec::new();
    let mut file = OpenOptions::new()
        .read(true)
        .open(file_path.clone())
        .unwrap();

    file.read_to_end(&mut data).unwrap();
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(
            axum::http::header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .header(axum::http::header::CONTENT_LENGTH, file_len.to_string())
        .header(axum::http::header::CONTENT_TYPE, "application/octet-stream") // Generic binary data
        .body(Body::from(data))
        .unwrap();

    Ok(response)
}