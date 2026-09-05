use async_compat::CompatExt;

const PROTECTION_PROFILE_ID: &str = "<insert-your-protection-profile-here>";

fn main() -> anyhow::Result<()> {
    smol::block_on(async {
        let solution = duckity::solve(PROTECTION_PROFILE_ID).into_future().compat().await?;

        Ok(())
    })
}
