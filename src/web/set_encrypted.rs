use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use axum::{routing::post, Extension, Json};
pub fn routes() -> axum::Router {
    axum::Router::new().route("/set_encrypted", post(set_encrypted))
}

async fn set_encrypted(state: Extension<Arc<AtomicBool>>, Json(flag): Json<bool>) -> &'static str {
    println!("->> {:12} - set_encrypted", "HANDLER");
    state.store(flag, Ordering::Relaxed);
    "Encryption flag set successfully"
}
