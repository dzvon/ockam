use hello_ockam::Echoer;
use ockam::access_control::AllowAll;
use ockam::{route, stream::Stream, Context, Result, TcpTransport, TCP};
use std::sync::Arc;

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    let _tcp = TcpTransport::create(&ctx).await?;

    // Set the address of the Kafka node you created here. (e.g. "192.0.2.1:4000")
    let hub_node_tcp_address = "<Your node Address copied from hub.ockam.network>";

    // Create a stream client
    Stream::new(&ctx)
        .await?
        .stream_service("stream_kafka")
        .index_service("stream_kafka_index")
        .client_id("stream-over-cloud-node-initiator")
        .connect(
            route![(TCP, hub_node_tcp_address)], // route to hub
            "responder-to-initiator",            // outgoing stream
            "initiator-to-responder",            // incoming stream
        )
        .await?;

    // Start an echoer worker
    ctx.start_worker("echoer", Echoer, Arc::new(AllowAll), Arc::new(AllowAll))
        .await?;

    // Don't call ctx.stop() here so this node runs forever.
    Ok(())
}
