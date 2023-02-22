use crate::greeter::greeter_server::{Greeter};
use tonic::{ Request, Response, Status};
use crate::greeter::{HelloReply, HelloRequest};

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
