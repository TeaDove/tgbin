use std::sync::Arc;
use axum::{
    routing::{post},
    Json, Router,
    http::StatusCode,

};
use axum::extract::State;
use serde::{Deserialize, Serialize};
use crate::services::paste_service;

pub async fn build_and_run(paste_service: Arc<paste_service::PasteService>) {
    let app = Router::new()
        .route("/paste/text", post(paste_text))
        .with_state(paste_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap(); // TODO move to settings

    log::info!("listening.on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


#[derive(Deserialize, Serialize)]
struct PasteTextRequest {
    text: String,
    user: String,
}

async fn paste_text(
    Json(payload): Json<PasteTextRequest>,
    State(paste_service): State<Arc<paste_service::PasteService>>,
) -> (StatusCode, String){
    match paste_service.paste_text(payload.text, &payload.user).await{
        Ok(_) => (StatusCode::OK, "OK".to_string()),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("something went wrong: {}", err)),
    }
}
