use std::env;
use ethers::prelude::{Provider, Http, Middleware, StreamExt};
use chrono::prelude::{DateTime, Local};
use serde_json::to_string_pretty;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const RPC: &'static str = env!("RPC_HTTPS_ETH");

    let provider = Provider::<Http>::try_from(RPC).expect("could not instantiate HTTP Provider");
    let mut stream_txs = provider.watch_pending_transactions().await.unwrap().stream();

    println!("Streaming the deployment of new contracts on the Ethereum blockchain");
    while let Some(tx_hash) = stream_txs.next().await {
        if let Some(tx) = provider.get_transaction(tx_hash).await.unwrap() {
            if tx.to == None { // Creation contract
                let now: DateTime<Local> = Local::now();
                println!("{}\nhttps://etherscan.io/tx/{:#x}", now, tx.hash);
                let serialized = to_string_pretty(&tx).unwrap();
                println!("{}", serialized);
            }
        }
    }

    Ok(())
}