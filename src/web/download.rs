use std::{fs, path::PathBuf, str::FromStr};

use axum::{
    body::Body,
    extract::{Path, Query},
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
};

use super::file_helper::get_file_from_db;

#[derive(Debug, serde::Deserialize)]
pub struct DownloadOptions {
    pub filename: Option<String>,
}
pub fn routes() -> axum::Router {
    axum::Router::new().route("/api/download/:filename", get(download))
}
pub async fn download(
    Path(file_path): Path<String>,
    query: Query<DownloadOptions>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let file_path = PathBuf::from(file_path.clone()); // Convert to PathBuf
    let result = match get_file_from_db(file_path.to_str().unwrap()).await {
        Ok(result) => result,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    if result == "5321" {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let file_content = fs::read(file_path.clone())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();
    let content_type = file_format::FileFormat::from_bytes(file_content.clone()); // Default to application/octet-stream

    let filename = query
        .filename
        .clone()
        .unwrap_or_else(|| file_path.file_name().unwrap().to_str().unwrap().to_string());
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_str("Content-Type").unwrap(),
        HeaderValue::from_str(content_type.to_string().as_str()).unwrap(),
    );
    headers.insert(
        HeaderName::from_str("Content-Length").unwrap(),
        HeaderValue::from(file_content.len()),
    );
    headers.insert(
        HeaderName::from_str("Accept-Ranges").unwrap(),
        HeaderValue::from_str("bytes, bytes").unwrap(),
    );
    headers.insert(
        HeaderName::from_str("Content-Disposition").unwrap(),
        HeaderValue::from_str(format!("attachment; filename=\"{}\"", filename).as_str()).unwrap(),
    );

    let nhmm_body = Body::into_data_stream(file_content.into());
    let new_body = Body::from_stream(nhmm_body);
    let response = (headers, new_body);
    std::fs::remove_file(filename).unwrap();

    Ok(Ok::<(HeaderMap, Body), ()>(response))
}
