use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use eyre::Result;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    // Create the provider.
    let rpc_url = "wss://eth-mainnet.g.alchemy.com/v2/mAxrDc7SL8D1WbRrlc6Ahhz_1NSlJV3x";
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    let sub = provider.subscribe_blocks().await?;

    let mut stream = sub.into_stream().take(4);

    println!("Awaiting block headers...");

    let handle = tokio::spawn(async move {
        while let Some(header) = stream.next().await {
            println!("Latest block header: {}", header.number);
        }
    });

    handle.await?;

    Ok(())
}