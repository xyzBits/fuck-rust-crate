use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        let (mut reader, mut writer) = tokio::io::split(socket);

        if tokio::io::copy(&mut reader, &mut writer).await.is_err() {
            eprintln!("failed to copy");
        }
    }

    Ok(())
}
