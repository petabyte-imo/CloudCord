use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::post,
    Json,
};
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::errors::uh_oh;

use super::db::upload::UploadDatabase;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DownloadOptions {
    pub filename: Option<String>,
}
pub fn routes() -> axum::Router {
    axum::Router::new().route("/api/delete/:filename", post(delete))
}
pub async fn delete(
    Path(file_path): Path<String>,
    query: Query<DownloadOptions>,
) -> Result<(StatusCode, Json<Value>), impl IntoResponse> {
    let upload_db = match UploadDatabase::new().await {
        Ok(db) => db,
        Err(_) => return Err(uh_oh()),
    };
    match upload_db.delete_from_filename(&file_path).await {
        Ok(_) => (),
        Err(_) => return Err(uh_oh()),
    };
    Ok::<(StatusCode, axum::Json<Value>), (StatusCode, axum::Json<Value>)>((
        StatusCode::OK,
        Json(json!({"result": format!("Deleted {file_path} successfully")})),
    ))
}
