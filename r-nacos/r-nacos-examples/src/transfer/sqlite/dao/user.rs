use rusqlite::Connection;

pub struct UserDao<'a> {
    conn: &'a Connection,
    inner: UserSql,
}

impl<'a> UserDao<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        UserDao {
            conn,
            inner: UserSql {},
        }
    }
}

pub struct UserSql {}
