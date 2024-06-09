use actix::{Actor, Context, Handler, Message, System};



struct Ping(usize);

impl Message for Ping {
    type Result = usize;
    
}

struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
    
}

impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        self.count += msg.0;
        self.count
    }
    
}


#[actix::main]
async fn main() {

    let addr = MyActor {count: 10}.start();

    let res = addr.send(Ping(10)).await;

    println!("RESULT: {}", res.unwrap() == 20);


    println!("Hello, world!");

    System::current().stop();
}
