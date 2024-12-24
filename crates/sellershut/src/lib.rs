use anyhow::Result;
use hut::Hut;

pub mod hut;

pub async fn run() -> Result<()> {
    let hut = Hut::new().await?;

    Ok(())
}
