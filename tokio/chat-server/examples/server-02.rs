use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// 使用 anyhow 模块进行优雅的错误传播，在任何想要返回 Result<T, Box<dyn std::err:Error>> 的地方，可以使用 anyhow::Result<T> 替代
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = TcpListener::bind("127.0.0.1:42069").await?;
    loop {
        let (mut tcp, _) = server.accept().await?;


        let mut buffer = [0u8; 16];
        loop {
            let n = tcp.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            let _ = tcp.write(&buffer[..n]).await?;
        }
    }


}