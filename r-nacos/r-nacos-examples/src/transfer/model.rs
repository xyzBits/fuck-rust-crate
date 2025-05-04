use crate::common::pb::transfer::TransferHeader;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, prost::Message, Serialize, Deserialize)]
pub struct UserDo {
    #[prost(string, tag = "1")]
    pub username: String,

    #[prost(string, tag = "2")]
    pub password: String,

    #[prost(string, tag = "3")]
    pub nickname: String,

    #[prost(uint32, tag = "4")]
    pub gmt_create: u32,

    #[prost(uint32, tag = "5")]
    pub gmt_modified: u32,

    #[prost(bool, tag = "6")]
    pub enable: bool,

    #[prost(string, repeated, tag = "7")]
    pub roles: ::prost::alloc::vec::Vec<String>,

    #[prost(map = "string, string", tag = "8")]
    pub extend_info: ::std::collections::HashMap<String, String>,

    #[prost(string, optional, tag = "9")]
    pub password_hash: Option<String>,

    #[prost(uint32, optional, tag = "10")]
    pub namespace_privilege_flags: Option<u32>,

    #[prost(string, repeated, tag = "11")]
    pub namespace_white_list: Vec<String>,

    #[prost(string, repeated, tag = "12")]
    pub namespace_black_list: Vec<String>,
}

impl UserDo {
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    pub fn from_bytes(v: &[u8]) -> anyhow::Result<Self> {
        Ok(prost::Message::decode(v)?)
    }
}

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
