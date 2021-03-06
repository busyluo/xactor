use std::time::Duration;
use xactor::*;

#[message]
struct Die;

struct MyActor;

#[async_trait::async_trait]
impl Actor for MyActor {
    async fn started(&mut self, ctx: &Context<Self>) {
        // Send the Die message 3 seconds later
        ctx.send_later(Die, Duration::from_secs(3));
    }
}

#[async_trait::async_trait]
impl Handler<Die> for MyActor {
    async fn handle(&mut self, ctx: &Context<Self>, _msg: Die) {
        // Stop the actor without error
        ctx.stop(None);
    }
}

#[xactor::main]
async fn main() {
    // Exit the program after 3 seconds
    MyActor.start().await;
}
