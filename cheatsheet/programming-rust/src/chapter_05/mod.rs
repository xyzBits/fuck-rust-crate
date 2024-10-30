use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

#[test]
fn test_1() {
    let mut table = Table::new();

    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );

    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );

    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(table);

    table = Table::new();
    println!("{:?}", table);
}

// 程序调用 table 时，整个结构体会被移动到函数时，变量 table 将变为未初始化
fn show(table: Table) {
    for (artist, works) in table {
        println!(" works by {}", artist);

        for work in works {
            println!("  {}", work);
        }
    }
}
