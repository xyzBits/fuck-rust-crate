use serde::{Deserialize, Serialize};

/// Deserialize 可以从字符串转为结构体
/// Serialize 可以从结构体转为字符串
#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    age: i32,
}

#[test]
fn test_serde() {
    let json_str = r#"
    {"name": "alex", "age": 30}
    "#
    .into();

    let mut user: User = serde_json::from_str(json_str).unwrap();
    let mut user: User = serde_json::from_slice(json_str.as_bytes()).unwrap();

    println!("user = {:?}", user);

    user.age = 43;
    user.name = "tom".into();

    let json = serde_json::to_string_pretty(&user).unwrap();

    println!("json = {}", json);
}
