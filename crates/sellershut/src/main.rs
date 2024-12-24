use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    sellershut::run().await?;

    Ok(())
}
