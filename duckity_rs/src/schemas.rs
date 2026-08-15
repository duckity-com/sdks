use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChallengeRequest {
    /// CCTC key-value pairs.
    pub keys: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct ChallengeResponse {
    /// The encoded challenge string.
    pub challenge: String,
}
