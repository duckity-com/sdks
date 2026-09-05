use anyhow::Context;
use clap::Parser;
use tokio::time::Instant;

/// A simple example of using Duckity to get a challenge for a protection profile.
#[derive(clap::Parser)]
struct Args {
    /// Your protection profile's ID.
    #[arg(env = "DUCKITY_PROTECTION_PROFILE_ID")]
    protection_profile_id: String,

    #[arg(env = "DUCKITY_APPLICATION_SECRET")]
    application_secret: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let args = Args::parse();

    println!("Welcome to the Duckity-rs example!");
    println!();
    println!(
        "This example will fetch a challenge from the Duckling API, solve it, and validate it."
    );
    println!("It may take some seconds depending on the challenge, your internet connection, and");
    println!("your device.");
    println!("Timings will be displayed at the end.");
    println!();

    if cfg!(debug_assertions) {
        println!("By the way, you're running this example in debug mode, which is considerably ");
        println!("slower than release mode.");
        println!("If you want to see the best performance, run this example in release mode by ");
        println!("running it with `cargo run --example timings --release` instead.");
        println!();
    }

    println!("----------------------------------------------");
    println!();
    println!("Fetching the challenge...");

    let challenge_start = Instant::now();

    let challenge = duckity::solve(&args.protection_profile_id)
        .send()
        .await
        .context("Could not get the challenge from the duckling API.")?;
    let decoded =
        duckity::core::decode(&challenge).context("Could not decode the challenge string.")?;

    let challenge_elapsed = challenge_start.elapsed();

    println!("Solving the challenge...");

    let solution_start = Instant::now();
    let decoded_copy = decoded.clone();

    let solution = tokio::task::spawn_blocking(move || {
        let solution = duckity::core::solve(&decoded);
        let encoded = duckity::core::encode(&challenge, &solution)?;

        anyhow::Ok(encoded)
    })
    .await
    .context("Could not solve the fetched challenge from the duclling API.")?
    .context("Could not solve the fetched challenge from the duclling API.")?;

    let solution_elapsed = solution_start.elapsed();

    let validation_start = Instant::now();

    let is_valid = duckity::validate(
        &solution,
        decoded_copy.ip,
        &args.application_secret,
        &args.protection_profile_id,
    )
    .await
    .context("Could not validate challenge solution.")?;

    let validation_elapsed = validation_start.elapsed();

    println!("Done!");
    println!();
    println!("----------------------------------------------");
    println!();
    println!("Timings:");
    println!("Fetching the challenge took:  {:?}", challenge_elapsed);
    println!("Solving the challenge took:   {:?}", solution_elapsed);
    println!(
        "Validating the solution took: {:?}    Is Valid? {:?}",
        validation_elapsed, is_valid
    );
    println!();
    println!("----------------------------------------------");
    println!();
    println!("The solution is:");
    println!("{}", solution);

    Ok(())
}
