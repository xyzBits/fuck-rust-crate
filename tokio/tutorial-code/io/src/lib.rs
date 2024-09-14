use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn test_tokio_read_bytes() -> tokio::io::Result<()> {
    let mut file = File::open("Cargo.toml").await?;

    let mut buffer = [0; 10];

    // 读取 10 个字节
    file.read(&mut buffer[..]).await?;

    let content = String::from_utf8(buffer.to_vec())
        .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidData))?;
    println!("{}", content);

    Ok(())
}

#[tokio::test]
async fn test_tokio_read_to_end() -> tokio::io::Result<()> {
    let mut file = File::open("Cargo.toml").await?;

    let mut buffer = Vec::new();

    // 读取整个文件
    file.read_to_end(&mut buffer).await?;

    let content = String::from_utf8(buffer.to_vec())
        .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidData))?;
    println!("{}", content);

    Ok(())
}

#[tokio::test]
async fn test_tokio_write() -> tokio::io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    // 写入字节字符串的一些前缀，但不一定是全部
    let write_size = file.write(b"some bytes").await?;
    println!("{}", write_size);

    Ok(())
}

#[tokio::test]
async fn test_tokio_write_all() -> tokio::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("foo.txt")
        .await?;

    // let mut file = File::create("foo.txt").await?;

    file.write_all(b"some bitch").await?;

    Ok(())
}

#[tokio::test]
async fn test_tokio_helper_fn() -> tokio::io::Result<()> {
    let mut reader: &[u8] = b"hello";

    let mut file = File::create("foo.txt").await?;

    tokio::io::copy(&mut reader, &mut file).await?;

    Ok(())
}
