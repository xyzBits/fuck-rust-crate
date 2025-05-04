use crate::common::constant::{CONFIG_TREE_NAME, NAMESPACE_TREE_NAME, USER_TREE_NAME};
use crate::transfer::model::{TransferRecordRef, UserDo};
use crate::transfer::reader::{TransferFileReader, reader_transfer_record};
use crate::transfer::sqlite::TableSeq;
use crate::transfer::sqlite::dao::ConfigDao;
use crate::transfer::sqlite::dao::config_history::ConfigHistoryDao;
use crate::transfer::sqlite::dao::tenant::TenantDao;
use crate::transfer::sqlite::dao::user::{UserDO, UserDao};
use rusqlite::Connection;

pub async fn data_to_sqlite(data_file: &str, db_path: &str) -> anyhow::Result<()> {
    let mut file_reader = TransferFileReader::new(data_file).await?;

    let conn = self::open_init_db(db_path).await?;

    let mut config_count = 0;
    let mut tenant_count = 0;
    let mut user_count = 0;
    let mut ignore = 0;

    let mut table_seq = TableSeq::default();
    let config_dao = ConfigDao::new(&conn);
    let config_history_dao = ConfigHistoryDao::new(&conn);
    let user_dao = UserDao::new(&conn);
    let tenant_dao = TenantDao::new(&conn);

    while let Ok(Some(vec)) = file_reader.read_record_vec().await {
        let record = reader_transfer_record(&vec, &file_reader.header)?;
        if record.table_name.as_str() == CONFIG_TREE_NAME.as_str() {
            config_count += 1;
            self::insert_config(&mut table_seq, &config_dao, &config_history_dao, record)?;
        } else if record.table_name.as_str() == NAMESPACE_TREE_NAME.as_str() {
            tenant_count += 1;
            self::insert_namespace(&mut table_seq, &tenant_dao, record)?;
        } else if record.table_name.as_str() == USER_TREE_NAME.as_str() {
            user_count += 1;
            self::insert_user(&mut table_seq, &user_dao, record)?;
        } else {
            ignore += 1;
        }
    }

    log::info!(
        "transfer to sqlite db finished, config count:{}, tenant count:{}, user count:{}, ignore count:{}",
        config_count,
        tenant_count,
        user_count,
        ignore
    );

    Ok(())
}

// anyhow::Result<Connection>  = anyhow::Result<Connection, anyhow::Error>
pub async fn open_init_db(db_path: &str) -> anyhow::Result<Connection, anyhow::Error> {
    // 这里 ? 返回的是 rusqlite::Result<Connection, rusqlite::Error>, rusqlite::Error 实现了 std::error::Error
    // 使用 ? 时，rusqlite::Error 转为 anyhow::Error
    let conn = Connection::open(db_path)?;
    let create_table_sql = r"
create table if not exists tb_config(
    id integer primary key autoincrement,
    data_id text,
    group_id text,
    tenant_id text,
    content text,
    config_type text,
    config_desc text,
    last_time long
);
create index if not exists tb_config_key_idx on tb_config(data_id,group_id,tenant_id);

create table if not exists tb_config_history(
    id integer primary key autoincrement,
    data_id text,
    group_id text,
    tenant_id text,
    content text,
    config_type text,
    config_desc text,
    op_user text,
    last_time long
);
create index if not exists tb_config_history_key_idx on tb_config_history(data_id,group_id,tenant_id);

create table if not exists tb_tenant(
    id integer primary key autoincrement,
    tenant_id text,
    tenant_name text,
    tenant_desc text,
    create_flag integer
);

create table if not exists tb_user(
    id integer primary key autoincrement,
    username text,
    nickname text,
    password_hash text,
    gmt_create integer,
    gmt_modified integer,
    enabled text,
    roles text,
    extend_info text
);
    ";

    conn.execute_batch(create_table_sql)?;
    Ok(conn)
}

fn insert_config(
    table_seq: &mut TableSeq,
    config_dao: &ConfigDao<'_>,
    config_history_dao: &ConfigHistoryDao<'_>,
    record: TransferRecordRef<'_>,
) -> anyhow::Result<()> {
    todo!()
}

fn insert_namespace(
    table_seq: &mut TableSeq,
    tenant_dao: &TenantDao<'_>,
    record: TransferRecordRef<'_>,
) -> anyhow::Result<()> {
    todo!()
}

fn insert_user(
    table_seq: &mut TableSeq,
    user_dao: &UserDao<'_>,
    record: TransferRecordRef<'_>,
) -> anyhow::Result<()> {
    let value_do = UserDo::from_bytes(&record.value)?;

    let mut user_do: UserDO = value_do.into();
    user_do.id = Some(table_seq.next_user_id());

    user_dao.insert(&user_do)?;

    Ok(())
}
