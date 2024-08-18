use std::pin::Pin;

use actix::{Actor, Context, Handler, Message, System};



struct Ping;

impl Message for Ping {
    // define return type
    type Result = Result<bool, std::io::Error>;
}

// Define actor
struct MyActor {

}

// Provide Actor implement for our actor 
impl Actor for MyActor {
    type Context = Context<Self>;

    
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Actor is alive");
    }
    
    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("Actor is stopped");
    }
}

// Define handler for Ping message 
impl Handler<Ping> for MyActor {
    type Result = Result<bool, std::io::Error>;
    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        println!("Ping received");

        Ok(true)
    }
}



#[actix::main]
async fn main() {
    let addr = MyActor.start();

}
