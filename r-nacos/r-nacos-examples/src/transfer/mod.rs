use std::borrow::Cow;

pub mod data_to_sqlite;
pub mod model;
pub mod reader;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TransferHeader<'a> {
    pub version: u64,
    pub modify_time: u64,
    pub from_sys: Cow<'a, str>,
    pub table_name_map_entries: Vec<TableNameMapEntry<'a>>,
    pub extend: Cow<'a, [u8]>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TableNameMapEntry<'a> {
    pub id: u32,
    pub name: Cow<'a, str>,
}
