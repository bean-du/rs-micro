use micro::{server};
use server::grpc::Grpc;
use micro::registry::registry::Etcd;
use micro::registry::IResult;

#[tokio::main]
async fn main() -> IResult<()> {
    tokio::spawn(async move {
        let registry = match Etcd::new("127.0.0.1:2379".to_string()).await {
            Ok(etcd) => etcd,
            Err(e) => {
                println!("registry init error: {}", e);
                return;
            }
        };

        let mut server = match Grpc::new(registry, "helloworld".to_string()).await {
            Ok(s) => s,
            Err(e) => {
                println!("server init error: {}", e);
                return;
            }
        };

        match server.start().await {
            Ok(_) => println!("grpc start success"),
            Err(e) => {
                println!("grpc server start error: {}", e);
                return;
            }
        };
    });

    let tcp_server = server::tcp::TcpServer::new("0.0.0.0:8855".into());
    tcp_server.start().await?;

    Ok(())
}
