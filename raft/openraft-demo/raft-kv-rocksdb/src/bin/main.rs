use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::info;
use raft_kv_rocksdb::start_example_raft_node;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::Subscriber;

#[derive(Parser, Clone, Debug, Deserialize, Serialize)]
#[clap(author, version, about, long_about = None)]
pub struct Opt {
    #[clap(long)]
    pub id: u64,

    #[clap(long)]
    pub http_addr: String,

    #[clap(long)]
    pub rpc_addr: String,
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // tracing_subscriber::fmt()
    //     .with_target(true)
    //     .with_thread_ids(true)
    //     .with_level(true)
    //     .with_ansi(false)
    //     .with_env_filter(EnvFilter::from_default_env())
    //     .init();


    let filter = tracing_subscriber::filter::LevelFilter::DEBUG;
    let subscriber = Subscriber::builder()
        .with_max_level(filter)
        .with_target(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let options = Opt::parse();


    info!("options: {:?}", serde_json::to_string(&options));

    start_example_raft_node(
        options.id,
        format!("{}.db", options.rpc_addr),
        options.http_addr,
        options.rpc_addr,
    ).await
}
