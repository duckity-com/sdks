use std::collections::HashMap;

use anyhow::Context;
use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::time::Instant;

/// A simple example of using Duckity to get a challenge for a protection profile.
#[derive(clap::Parser)]
struct Args {
    /// Your protection profile's ID.
    #[arg(env = "DUCKITY_PROTECTION_PROFILE_ID")]
    protection_profile_id: String,
}

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let args = Args::parse();

    println!("Welcome to the Duckity-core example!");
    println!();
    println!("This example will fetch a challenge from the Duckling API and solve it.");
    println!("It may take some seconds depending on the challenge, your internet connection, and");
    println!("your device.");
    println!("Timings will be displayed at the end.");
    println!();

    if cfg!(debug_assertions) {
        println!("By the way, you're running this example in debug mode, which is considerably ");
        println!("slower than release mode.");
        println!("If you want to see the best performance, run this example in release mode by ");
        println!("running it with `cargo run --example basic --release` instead.");
        println!();
    }

    println!("----------------------------------------------");
    println!();
    println!("Fetching the challenge...");

    let fetching_start = Instant::now();

    let client = reqwest::Client::new();

    let url = format!(
        "https://quack.duckity.com/v1/challenges/{}/issue",
        args.protection_profile_id
    );
    let request = client.post(url).json(&ChallengeRequest {
        keys: HashMap::new(),
    });

    let response = request
        .send()
        .await
        .context("The request to fetch the challenge could not be made.")?;
    let response: ChallengeResponse = response
        .json()
        .await
        .context("The response of the challenge fetching request could not be deserialized.")?;
    let original = response.challenge;

    let fetching_elapsed = fetching_start.elapsed();

    let challenge_decoding_start = Instant::now();

    println!("Decoding the challenge...");

    let challenge =
        duckity_core::decode(&original).context("The fetched challenge could not be decoded.")?;

    let challenge_decoding_elapsed = challenge_decoding_start.elapsed();

    println!("Solving the challenge...");

    let challenge_solving_start = Instant::now();

    let solution = duckity_core::solve(&challenge);

    let challenge_solving_elapsed = challenge_solving_start.elapsed();

    let challenge_encoding_start = Instant::now();

    let solution =
        duckity_core::encode(&original, &solution).context("The solution could not be encoded.")?;

    let challenge_encoding_elapsed = challenge_encoding_start.elapsed();

    println!("Solved!");
    println!();
    println!("----------------------------------------------");
    println!();
    println!("Timings:");
    println!("Fetching the challenge took: {:?}", fetching_elapsed);
    println!(
        "Decoding the challenge took: {:?}",
        challenge_decoding_elapsed
    );
    println!(
        "Solving the challenge took: {:?}",
        challenge_solving_elapsed
    );
    println!(
        "Encoding the solution took: {:?}",
        challenge_encoding_elapsed
    );
    println!();
    println!("----------------------------------------------");
    println!();
    println!("The solution is:");
    println!("{}", solution);

    Ok(())
}
