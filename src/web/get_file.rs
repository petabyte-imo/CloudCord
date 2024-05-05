use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json::json;

use crate::errors::uh_oh;

use super::db::upload::UploadDatabase;
pub fn routes() -> Router {
    Router::new().route("/files", get(get_files))
}
// Gets all files from the database to display on the frontend
async fn get_files() -> Result<impl IntoResponse, impl IntoResponse> {
    //Initialize the database
    let db = match UploadDatabase::new().await {
        Ok(db) => db,
        Err(_) => return Err(uh_oh()),
    };
    //Get the urls from the database
    let filenames = match db.get_names().await {
        Ok(files) => files,
        Err(_) => return Err(uh_oh()),
    };
    let fileencryptions = match db.get_encrypted().await {
        Ok(fileencryptions) => fileencryptions,
        Err(_) => return Err(uh_oh()),
    };
    db.close().await;
    //Create the json response
    let json_response = Json(json!({ "names": filenames, "encryptions": fileencryptions }));
    Ok::<
        (axum::http::StatusCode, Json<serde_json::Value>),
        (axum::http::StatusCode, Json<serde_json::Value>),
    >((axum::http::StatusCode::OK, json_response))
}
