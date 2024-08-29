use axum::handler::get;
use axum::Router;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::runtime::Builder;

fn main() {
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    // 阻塞到异步的任务一直完成，这是一个常驻的服务
    runtime.block_on(async move {
        // 一个请求分好几个
        let app = Router::new().route("/", get(root));

        let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
}

async fn root() -> &'static str {
    "Hello, World!"
}
