use quick_protobuf::BytesReader;
use tokio::fs::OpenOptions;
use crate::common::protobuf_utils::FileMessageReader;
use crate::transfer::model::TransferHeaderDto;

pub struct TransferFileReader {
    message_reader: FileMessageReader,

    // 可以在当前 crate 的所有模块中访问
    pub(crate) header: TransferHeaderDto,
}

impl TransferFileReader {
    pub async fn new(path: &str) -> anyhow::Result<Self> {
        let file = OpenOptions::new().read(true).open(path).await?;
        let mut message_reader = FileMessageReader::new(file, 8);
        message_reader.seek_start(8).await?;

        let header = if let Ok(v) = message_reader.read_next().await {
            let mut reader = BytesReader::from_bytes(&v);
            let header_do: TransferHeader = reader.read_message(&v)?;
            header_do.into()
        }
    }
}











