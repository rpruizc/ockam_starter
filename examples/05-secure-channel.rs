// This node creates a secure channel and routes messages through it

use ockam::{Context, Result, Route, SecureChannel, Vault};
use ockam_get_started::Echoer;

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // Start an Echoer worker at "echoer" address
    ctx.start_worker("echoer", Echoer).await?;

    let vault = Vault::create(&ctx)?;

    // Create a secure channel listener
    SecureChannel::create_listener(&mut ctx, "secure_channel_listener", &vault).await?;

    // Connect to a secure channel listener and perform a handshake
    let channel = SecureChannel::create(&mut ctx, "secure_channel_listener", &vault).await?;

    // Send a message to the echoer worker, via the secure channel
    ctx.send(
        // Route to the "echoer" worker via the secure channel
        Route::new().append(channel.address()).append("echoer"),
        "Hello Ockam!".to_string(),
    ).await?;

    let reply = ctx.receive::<String>().await?;
    println!("App received: {}", reply);

    ctx.stop().await
}