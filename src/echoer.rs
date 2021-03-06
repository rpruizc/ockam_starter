use ockam::{Context, Result, Routed, Worker};

pub struct Echoer;

#[ockam::worker]
impl Worker for Echoer {
    type Context = Context;
    type Message = String;

    async fn handle_message(&mut self, ctx: &mut Context, msg: Routed<String>) -> Result<()> {
        println!("I'm Echoer. Address: {}, Received: {}", ctx.address(), msg);

        // Echo the message body back on its return route
        ctx.send(msg.return_route(), msg.body()).await
    }
}