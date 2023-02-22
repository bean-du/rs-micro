pub mod server;
pub mod registry;
pub mod protocol;

pub mod greeter {
    tonic::include_proto!("helloworld");
}