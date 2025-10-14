use std::sync::Arc;
use axum::{
    routing::{post},
    Json, Router,
    http::StatusCode,

};
use axum::extract::State;
use crate::services::paste_service;
use crate::services::paste_service::PasteTextRequest;

#[derive(Clone)]
struct AppState {
    paste_service: Arc<paste_service::PasteService>
}

pub async fn build_and_run(paste_service: Arc<paste_service::PasteService>, url: &String) {
    let state = AppState{paste_service};

    let app = Router::new()
        .route("/paste/text", post(paste_text))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(url).await.unwrap(); // TODO move to settings

    log::info!("listening.on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}



async fn paste_text(
    State(state): State<AppState>,
    Json(payload): Json<PasteTextRequest>,
) -> (StatusCode, String){
    match state.paste_service.paste_text(&payload).await{
        Ok(_) => (StatusCode::OK, "OK".to_string()),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("something went wrong: {}", err)),
    }
}
