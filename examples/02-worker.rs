use ockam::{Context, Result};
use ockam_get_started::Echoer;

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // Start an Echoer worker at address "echoer"
    ctx.start_worker("echoer", Echoer).await?;

    // Send a message to the "echoer" worker
    ctx.send("echoer", "Hello World!".to_string()).await?;

    // Wait to receive a reply and print it
    let reply = ctx.receive::<String>().await?;
    println!("App received: {}", reply);

    ctx.stop().await
}