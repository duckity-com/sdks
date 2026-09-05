use std::net::SocketAddr;

use axum::extract::ConnectInfo;
use axum::response::IntoResponse;
use axum::{Json, Router, routing};
use reqwest::StatusCode;
use serde::Deserialize;
use tokio::net::TcpListener;

// In an actual application, make these two configurable. `clap` is a good tool for that.
const APPLICATION_SECRET: &str = "<your-application-secret>";
const PROTECTION_PROFILE_ID: &str = "<your-protection-profile-id>";

#[tokio::main]
async fn main() {
    let router = Router::new().route("/protected", routing::post(handler));

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

#[derive(Deserialize)]
struct ProtectedRequestPayload {
    solution: String,
}

async fn handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<ProtectedRequestPayload>,
) -> impl IntoResponse {
    let is_valid = duckity::validate(
        payload.solution,
        addr.ip(),
        APPLICATION_SECRET,
        PROTECTION_PROFILE_ID,
    )
    .await
    .unwrap();

    if is_valid {
        (StatusCode::OK, Json("This is protected!"))
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json("The provided solution token was invalid."),
        )
    }
}
