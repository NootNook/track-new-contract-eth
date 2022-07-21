use chrono::prelude::{DateTime, Utc};
use ethers::prelude::{Block, Http, Middleware, Provider, StreamExt, H256};
use std::env;

// Helper functions

fn get_provider() -> Provider<Http> {
    const RPC: &'static str = env!("RPC_HTTPS_ETH");
    Provider::<Http>::try_from(RPC).expect("could not instantiate HTTP Provider")
}

fn format_data(hash: H256) -> String {
    let now: DateTime<Utc> = Utc::now();
    format!("{}\nhttps://etherscan.io/tx/{:#x}", now, hash)
}

fn get_timestamp_on_block(block: Block<H256>) -> u64 {
    block.timestamp.as_u64()
}

// Multiple ways, binary search...
//I choose a big approximation
async fn estimate_block_number_by_timestamp(start_timestamp: u64, latest_block: u64) -> u64 {
    const AVERAGE_MINING_TIME: u64 = 13; // https://ycharts.com/indicators/ethereum_average_block_time

    let provider = get_provider();
    let data_latest_block = provider.get_block(latest_block).await.unwrap().unwrap();
    let timestamp_latest = get_timestamp_on_block(data_latest_block);

    latest_block - ((timestamp_latest - start_timestamp) / AVERAGE_MINING_TIME)
}

// Features

async fn live_pending_deploy_contract() -> Result<(), Box<dyn std::error::Error>> {
    let provider = get_provider();
    let mut stream_txs = provider
        .watch_pending_transactions()
        .await
        .unwrap()
        .stream();

    println!("Streaming the deployment of new contracts on the Ethereum blockchain");
    while let Some(tx_hash) = stream_txs.next().await {
        if let Some(tx) = provider.get_transaction(tx_hash).await.unwrap() {
            if tx.to == None {
                // Creation contract
                println!("{}", format_data(tx_hash));
            }
        }
    }

    Ok(())
}

async fn history_deploy_contract(start_timestamp: u64) -> Result<(), Box<dyn std::error::Error>> {
    let provider = get_provider();
    let latest_block = provider.get_block_number().await.unwrap().as_u64();
    let start_block = estimate_block_number_by_timestamp(start_timestamp, latest_block).await;

    let diff_block = latest_block - start_block;

    for current_block in start_block..latest_block {
        if let Some(block) = provider.get_block_with_txs(current_block).await.unwrap() {
            let current_progress = current_block - start_block;
            for tx in block.transactions.iter() {
                if tx.to == None {
                    println!("{} - Progressing... {}/{}", format_data(tx.hash), current_progress, diff_block);
                }
            }
        }
    }

    Ok(())
}

// Main

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("write");
        return Ok(());
    }

    if ["-l", "--live"].contains(&&*args[1]) {
        live_pending_deploy_contract().await.unwrap();
    } else if ["-h", "--history"].contains(&&*args[1]) {
        let start: u64 = args[3].parse().unwrap();
        if ["-t", "--timestamp"].contains(&&*args[2]) {
            history_deploy_contract(start).await.unwrap();
        } else if ["-s", "--seconds"].contains(&&*args[2]) {
            let timestamp = Utc::now().timestamp() as u64 - start;
            history_deploy_contract(timestamp).await.unwrap();
        }
    } else {
        println!("Unknown command...");
    }

    Ok(())
}
