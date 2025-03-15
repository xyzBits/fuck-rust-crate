use alloy::node_bindings::Anvil;
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use eyre::Result;
use futures_util::{StreamExt, stream};

#[tokio::main]
async fn main() -> Result<()> {
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    let ws = WsConnect::new(anvil.ws_endpoint());
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    let subscription = provider.subscribe_blocks().await?;
    let mut stream = subscription.into_stream().take(2);

    while let Some(header) = stream.next().await {
        println!("Received block number: {}", header.number);
    }

    let poller = provider.watch_blocks().await?;
    let mut stream = poller.into_stream().flat_map(stream::iter).take(2);

    while let Some(block_hash) = stream.next().await {
        println!("Polled for block header: {:?}", block_hash);
    }

    Ok(())
}
