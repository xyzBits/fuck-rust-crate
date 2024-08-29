use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloRequest, HelloResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

// 定义一个结构体，里面可以有数据库或者其他的连接
#[derive(Default)]
pub struct MyGreeter {}

// 为结构体实现 Greeter trait
#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let response = hello_world::HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
