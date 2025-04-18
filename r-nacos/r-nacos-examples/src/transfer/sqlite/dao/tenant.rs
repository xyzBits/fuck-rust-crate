use rusqlite::Connection;

pub struct TenantDao<'a> {
    conn: &'a Connection,
    inner: TenantSql,
}

impl<'a> TenantDao<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self {
            conn,
            inner: TenantSql {},
        }
    }
}
pub struct TenantSql {}
