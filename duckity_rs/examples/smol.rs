use async_compat::CompatExt;

const POLICY_ID: &str = "<insert-your-policy-here>";

fn main() -> anyhow::Result<()> {
    smol::block_on(async {
        let solution = duckity::solve(POLICY_ID).into_future().compat().await?;

        Ok(())
    })
}
