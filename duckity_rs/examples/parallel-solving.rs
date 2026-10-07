use std::io::{self, Write};

const POLICY_ID: &str = "<your-own-policy-id>";

struct Credentials {
    email: String,
    password: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let solution_task = tokio::spawn(duckity::solve(POLICY_ID).into_future());

    let credentials = tokio::task::spawn_blocking(|| {
        let mut email = String::new();
        let mut password = String::new();

        print!("Enter your email: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut email)?;

        print!("Enter your password: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut password)?;

        anyhow::Ok(Credentials { email, password })
    })
    .await??;

    let solution = solution_task.await??;

    Ok(())
}
