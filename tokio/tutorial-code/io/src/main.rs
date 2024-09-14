use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:6124").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;

        let (mut reader, mut writer) = tokio::io::split(socket);

        // 在后台写入数据
        let write_task = tokio::spawn(async move {
            // 这里 copy 数据

            writer.write_all(b"hello\r\n").await?;
            writer.write_all(b"world\r\n").await?;

            Ok::<_, std::io::Error>(())
        });

        let mut buf = vec![0u8; 128];

        loop {
            let byte_size = reader.read(&mut buf).await?;
            if byte_size == 0 {
                break;
            }

            println!(
                "GOT {:?}",
                String::from_utf8(buf[..byte_size].to_vec()).unwrap()
            );
        }
    }
}
