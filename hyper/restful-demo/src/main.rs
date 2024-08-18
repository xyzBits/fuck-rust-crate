
// 需要在 toml 中添加依赖时，加入 derive feature
#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    id: u32, 
    name: String, 
}
#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

