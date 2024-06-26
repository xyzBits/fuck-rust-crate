use std::collections::HashMap;

use mini_redis::{Command, Connection, Frame};
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

}

async fn process(socket: TcpStream) {


    // data is not shared across the network, if we launch more than one client,
    // first set hell, word using first client, and second get hello using second client,
    // the second client will get (nil)
    // A hashmap is used to share data
    let mut db = HashMap::new();

    // Connection, provided by mini-redis, handles parsing frames from the socket
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}