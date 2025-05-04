use crate::common::pb::transfer::TransferHeader;
use crate::common::protobuf_utils::FileMessageReader;
use crate::transfer::model::{TransferHeaderDto, TransferRecordRef};
use quick_protobuf::BytesReader;
use tokio::fs::OpenOptions;

pub(crate) fn reader_transfer_record<'a>(
    v: &'a [u8],
    header: &'a TransferHeaderDto,
) -> anyhow::Result<TransferRecordRef<'a>> {
    todo!()
}

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
        } else {
            return Err(anyhow::anyhow!("read header error from transfer file"));
        };

        Ok(Self {
            message_reader,
            header,
        })
    }

    pub async fn read_record_vec(&mut self) -> anyhow::Result<Option<Vec<u8>>> {
        todo!()
    }
}
