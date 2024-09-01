use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloRequest, HelloResponse};
use opentelemetry::global;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::trace::{TraceError, Tracer};
use tonic::metadata::MetadataMap;
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
        mut request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let parent_cx =
        //     global::get_text_map_propagator(|prop|
        //         prop.extract(&MetadataMap {
        //             request.get_metadata_mut()
        //         }));
        //
        // global::tracer("greeter").start_with_context("Processing reply");


        println!("Got a request from {:?}", request.remote_addr());

        let response = hello_world::HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(response))
    }
}


fn tracing_init() -> Result<impl Tracer, TraceError> {
    global::set_text_map_propagator(TraceContextPropagator::new());
    opentelemetry_jaeger::new_pipeline()
        .with_service_name("grpc-server")
        .install_simple()
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_init()?;


    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        // 加服务，也就是业务逻辑的实现
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        // await 会一直阻塞在这里
        .await?;

    Ok(())
}
