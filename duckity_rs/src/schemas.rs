use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ChallengeResponse {
    /// The encoded challenge string.
    pub challenge: String,
}

#[derive(Serialize)]
pub struct ValidateRequest {
    pub solution: String,
    pub ip: IpAddr,
}

#[derive(Deserialize)]
pub struct ValidateResponse {
    pub is_valid: bool,
}
