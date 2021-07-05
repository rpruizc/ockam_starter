use ockam::{Context, Result, Route, TcpTransport, TCP};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    let tcp = TcpTransport::create(&ctx).await?;
    tcp.connect("127.0.0.1:3000").await?;
    ctx.send(
        Route::new()
            .append_t(TCP, "127.0.0.1:3000")
            .append_t(TCP, "127.0.0.1:4000")
            .append("echoer"),
        "Hello there!".to_string(),
    )
        .await?;

    let reply = ctx.receive::<String>().await?;
    println!("App received: {}", reply);

    ctx.stop().await
}