use micro::{registry, server};
use server::Grpc;
use micro::registry::registry::Etcd;
use micro::registry::IResult;

#[tokio::main]
async fn main()->IResult<()> {
    let registry = Etcd::new("127.0.0.1:2379".to_string()).await?;
    let mut server = Grpc::new(registry).await?;
    server.start().await?;
    Ok(())
}
