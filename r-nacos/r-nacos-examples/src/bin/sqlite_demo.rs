use rusqlite::Connection;

fn main() -> anyhow::Result<()> {
    // 1. 连接数据库
    // 如果文件不存在，它会自动创建
    // ? 是错误处理方式，如果连接失败，程序会报错并停止
    let conn = match Connection::open("sqlite_demo.db") {
        Ok(conn) => conn,
        Err(e) => return Err(anyhow::Error::from(e)),
    };
    println!("connected to sqlite_demo.db");

    // 2. 创建一个表格
    conn.execute(
        r"
    CREATE TABLE IF NOT EXISTS user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    age INTEGER NOT NULL
    );",
        [], // 表示 没有 参数需要替换到 sql 中
    )?;

    println!("create table user success");

    // 3. 插入数据
    conn.execute(
        r"
    insert into user (username, age) values (?1, ?2);",
        ["Alice", "30"],
    )?;

    conn.execute(
        r"
    insert into user (username, age) values (?1, ?2);",
        ["Bob", "25"],
    )?;

    println!("insert user success");

    let mut query_result = conn.prepare(r"select id, username, age from user;")?;

    let rows = query_result.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,    // 从第 0 列取 i32
            row.get::<_, String>(1)?, // 从第 1 列取 string
            row.get::<_, i32>(2)?,
        ))
    })?;

    println!("query result");
    for row in rows {
        let (id, username, age) = row?;
        println!("id: {}, username: {}, age: {}", id, username, age);
    }

    Ok(())
}
