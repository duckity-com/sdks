use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChallengeResponse {
    /// The encoded challenge string.
    pub challenge: String,
}
