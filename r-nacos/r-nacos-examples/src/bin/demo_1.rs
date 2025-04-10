use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, put, web};
use r_nacos_examples::cli;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Mutex;
// parse() 是 clap::Parser trait 提供的方法，而 Rust 要求 trait 在调用其他方法时必须在作用域内
// 不引入，rust 编译器找不到 parse() 方法，使用 uer::Parser 让编译器知道 可以调用 parse 方法
use clap::Parser;

// actix_web::main 是 Actix Web 提供的一个属性宏，主要作用是将一个普通的异步async函数
// 转为 Actix Web 应用程序的入口，让你的main函数能够运行 actix web 的异步环境，用于启动 web服务器
// actix_web::main 是一个方便的工具，简化了 web 应用程序的启动过程，通过在异步函数上添加这个宏
// 可以直接编写异步代码运行web 服务器，无需手动配置运行
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("version: {}", get_app_version());

    for (key, value) in env::vars() {
        // println!("key = {}: version = {}", key, value);
    }

    let cli_opt = cli::Cli::parse();

    // 从环境变量中读取名为 RUST_LOG 的值，并将其存储在变量 rust_log 中，如果环境变量读取失败，则使用默认值 info
    // std::env::var 是 std lig 中的函数，用于读取指定名称的环境变量
    let rust_log = std::env::var("RUST_LOG")
        // 对 Result 的处理，如果是 Ok，返回其内部的值
        // 如果是 Err，返回其提供的默认值
        .unwrap_or("debug".to_owned());
    // 为日志级别提供默认值
    unsafe {
        std::env::set_var("RUST_LOG", &rust_log);
    }

    // 结构体的一个方法，用于在所有路由和处理函数中共享的资源，例如数据库连接池，配置设置或者其他全局状态
    let app_state = Data::new(AppState {
        users: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        // 创建 http 服务器实例
        // 配置应用程序的路由和逻辑
        App::new() // 配置 根路由的 get 请求
            .route("/", web::get().to(root))
            .route("/hello", web::get().to(hello))
            .app_data(app_state.clone())
            .service(get_users)
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind("127.0.0.1:6789")? // 绑定到本地 5678 端口
    .run() // 启动服务器
    .await // 等待服务器运行完成
}

fn get_app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

async fn root() -> impl Responder {
    HttpResponse::Ok().body("This is root.")
}

// 一个简单的路由处理函数
// #[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
}

struct AppState {
    users: Mutex<Vec<User>>,
}

#[get("/users")]
async fn get_users(state: web::Data<AppState>) -> impl Responder {
    let users = state.users.lock().unwrap();
    HttpResponse::Ok().json(&*users)
}

#[get("/users/{id}")]
async fn get_user(path: web::Path<u32>, state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let users = state.users.lock().unwrap();
    if let Some(user) = users.iter().find(|u| u.id == id) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[post("/users")]
async fn create_user(new_user: web::Json<User>, state: web::Data<AppState>) -> impl Responder {
    let mut users = state.users.lock().unwrap();
    users.push(new_user.into_inner());
    HttpResponse::Created().finish()
}

#[put("/users/{id}")]
async fn update_user(
    path: web::Path<u32>,
    updated_user: web::Json<User>,
    state: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let mut users = state.users.lock().unwrap();

    if let Some(user) = users.iter_mut().find(|u| u.id == id) {
        *user = updated_user.into_inner();
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[delete("/users/{id}")]
async fn delete_user(path: web::Path<u32>, state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let mut users = state.users.lock().unwrap();
    if let Some(pos) = users.iter().position(|u| u.id == id) {
        users.remove(pos);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
