// This node routes a message, to a worker on a different node, over the tcp transport
use ockam::{Context, Result, Route, TcpTransport, TCP};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // Initialize the TCP transport
    let tcp = TcpTransport::create(&ctx).await?;

    // Create a TCP connection
    tcp.connect("127.0.0.1:4000").await?;

    // Send a message to the "echoer" worker, on a different node, over tcp
    ctx.send(
        Route::new()
            .append_t(TCP, "127.0.0.1:4000")
            .append("echoer"),
        // The message to be echoed
        "Echoed message!".to_string(),
    )
        .await?;

    // Wait to receive a reply and print it
    let reply = ctx.receive::<String>().await?;
    println!("App received: {}", reply);

    // Stop workers, the node, cleanup and return
    ctx.stop().await
}