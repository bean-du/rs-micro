use std::collections::HashMap;
use std::net::{TcpListener, SocketAddr};
use crate::registry::{IResult, registry, Service};
use crate::registry::registry::Registry;
use tonic::{transport::Server};
use tonic::transport::server::TcpIncoming;
use crate::greeter::greeter_server::{GreeterServer};
use crate::registry::Node;
use crate::server::gen_server_id;
use super::greeter as greeter_impl;

pub struct Grpc<T: Registry> {
    pub server: Server,
    pub registry: T,
    pub id: String,
    pub name: String,
}

impl<T: Registry> Grpc<T>
{
    pub async fn new(registry: T, name: String) -> IResult<Grpc<T>> {
        let server = Server::builder();
        let id = gen_server_id();
        Ok(Self { server, registry, id, name })
    }

    pub async fn start(&mut self) -> IResult<()> {
        let mut addr: SocketAddr = ([0, 0, 0, 0], 0).into();

        let mut port = 7722;
        let tinc = loop {
            addr = format!("0.0.0.0:{}", port).parse().unwrap();
            match TcpIncoming::new(addr, true, None) {
                Ok(t) => break t,
                Err(_) => port += 1
            }
        };

        let greeter = greeter_impl::MyGreeter::default();

        let s = self.gen_service(addr.port(), self.name.clone(), "v1.0.0".to_string());

        self.registry.register(&s).await?;

        self.server
            .add_service(GreeterServer::new(greeter))
            .serve_with_incoming(tinc)
            .await?;

        Ok(())
    }

    pub fn gen_service(&self, port: u16, server_name: String, version: String) -> Service {
        let a = super::get_local_addr().unwrap();
        let register_addr = format!("{}:{}", a, port);

        let node_id = format!("{}-{}", server_name, self.id.clone());

        let mut node = Node::new(node_id, register_addr.to_string(), HashMap::new());
        node.metadata.insert("registry".to_string(), self.registry.string());
        node.metadata.insert("transport".to_string(), "grpc".to_string());
        node.metadata.insert("protocol".to_string(), "grpc".to_string());
        node.metadata.insert("server".to_string(), "grpc".to_string());

        Service::new(server_name, version, HashMap::new(), Vec::new(), vec![node])
    }
}
