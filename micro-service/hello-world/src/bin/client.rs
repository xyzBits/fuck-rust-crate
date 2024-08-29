// 为什么这里是 hello_world 因为这个lib 的名字就是 hello_world
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

// client 也运行在异步的运行时上
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("response={:?}", response);
    Ok(())
}
