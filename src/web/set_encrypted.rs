use std::sync::atomic::Ordering;

use axum::{routing::post, Extension, Json};

use crate::States;
pub fn routes() -> axum::Router {
    axum::Router::new()
        .route("/set_encrypted", post(set_encrypted))
        .route("/set_encrypted_key", post(set_encrypt_key))
}

async fn set_encrypted(state: Extension<States>, Json(flag): Json<bool>) -> &'static str {
    println!("->> {:12} - set_encrypted", "HANDLER");
    state.encrypted.store(flag, Ordering::Relaxed);
    "Encryption flag set successfully"
}
async fn set_encrypt_key(state: Extension<States>, Json(flag): Json<String>) -> &'static str {
    println!("->> {:12} - set_encrypted_key", "HANDLER");
    let mut state = state.key.lock().unwrap();
    *state = flag;
    "Encryption Key set successfully"
}
