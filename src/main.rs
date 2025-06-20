use actix::prelude::*;

struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
struct Ping(usize);

impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

#[actix::main]
async fn main() {
    // start new actor
   let addr = MyActor{ count: 10}.start();
   let res = addr.send(Ping(10)).await;
   //
   println!("res: {}", res.unwrap() == 20);

   System::current().stop();
}
