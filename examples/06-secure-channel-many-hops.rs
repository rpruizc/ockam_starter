use ockam::{Context, Result, Route, SecureChannel, Vault};
use ockam_get_started::{Echoer, Hop};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    ctx.start_worker("echoer", Echoer).await?;

    ctx.start_worker("h1", Hop).await?;
    ctx.start_worker("h2", Hop).await?;
    ctx.start_worker("h3", Hop).await?;

    let vault = Vault::create(&ctx)?;

    SecureChannel::create_listener(&mut ctx, "secure_channel_listener", &vault).await?;

    let channel = SecureChannel::create(
        &mut ctx,
        Route::new()
            .append("h1")
            .append("h2")
            .append("h3")
            .append("secure_channel_listener"),
        &vault
    ).await?;

    ctx.send(
        Route::new().append(channel.address()).append("echoer"),
        "Hello Ockam".to_string(),
    ).await?;

    let reply = ctx.receive::<String>().await?;
    println!("App received: {}", reply);

    ctx.stop().await
}