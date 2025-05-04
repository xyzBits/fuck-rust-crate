use rusqlite::{Connection, Row, params_from_iter};

fn result2option<T>(r: rusqlite::Result<T>) -> Option<T> {
    match r {
        Ok(v) => Some(v),
        _ => None,
    }
}

pub fn get_row_value<T>(r: &Row, name: &str) -> Option<T>
where
    T: rusqlite::types::FromSql,
{
    match r.column_index(name) {
        Ok(i) => result2option(r.get(i)),
        _ => None,
    }
}

pub fn convert_json_param(val: &serde_json::Value) -> rusqlite::types::ToSqlOutput<'_> {
    todo!()
}

pub fn convert_json_params(inputs: &[serde_json::Value]) -> Vec<rusqlite::types::ToSqlOutput<'_>> {
    inputs.iter().map(convert_json_param).collect()
}

pub fn sqlite_execute(
    conn: &Connection,
    sql: &str,
    args: &[serde_json::Value],
) -> anyhow::Result<usize> {
    // println!("sqlite_execute sql:{}, args:{}", &sql, &args);

    let result = conn.execute(sql, params_from_iter(convert_json_params(args).iter()));
    Ok(result?)
}
