use std::error::Error;
use mini_redis::client;

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    println!("Hello, world!");

    // Open a connection to the mini-redis address
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key with value
    client.set("hello", "world".into()).await?;

    // get the key hello
    let result = client.get("hello").await?;

    println!("Got value from the server; result={:?}", result);

    Ok(())
}
