use rusqlite::Connection;

pub struct ConfigHistoryDao<'a> {
    conn: &'a Connection,
    inner: ConfigHistorySql,
}

impl<'a> ConfigHistoryDao<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self {
            conn,
            inner: ConfigHistorySql {},
        }
    }
}

pub struct ConfigHistorySql {}
