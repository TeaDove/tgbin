use std::sync::Arc;
use axum::{routing::{post}, Json, Router, http::StatusCode};
use axum::extract::State;
use axum::{
    extract::{Request},
    middleware::{self, Next},
    response::Response,
};
use crate::services::paste_service;
use crate::services::paste_service::PasteTextRequest;
use tower::{ServiceBuilder};
use tower_http::cors::{CorsLayer};

#[derive(Clone)]
struct AppState {
    paste_service: Arc<paste_service::PasteService>
}

pub async fn build_and_run(paste_service: Arc<paste_service::PasteService>, url: &String) {
    let state = AppState{paste_service};

    let app = Router::new()
        .route("/api/paste/text", post(paste_text))
        .route("/api", post(paste_text_default))
        .layer(CorsLayer::permissive())
        .layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(logging_middleware)))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(url).await.unwrap();

    log::info!("listening.on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn paste_text_default(
    State(state): State<AppState>,
    body: String,
) -> Result<& 'static str, StatusCode> {
    let payload = PasteTextRequest{text: body, user: "@TeaDove".to_string(), with_code: true};

    match state.paste_service.paste_text(&payload).await {
        Ok(_) => Ok("OK"),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn paste_text(
    State(state): State<AppState>,
    Json(payload): Json<PasteTextRequest>,
) -> Result<& 'static str, StatusCode> {
    match state.paste_service.paste_text(&payload).await {
        Ok(_) => Ok("OK"),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


async fn logging_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Log something before the request is processed by the handler
    let method = request.method().clone();
    let uri = request.uri().clone();

    let response = next.run(request).await;

    log::info!("request.processed {} {} {}", method, uri, response.status());

    Ok(response)
}