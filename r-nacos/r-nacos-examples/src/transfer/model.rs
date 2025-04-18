use crate::transfer::TransferHeader;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

pub struct TransferHeaderDto {
    pub version: u64,
    pub modify_time: u64,
    pub from_sys: Option<String>,
    pub name_to_id: HashMap<Arc<String>, u32>,
    pub id_to_name: HashMap<u32, Arc<String>>,
    pub max_id: u32,
    pub extend_info: HashMap<String, String>,
}

impl<'a> From<TransferHeader<'a>> for TransferHeaderDto {
    fn from(t: TransferHeader<'a>) -> Self {
        todo!()
    }
}

pub struct TransferRecordRef<'a> {
    pub table_name: Arc<String>,
    pub key: Cow<'a, [u8]>,
    pub value: Cow<'a, [u8]>,
}
