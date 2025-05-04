use crate::common::EMPTY_STR;
use crate::transfer::model::UserDo;
use crate::transfer::sqlite::dao::user::UserDO;
use std::collections::HashMap;

pub mod dao;

#[derive(Debug, Default)]
pub struct TableSeq {
    pub(crate) config_id: i64,
    pub(crate) config_history_id: i64,
    pub(crate) tenant_id: i64,
    pub(crate) user_id: i64,
}

impl TableSeq {
    pub fn next_user_id(&mut self) -> i64 {
        self.user_id += 1;
        self.user_id
    }
}

impl From<UserDo> for UserDO {
    fn from(value: UserDo) -> Self {
        Self {
            id: None,
            username: Some(value.username),
            nickname: Some(value.nickname),
            password_hash: value.password_hash,
            gmt_create: Some(value.gmt_create as i64),
            gmt_modified: Some(value.gmt_modified as i64),
            enabled: Some(value.enable.to_string()),
            roles: serde_json::to_string(&value.roles).ok(),
            extend_info: serde_json::to_string(&value.extend_info).ok(),
        }
    }
}

impl From<UserDO> for UserDo {
    fn from(value: UserDO) -> Self {
        let enable = if let Some(s) = &value.enabled {
            s.parse().unwrap()
        } else {
            false
        };

        let roles = if let Some(s) = &value.roles {
            serde_json::from_str(s).unwrap()
        } else {
            vec![]
        };

        let extend_info = if let Some(s) = &value.extend_info {
            serde_json::from_str(s).unwrap_or_default()
        } else {
            HashMap::default()
        };

        Self {
            username: value.username.unwrap_or_default(),
            password: EMPTY_STR.to_string(),
            nickname: value.nickname.unwrap_or_default(),
            gmt_create: value.gmt_create.unwrap_or_default() as u32,
            gmt_modified: value.gmt_modified.unwrap_or_default() as u32,
            enable,
            roles,
            extend_info,
            password_hash: value.password_hash,
            namespace_privilege_flags: None,
            namespace_white_list: Default::default(),
            namespace_black_list: Default::default(),
        }
    }
}
