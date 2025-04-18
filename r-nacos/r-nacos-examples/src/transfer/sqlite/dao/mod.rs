pub mod config;
pub mod config_history;
pub mod tenant;
pub mod user;

use crate::transfer::sqlite::dao::config::ConfigSql;
use rusqlite::Connection;

pub struct ConfigDao<'a> {
    conn: &'a Connection,
    inner: ConfigSql,
}

impl<'a> ConfigDao<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self {
            conn,
            inner: ConfigSql {},
        }
    }
}
