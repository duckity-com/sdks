use std::net::SocketAddr;

use actix_web::web::Json;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, post};
use serde::Deserialize;

// In an actual application, make these two configurable. `clap` is a good tool for that.
const APPLICATION_SECRET: &str = "<your-application-secret>";
const POLICY_ID: &str = "<your-policy-id>";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(protected))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}

#[derive(Deserialize)]
struct ProtectedRequestPayload {
    solution: String,
}

#[post("/protected")]
async fn protected(req: HttpRequest, payload: Json<ProtectedRequestPayload>) -> impl Responder {
    // If behind a reverse proxy, use X-Forwarded-For instead.
    // Make sure it cannot be spoofed.
    let addr: SocketAddr = req.peer_addr().expect("remote address unavailable");

    let is_valid = duckity::validate(
        payload.solution.clone(),
        addr.ip(),
        APPLICATION_SECRET,
        POLICY_ID,
    )
    .await
    .unwrap();

    if is_valid {
        HttpResponse::Ok().json("This is protected!")
    } else {
        HttpResponse::BadRequest().json("The provided solution token was invalid.")
    }
}
