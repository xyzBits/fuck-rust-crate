use crate::errors::Result;

pub struct Cli {}

impl Cli {
    pub fn new() -> Result<Cli> {
        todo!()
    }

    pub fn run(&mut self) -> Result<()> {
        todo!()
    }

    fn cmd_send(from: &str, to: &str, amount: i32, mine_now: bool) -> Result<()> {
        todo!()
    }

    fn cmd_create_wallet() -> Result<String> {
        todo!()
    }

    fn cmd_reindex() -> Result<i32> {
        todo!()
    }

    fn cmd_create_blockchain(address: &str) -> Result<()> {
        todo!()
    }

    fn cmd_get_balance(address: &str) -> Result<i32> {
        todo!()
    }

    fn cmd_print_chain() -> Result<()> {
        todo!()
    }

    fn cmd_list_address() -> Result<()> {
        todo!()
    }
}
