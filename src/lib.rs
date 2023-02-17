pub mod server;
pub mod registry;

pub mod greeter {
    tonic::include_proto!("helloworld");
}