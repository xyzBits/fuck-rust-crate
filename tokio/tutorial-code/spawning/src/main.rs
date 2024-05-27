use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the ip and port of the new connection
        let (socket, _) = listener.accept().await.unwrap();

        // A new task is spawned for each inbound socket.
        // The socket is moved to the new task and processed there.

        tokio::spawn(async move {
            process(socket).await;
        });
    }

    println!("Hello, world!");
}

async fn process(socket: TcpStream) {

}