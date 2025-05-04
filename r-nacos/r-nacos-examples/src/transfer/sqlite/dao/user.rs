use crate::common::rusqlite_utils::{get_row_value, sqlite_execute};
use crate::transfer::model::TransferRecordRef;
use rsql_builder::SqlBuilder;
use rusqlite::{Connection, Row, params};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserDO {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub password_hash: Option<String>,
    pub gmt_create: Option<i64>,
    pub gmt_modified: Option<i64>,
    pub enabled: Option<String>,
    pub roles: Option<String>,
    pub extend_info: Option<String>,
}

impl UserDO {
    fn from_row(r: &Row) -> Self {
        let mut user = UserDO::default();
        user.id = get_row_value(r, "id");
        user.username = get_row_value(r, "username");
        user.nickname = get_row_value(r, "nickname");
        user.password_hash = get_row_value(r, "password_hash");
        user.gmt_create = get_row_value(r, "gmt_create");
        user.gmt_modified = get_row_value(r, "gmt_modified");
        user.enabled = get_row_value(r, "enabled");
        user.roles = get_row_value(r, "roles");
        user.extend_info = get_row_value(r, "extend_info");

        user
    }
}

#[derive(Debug, Default)]
pub struct UserParam {
    pub id: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub struct UserSql {}

impl UserSql {
    fn conditions(&self, param: &UserParam) -> rsql_builder::B {
        let mut whr = rsql_builder::B::new_where();

        if let Some(id) = &param.id {
            whr.eq("id", id);
        }

        whr
    }

    pub fn query_prepare(&self, param: &UserParam) -> (String, Vec<serde_json::Value>) {
        rsql_builder::B::prepare(
            rsql_builder::B::new_sql("select id, username, nickname, password_hash, gmt_create, gmt_modified, enabled, roles, extend_info from tb_user")
                .push_build(&mut self.conditions(param))
        )
    }

    pub fn insert_prepare(&self, record: &UserDO) -> (String, Vec<serde_json::Value>) {
        let mut field_builder = rsql_builder::B::new_comma_paren();
        let mut value_builder = rsql_builder::B::new_where();

        if let Some(id) = &record.id {
            field_builder.push_sql("id");
            value_builder.push("?", id);
        }

        if let Some(username) = &record.username {
            field_builder.push_sql("username");
            value_builder.push("?", username);
        }

        if let Some(nickname) = &record.nickname {
            field_builder.push_sql("nickname");
            value_builder.push("?", nickname);
        }

        if let Some(password_hash) = &record.password_hash {
            field_builder.push_sql("password_hash");
            value_builder.push("?", password_hash);
        }

        if let Some(gmt_create) = &record.gmt_create {
            field_builder.push_sql("gmt_create");
            value_builder.push("?", gmt_create);
        }

        if let Some(gmt_modified) = &record.gmt_modified {
            field_builder.push_sql("gmt_modified");
            value_builder.push("?", gmt_modified);
        }

        if let Some(enabled) = &record.enabled {
            field_builder.push_sql("enabled");
            value_builder.push("?", enabled);
        }

        if let Some(roles) = &record.roles {
            field_builder.push_sql("roles");
            value_builder.push("?", roles);
        }

        if let Some(extend_info) = &record.extend_info {
            field_builder.push_sql("extend_info");
            value_builder.push("?", extend_info);
        }

        rsql_builder::B::prepare(
            rsql_builder::B::new_sql("insert into tb_user")
                .push_build(&mut field_builder)
                .push_sql("values")
                .push_build(&mut value_builder),
        )
    }

    pub fn update_prepare(
        &self,
        record: &UserDO,
        param: &UserParam,
    ) -> (String, Vec<serde_json::Value>) {
        let mut set_builder = rsql_builder::B::new_comma();

        if let Some(id) = &record.id {
            set_builder.eq("id", id);
        }

        if let Some(username) = &record.username {
            set_builder.eq("username", username);
        }

        if let Some(nickname) = &record.nickname {
            set_builder.eq("nickname", nickname);
        }

        if let Some(password_hash) = &record.password_hash {
            set_builder.eq("password_hash", password_hash);
        }

        if let Some(gmt_create) = &record.gmt_create {
            set_builder.eq("gmt_create", gmt_create);
        }

        if let Some(gmt_modified) = &record.gmt_modified {
            set_builder.eq("gmt_modified", gmt_modified);
        }

        if let Some(enabled) = &record.enabled {
            set_builder.eq("enabled", enabled);
        }

        if let Some(roles) = &record.roles {
            set_builder.eq("roles", roles);
        }

        if let Some(extend_info) = &record.extend_info {
            set_builder.eq("extend_info", extend_info);
        }

        let mut whr = self.conditions(param);
        if whr.is_empty() {
            panic!("update condition is empty");
        }

        rsql_builder::B::prepare(
            rsql_builder::B::new_sql("update tb_user set ")
                .push_build(&mut set_builder)
                .push_build(&mut whr),
        )
    }
}

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

    pub fn execute(&self, sql: &str, args: &[serde_json::Value]) -> anyhow::Result<usize> {
        sqlite_execute(&self.conn, sql, args)
    }

    pub fn fetch(&self, sql: &str, args: &[serde_json::Value]) -> anyhow::Result<Vec<UserDO>> {
        todo!()
    }

    pub fn fetch_count(&self, sql: &str, args: &[serde_json::Value]) -> anyhow::Result<u64> {
        todo!()
    }

    pub fn insert(&self, record: &UserDO) -> anyhow::Result<usize> {
        let (sql, args) = self.inner.insert_prepare(record);
        self.execute(&sql, &args)
    }
}
