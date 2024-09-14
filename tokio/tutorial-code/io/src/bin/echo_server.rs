use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // 返回 Ok(0) 值标志远程连接已经关闭
                    Ok(0) => return,

                    Ok(n) => {
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // 未知的 socket error，停止处理
                            eprintln!("write socket error");
                            return;
                        }
                    }
                    Err(_) => {
                        eprintln!("failed to read from socket");
                        return;
                    }
                }
            }
        });
    }

    Ok(())
}
