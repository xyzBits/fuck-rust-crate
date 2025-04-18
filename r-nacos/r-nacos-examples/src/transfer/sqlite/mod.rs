pub mod dao;

#[derive(Debug, Default)]
pub struct TableSeq {
    pub(crate) config_id: i64,
    pub(crate) config_history_id: i64,
    pub(crate) tenant_id: i64,
    pub(crate) user_id: i64,
}
