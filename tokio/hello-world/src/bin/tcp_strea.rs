use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    println!("Starting TCP Stream example....");

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server listening on port 8080");

    let (socket, addr) = listener.accept().await?;
    println!("New accepted TCP connection from {:?}", addr);

    let reader = BufReader::new(socket);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        println!("Received msg: {}", line);
    }

    Ok(())
}
