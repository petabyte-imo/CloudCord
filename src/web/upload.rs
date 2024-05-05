use axum::{
    extract::{
        multipart::{Field, MultipartError},
        Multipart,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde_json::Value;

use std::{
    env::current_dir,
    fs::{create_dir_all, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::{errors::uh_oh, web::send_message::send_message};

use super::send_message::MessagePayload;

// Define a struct to store uploaded file information
#[derive(Clone)]
pub struct UploadedFile {
    pub filename: String,
    pub content_type: Option<String>,
}
// Define the routes
pub fn routes() -> axum::Router {
    Router::new().route("/upload", post(upload))
}
//Function to save the uploaded file
async fn save_file(mut field: Field<'_>, path: PathBuf) -> Result<UploadedFile, MultipartError> {
    //Create the directory unless it isnt made
    create_dir_all(format!(
        "{}/uploads/chunks",
        current_dir().unwrap().display()
    ))
    .unwrap();
    //Define uploaded file info
    let filename = field.file_name().unwrap_or("unknown.bin").to_string();
    let content_type = field.content_type().map(|ct| ct.to_string());
    //Open file
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path.join(filename.clone()))
        .unwrap();
    //Define BufWriter
    let mut writer = BufWriter::new(file);

    //Write data to file using BufWriter
    while let Some(chunk) = field.chunk().await.unwrap() {
        writer.write_all(&chunk).unwrap();
    }
    //Flush writer
    writer.flush().unwrap();
    //Return uploaded file info
    Ok(UploadedFile {
        filename,
        content_type,
    })
}

async fn upload(mut multipart: Multipart) -> Result<impl IntoResponse, impl IntoResponse> {
    //Handler print
    println!("->> {:12} - upload", "HANDLER");
    //Define files vector to store all files
    let mut files = Vec::new();
    //Make sure there is a file in the request
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
        let upload_path = PathBuf::from(format!("{}/uploads", current_dir().unwrap().display())); // Customize upload directory

        //Define the file name
        let filename = match field.file_name() {
            Some(filename) => filename,
            None => {
                return Err(uh_oh());
            }
        }
        .to_string();
        //Save the uploaded file, using the save_file function, defined earlier
        let uploaded_file = save_file(field, upload_path).await.unwrap();
        //Define message payload to send to the send_message function that sends the files to discord
        let payload = MessagePayload {
            file_name: filename,
        };

        //Add the uploaded file to the files vector
        files.push(uploaded_file.clone());

        // region:  ---SendMessage
        match send_message(payload).await {
            Ok(o) => {
                println!("{o}");
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
