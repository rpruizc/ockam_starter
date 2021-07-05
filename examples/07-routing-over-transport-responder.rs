use ockam::{Context, Result, TcpTransport};
use ockam_get_started::Echoer;

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    // Initialize TCP transport
    let tcp = TcpTransport::create(&ctx).await?;

    // Create a TCP listener and wait for incoming connections
    tcp.listen("127.0.0.1:4000").await?;

    // Create an echoer worker
    ctx.start_worker("echoer", Echoer).await?;

    // Don't call ctx.stop() so the node runs forever
    Ok(())
}