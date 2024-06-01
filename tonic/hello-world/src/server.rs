use tonic::{Request, Response, Status};
use tonic::transport::Server;
use hello_world::greeter_server::{Greeter, GreeterServer};
use crate::hello_world::{HelloRequest, HelloResponse};

pub mod hello_world {
    tonic::include_proto!("hello_world");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let response = hello_world::HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(response))
    }

}

#[tokio::main]
async fn  main() -> Result<(), Box<dyn  std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
