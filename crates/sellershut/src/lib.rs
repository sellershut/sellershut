use anyhow::Result;
use hut::Hut;
use sellershut_core::google::protobuf::Empty;
use tonic::IntoRequest;

pub mod hut;

pub async fn run() -> Result<()> {
    let mut hut = Hut::new().await?;
    let empty = Empty::default().into_request();
    println!("requesting");
    let a = hut.query_users_client.query_users(empty).await?;
    println!("responded");

    Ok(())
}
