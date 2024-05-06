use std::sync::Mutex;
use std::sync::{atomic::AtomicBool, Arc};

use axum::http::Method;
use axum::Router;
use axum::{extract::DefaultBodyLimit, Extension};
use tokio::net::TcpListener;

use tower_http::cors::{Any, CorsLayer};

type AsyncError = Box<dyn std::error::Error + Send + Sync>;

mod errors;
mod secrets;
mod web;
#[derive(Clone)]
pub struct States {
    pub encrypted: Arc<AtomicBool>,
    pub key: Arc<Mutex<String>>,
}
#[tokio::main]
async fn main() -> core::result::Result<(), AsyncError> {
    let state = States {
        encrypted: Arc::new(AtomicBool::new(false)),
        key: Arc::new(Mutex::new("".to_string())),
    };

    // Initialize cors
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    //Initialize routes and middleware
    let all_routes = Router::new()
        .merge(crate::web::upload::routes().layer(Extension(state.clone())))
        .merge(crate::web::download::routes().layer(Extension(state.clone())))
        .merge(crate::web::get_file::routes())
        .merge(crate::web::delete::routes())
        .merge(crate::web::set_encrypted::routes().layer(Extension(state.clone())))
        .layer(cors)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024));
    //Bind TCPListener
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await?;
    // region:  ---LaunchServer
    //Debug Print
    println!(
        "->> SERVER INFO - LISTENING ON {}",
        tcp_listener.local_addr().unwrap()
    );
    //Start the server
    axum::serve(tcp_listener, all_routes).await?;

    // endregion:  ---LaunchServer

    Ok(())
}
