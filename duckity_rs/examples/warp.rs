use std::convert::Infallible;
use std::net::SocketAddr;

use serde::Deserialize;
use warp::{Filter, Reply};

// In an actual application, make these two configurable. `clap` is a good tool for that.
const APPLICATION_SECRET: &str = "<your-application-secret>";
const PROTECTION_PROFILE_ID: &str = "<your-protection-profile-id>";

#[tokio::main]
async fn main() {
    let protected = warp::path("protected")
        .and(warp::post())
        .and(warp::addr::remote())
        .and(warp::body::json::<ProtectedRequestPayload>())
        .and_then(handler);

    warp::serve(protected)
        .run(([0, 0, 0, 0], 8000))
        .await;
}

#[derive(Deserialize)]
struct ProtectedRequestPayload {
    solution: String,
}

async fn handler(
    // If behind a reverse proxy, use X-Forwarded-For instead. Make sure it cannot be spoofed.
    addr: Option<SocketAddr>,
    payload: ProtectedRequestPayload,
) -> Result<impl Reply, Infallible> {
    let addr = addr.expect("remote address unavailable");

    let is_valid = duckity::validate(
        payload.solution,
        addr.ip(),
        APPLICATION_SECRET,
        PROTECTION_PROFILE_ID,
    )
    .await
    .unwrap();

    if is_valid {
        Ok(warp::reply::with_status(
            warp::reply::json(&"This is protected!"),
            warp::http::StatusCode::OK,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&"The provided solution token was invalid."),
            warp::http::StatusCode::BAD_REQUEST,
        ))
    }
}
