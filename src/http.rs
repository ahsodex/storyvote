use crate::state::SharedState;
use crate::ui;
use crate::ws;
use axum::extract::{Query, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Json, Router};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

#[derive(Debug, Deserialize)]
pub struct JoinQuery {
    pub name: String,
}

pub fn router(shared: Arc<SharedState>, http_access_log: bool) -> Router {
    let router = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/api/info", get(info))
        .route("/ws", get(ws_route))
        .with_state(shared);

    if http_access_log {
        router.layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
    } else {
        router
    }
}

async fn index() -> Html<&'static str> {
    Html(ui::index_html())
}

async fn health() -> &'static str {
    "ok"
}

async fn info() -> Json<serde_json::Value> {
    Json(json!({
        "name": "storyvote",
        "sessionModel": "single",
        "persistence": false
    }))
}

async fn ws_route(
    ws: WebSocketUpgrade,
    State(shared): State<Arc<SharedState>>,
    Query(join): Query<JoinQuery>,
) -> impl IntoResponse {
    if join.name.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "name query parameter is required").into_response();
    }

    ws.on_upgrade(move |socket| ws::handle_socket(socket, shared, join.name))
}
