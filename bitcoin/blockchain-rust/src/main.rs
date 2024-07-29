use log::LevelFilter;
use structopt::StructOpt;
use blockchain_rust::{Blockchain, GLOBAL_CONFIG, Server, validate_address};

/// mine 标志是指块立即会被同一节点挖出来 ，必须要有这个标志，因为初始状态时，网络中没有矿工节点
const MINE_TRUE: usize = 1;

#[derive(Debug, StructOpt)]
#[structopt(name = "blockchain_rust")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "create-blockchain", about = "Create a new blockchain")]
    CreateBlockchain {
        #[structopt(name = "address", help = "The address to send genesis block reward to")]
        address: String,
    },

    #[structopt(name = "create-wallet", about = "Create a new wallet")]
    CreateWallet,

    #[structopt(name = "get-balance", about = "Get the wallet balance of the target address")]
    GetBalance {
        #[structopt(name = "address", help = "The wallet address")]
        address: String,
    },

    #[structopt(name = "list-addresses", about = "Print local wallet address")]
    ListAddresses,

    #[structopt(name = "send", about = "Add new block to chain")]
    Send {
        #[structopt(name = "from", help = "Source wallet address")]
        from: String,

        #[structopt(name = "to", help = "Destination wallet address")]
        to: String,

        #[structopt(name = "amount", help = "Amount to send")]
        amount: i32,

        #[structopt(name = "mine", help = "Mine immediately on the same node")]
        mine: usize,
    },

    #[structopt(name = "print-chain", about = "Print blockchain all block")]
    PrintChain,

    #[structopt(name = "reindex-utxo", about = "rebuild UTXO set")]
    ReindexUtxo,

    #[structopt(name = "start-node", about = "Start a node")]
    StartNode {
        #[structopt(name = "miner", help = "Enable mining mode and send reward to ADDRESS")]
        miner: Option<String>,
    },
}
fn main() {

    env_logger::builder().filter_level(LevelFilter::Info).init();

    let opt = Opt::from_args();


    match opt.command {
        Command::CreateBlockchain { .. } => {}

        Command::CreateWallet => {}

        Command::GetBalance { .. } => {}

        Command::ListAddresses => {}

        Command::Send { .. } => {}

        Command::PrintChain => {}

        Command::ReindexUtxo => {}

        Command::StartNode { miner} => {
            if let Some(addr) = miner {
                if validate_address(&addr) == false {
                    panic!("Wrong miner address")
                }

                println!("Mining is on. Address to receive rewards: {}", addr);

                GLOBAL_CONFIG.set_mining_addr(addr);

            }

            let blockchain = Blockchain::new_blockchain();
            let socket_addr = GLOBAL_CONFIG.get_node_addr();
            Server::new(blockchain).run(&socket_addr);
        }
    }
}

























