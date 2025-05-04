use crate::common::string_utils::StringUtils;

pub mod constant;
pub mod pb;
pub mod protobuf_utils;
pub mod rusqlite_utils;
pub mod string_utils;

use std::sync::Arc;

pub const EMPTY_STR: &str = "";
const DEFAULT_DB_PATH: &str = "nacos_db";

#[derive(Default, Clone, Debug)]
pub struct AppSysConfig {
    pub config_db_file: String,
    pub local_db_dir: String,
    pub config_max_content: usize,
    pub http_port: u16,
    pub http_console_port: u16,
    pub enable_no_auth_console: bool,
    pub http_workers: Option<usize>,
    pub grpc_port: u16,
    pub raft_node_id: u64,
    pub raft_node_addr: String,
    pub raft_auto_init: bool,
    pub raft_join_addr: String,
    pub raft_snapshot_log_size: u64,
    pub console_login_timeout: i32,
    pub console_login_one_hour_limit: u32,
    pub gmt_fixed_offset_hours: Option<i32>,
    pub openapi_login_timeout: i32,
    pub openapi_login_one_minute_limit: u32,
    pub openapi_enable_auth: bool,
    pub cluster_token: Arc<String>,
    pub backup_token: Arc<String>,
    pub init_admin_username: String,
    pub init_admin_password: String,
    pub metrics_enable: bool,
    pub metrics_collect_interval_second: u64,
    pub metrics_log_interval_second: u64,
    pub metrics_log_enable: bool,
    pub console_captcha_enable: bool,
    pub run_in_docker: bool,
    pub naming_health_timeout: u64,
    pub naming_instance_timeout: u64,
}

impl AppSysConfig {
    pub fn init_from_env() -> Self {
        //println!("init_from_env");
        let config_db_file =
            std::env::var("RNACOS_CONFIG_DB_FILE").unwrap_or("config.db".to_owned());
        let config_max_content = std::env::var("RNACOS_CONFIG_MAX_CONTENT")
            .unwrap_or("10485760".to_owned())
            .parse()
            .unwrap_or(10 * 1024 * 1024);
        let http_port = std::env::var("RNACOS_HTTP_PORT")
            .unwrap_or("8848".to_owned())
            .parse()
            .unwrap_or(8848);
        let http_workers = std::env::var("RNACOS_HTTP_WORKERS")
            .unwrap_or("".to_owned())
            .parse()
            .ok();
        let grpc_port = std::env::var("RNACOS_GRPC_PORT")
            .unwrap_or("".to_owned())
            .parse()
            .unwrap_or(http_port + 1000);
        let http_console_port = std::env::var("RNACOS_HTTP_CONSOLE_PORT")
            .unwrap_or("".to_owned())
            .parse()
            .unwrap_or(http_port + 2000);
        let run_in_docker = std::env::var("RNACOS_RUN_IN_DOCKER")
            .unwrap_or("".to_owned())
            .eq_ignore_ascii_case("true");
        let local_db_dir = Self::get_data_dir(run_in_docker);
        let raft_node_id = std::env::var("RNACOS_RAFT_NODE_ID")
            .unwrap_or("1".to_owned())
            .parse()
            .unwrap_or(1);
        let raft_node_addr =
            std::env::var("RNACOS_RAFT_NODE_ADDR").unwrap_or(format!("127.0.0.1:{}", &grpc_port));
        let raft_auto_init = std::env::var("RNACOS_RAFT_AUTO_INIT")
            .unwrap_or("".to_owned())
            .parse()
            .unwrap_or(raft_node_id == 1);
        let raft_join_addr = std::env::var("RNACOS_RAFT_JOIN_ADDR").unwrap_or_default();
        let console_login_timeout = std::env::var("RNACOS_CONSOLE_LOGIN_TIMEOUT")
            .unwrap_or("86400".to_owned())
            .parse()
            .unwrap_or(86400);
        let console_login_one_hour_limit = std::env::var("RNACOS_CONSOLE_LOGIN_ONE_HOUR_LIMIT")
            .unwrap_or("5".to_owned())
            .parse()
            .unwrap_or(5);
        let openapi_login_timeout = std::env::var("RNACOS_API_LOGIN_TIMEOUT")
            .unwrap_or("3600".to_owned())
            .parse()
            .unwrap_or(3600);
        let openapi_login_one_minute_limit = std::env::var("RNACOS_API_LOGIN_ONE_MINUTE_LIMIT")
            .unwrap_or("100".to_owned())
            .parse()
            .unwrap_or(100);
        let raft_snapshot_log_size = std::env::var("RNACOS_RAFT_SNAPSHOT_LOG_SIZE")
            .unwrap_or("10000".to_owned())
            .parse()
            .unwrap_or(10000);
        let enable_no_auth_console = std::env::var("RNACOS_ENABLE_NO_AUTH_CONSOLE")
            .unwrap_or("false".to_owned())
            .parse()
            .unwrap_or(false);
        let gmt_fixed_offset_hours = std::env::var("RNACOS_GMT_OFFSET_HOURS")
            .unwrap_or_default()
            .parse()
            .ok();
        let openapi_enable_auth = std::env::var("RNACOS_ENABLE_OPEN_API_AUTH")
            .unwrap_or("false".to_owned())
            .parse()
            .unwrap_or(false);
        let cluster_token = std::env::var("RNACOS_CLUSTER_TOKEN")
            .map(Arc::new)
            .unwrap_or(constant::EMPTY_ARC_STRING.clone());
        let mut backup_token = std::env::var("RNACOS_BACKUP_TOKEN")
            .map(Arc::new)
            .unwrap_or(constant::EMPTY_ARC_STRING.clone());
        if backup_token.len() < 32 {
            backup_token = constant::EMPTY_ARC_STRING.clone();
        }
        let init_admin_username =
            StringUtils::map_not_empty(std::env::var("RNACOS_INIT_ADMIN_USERNAME").ok())
                .unwrap_or("admin".to_owned());
        let init_admin_password =
            StringUtils::map_not_empty(std::env::var("RNACOS_INIT_ADMIN_PASSWORD").ok())
                .unwrap_or("admin".to_owned());
        let metrics_enable = std::env::var("RNACOS_ENABLE_METRICS")
            .unwrap_or("true".to_owned())
            .parse()
            .unwrap_or(true);
        let mut metrics_collect_interval_second =
            std::env::var("RNACOS_METRICS_COLLECT_INTERVAL_SECOND")
                .unwrap_or("15".to_owned())
                .parse()
                .unwrap_or(15);
        let console_captcha_enable = std::env::var("RNACOS_CONSOLE_ENABLE_CAPTCHA")
            .unwrap_or("true".to_owned())
            .parse()
            .unwrap_or(true);
        if metrics_collect_interval_second < 1 {
            metrics_collect_interval_second = 1;
        }
        let metrics_log_enable = std::env::var("RNACOS_METRICS_ENABLE_LOG")
            .unwrap_or("false".to_owned())
            .parse()
            .unwrap_or(false);
        let mut metrics_log_interval_second = std::env::var("RNACOS_METRICS_LOG_INTERVAL_SECOND")
            .unwrap_or("60".to_owned())
            .parse()
            .unwrap_or(60);
        if metrics_log_interval_second < 5 {
            metrics_log_interval_second = 5;
        }
        if metrics_log_interval_second < metrics_collect_interval_second {
            metrics_collect_interval_second = metrics_log_interval_second;
        }
        let naming_health_timeout = std::env::var("RNACOS_NAMING_HEALTH_TIMEOUT_SECOND")
            .unwrap_or("15".to_owned())
            .parse()
            .unwrap_or(15)
            * 1000;
        let mut naming_instance_timeout = std::env::var("RNACOS_NAMING_INSTANCE_TIMEOUT_SECOND")
            .unwrap_or("30".to_owned())
            .parse()
            .unwrap_or(30)
            * 1000;
        if naming_health_timeout >= naming_instance_timeout {
            //如果配置不合理，则默认使过期时间大于心跳时间15秒
            naming_instance_timeout = naming_health_timeout + 15 * 1000;
        }
        Self {
            local_db_dir,
            config_db_file,
            config_max_content,
            http_port,
            http_console_port,
            enable_no_auth_console,
            grpc_port,
            http_workers,
            raft_node_id,
            raft_node_addr,
            raft_auto_init,
            raft_join_addr,
            raft_snapshot_log_size,
            console_login_timeout,
            console_login_one_hour_limit,
            openapi_login_timeout,
            openapi_login_one_minute_limit,
            gmt_fixed_offset_hours,
            openapi_enable_auth,
            cluster_token,
            backup_token,
            init_admin_username,
            init_admin_password,
            metrics_enable,
            metrics_log_enable,
            metrics_collect_interval_second,
            metrics_log_interval_second,
            console_captcha_enable,
            run_in_docker,
            naming_health_timeout,
            naming_instance_timeout,
        }
    }

    fn get_data_dir(run_in_docker: bool) -> String {
        if let Ok(v) = std::env::var("RNACOS_DATA_DIR") {
            v
        } else if let Ok(v) = std::env::var("RNACOS_CONFIG_DB_DIR") {
            v
        } else if run_in_docker {
            // 运行在 docker 默认值一致
            DEFAULT_DB_PATH.to_owned()
        } else {
            // 条件编译，表示以下代码仅在 linux 或 macos 系统上编译执行
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            {
                if let Some(mut home) = dirs::home_dir() {
                    // 返回用户的主目录
                    home.push(".local/share/r-nacos/nacos.db"); // 在主目录后追加子路径
                    return home.to_string_lossy().to_string();
                }
            }

            // windows 系统默认值 保持一致
            DEFAULT_DB_PATH.to_owned()
        }
    }
}
