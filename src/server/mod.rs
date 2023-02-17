use std::collections::HashMap;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::net::{SocketAddr, UdpSocket};
use crate::registry::{IResult, Node, Service};
use super::registry::registry::{Registry};
use tonic::{transport::Server, Request, Response, Status};
use super::greeter::greeter_server::{Greeter, GreeterServer};
use super::greeter::{HelloReply, HelloRequest};

pub struct Grpc<T: Registry> {
    pub server: Server,
    pub registry: T,
}

impl<T: Registry> Grpc<T>
{
    pub async fn new(registry: T) -> IResult<Grpc<T>> {
        let server = Server::builder();

        Ok(Self { server, registry })
    }

    pub async fn start(&mut self) -> IResult<()> {
        let addr: SocketAddr = "[::]:50051".parse()?;

        let greeter = MyGreeter::default();

        let s = gen_service(addr.port(), "helloworld".to_string(), "v1.0.0".to_string());

        self.registry.register(&s).await?;

        self.server
            .add_service(GreeterServer::new(greeter))
            .serve(addr)
            .await?;

        Ok(())
    }
}

fn gen_service(port: u16, server_name: String, version: String) -> Service {
    let a = get_local_addr().unwrap();
    let register_addr = format!("{}:{}", a, port);

    let id = gen_server_id(server_name.clone());

    let mut node = Node::new(id, register_addr.to_string(), HashMap::new());
    node.metadata.insert("registry".to_string(), "etcd".to_string());
    node.metadata.insert("transport".to_string(), "grpc".to_string());
    node.metadata.insert("protocol".to_string(), "grpc".to_string());
    node.metadata.insert("server".to_string(), "grpc".to_string());

    Service::new(server_name, version, HashMap::new(), Vec::new(), vec![node])
}

fn gen_server_id(name: String) -> String {
    let mut rng = thread_rng();
    let s: String = rng.sample_iter(&Alphanumeric)
        .take(32)
        .map(|c: u8| c as char)
        .collect();

    format!("{}-{}", name, s.to_lowercase())
}


#[derive(Default, Debug)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        println!("Got a Request {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into()
        };

        Ok(Response::new(reply))
    }
}


fn get_local_addr() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None
    };

    return match socket.local_addr() {
        Ok(s) => Some(s.ip().to_string()),
        Err(_) => None
    };
}