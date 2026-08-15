use anyhow::Context;
use clap::Parser;
use tokio::time::Instant;

/// A simple example of using Duckity to get a challenge for a protection profile.
#[derive(clap::Parser)]
struct Args {
    /// Your protection profile's ID.
    #[arg(env = "DUCKITY_PROTECTION_PROFILE_ID")]
    protection_profile_id: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let args = Args::parse();

    println!("Welcome to the Duckity-rs example!");
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

    let challenge_start = Instant::now();

    let challenge = duckity::get(args.protection_profile_id)
        .await
        .context("Could not get the challenge from the duckling API.")?;

    let challenge_elapsed = challenge_start.elapsed();

    println!("Solving the challenge...");

    let solution_start = Instant::now();

    let solution = tokio::task::spawn_blocking(move || duckity::solve(&challenge))
        .await
        .context("Could not solve the fetched challenge from the duclling API.")?
        .context("Could not solve the fetched challenge from the duclling API.")?;

    let solution_elapsed = solution_start.elapsed();

    println!("Solved!");
    println!();
    println!("----------------------------------------------");
    println!();
    println!("Timings:");
    println!("Fetching the challenge took: {:?}", challenge_elapsed);
    println!("Solving the challenge took: {:?}", solution_elapsed);
    println!();
    println!("----------------------------------------------");
    println!();
    println!("The solution is:");
    println!("{}", solution);

    Ok(())
}
