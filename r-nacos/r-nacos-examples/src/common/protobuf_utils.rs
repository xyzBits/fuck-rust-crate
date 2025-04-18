use std::io::SeekFrom;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

#[derive(Debug, Default)]
pub struct MessagePosition {
    pub position: u64,
    pub len: u64,
}

impl MessagePosition {
    pub fn get_end_position(&self) -> u64 {
        self.position + self.len
    }
}

pub struct FileMessageReader {
    file: tokio::fs::File,
    start: u64,
}

impl FileMessageReader {
    pub fn new(file: tokio::fs::File, start: u64) -> Self {
        Self { file, start }
    }

    pub async fn seek_start(&mut self, start: u64) -> anyhow::Result<()> {
        self.file.seek(SeekFrom::Start(start)).await?;
        self.start = start;
        Ok(())
    }

    pub async fn read_next(&mut self) -> anyhow::Result<Vec<u8>> {
        let len = self.read_len().await?;
        let mut data_buf = vec![0u8; len as usize];
        let data_len = self.file.read(&mut data_buf).await?;

        if data_len < data_buf.len() {
            return Err(anyhow::anyhow!("read data not enough"));
        }

        self.start += data_len as u64;
        Ok(data_buf)
    }

    pub async fn read_by_position(&mut self, position: (u64, usize)) -> anyhow::Result<Vec<u8>> {
        todo!()
    }

    pub async fn read_to_end(&mut self) -> anyhow::Result<(u64, MessagePosition)> {
        todo!()
    }

    pub async fn read_next_position(&mut self) -> anyhow::Result<MessagePosition> {
        todo!()
    }

    pub async fn read_index_position(&mut self, index: usize) -> anyhow::Result<MessagePosition> {
        todo!()
    }

    async fn read_len(&mut self) -> anyhow::Result<u64> {
        todo!()
    }
}
