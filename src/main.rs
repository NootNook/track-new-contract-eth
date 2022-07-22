mod cli;
mod lib;

use chrono::prelude::{Utc};
use ethers::prelude::{Middleware, StreamExt};

// Features

async fn live_pending_deploy_contract() -> Result<(), Box<dyn std::error::Error>> {
    let provider = lib::get_provider();
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
                println!("{}", lib::format_data(tx_hash));
            }
        }
    }

    Ok(())
}

async fn history_deploy_contract(start_timestamp: u64) -> Result<(), Box<dyn std::error::Error>> {
    let provider = lib::get_provider();
    let latest_block = provider.get_block_number().await.unwrap().as_u64();
    let start_block = lib::estimate_block_number_by_timestamp(start_timestamp, latest_block).await;

    let diff_block = latest_block - start_block;

    for current_block in start_block..latest_block {
        if let Some(block) = provider.get_block_with_txs(current_block).await.unwrap() {
            let current_progress = current_block - start_block;
            for tx in block.transactions.iter() {
                if tx.to == None {
                    println!("{} - Progressing... {}/{}", lib::format_data(tx.hash), current_progress, diff_block);
                }
            }
        }
    }

    Ok(())
}

// Main

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::get_parser();

    match matches.subcommand() {
        Some(("live", _)) => live_pending_deploy_contract().await.unwrap(),
        Some(("history", history_matches)) => {
            if let Some(timestamp) = history_matches.get_one::<u64>("timestamp") {
                history_deploy_contract(*timestamp).await.unwrap();
            } else if let Some(start) = history_matches.get_one::<u64>("seconds") {
                let timestamp = Utc::now().timestamp() as u64 - *start;
                history_deploy_contract(timestamp).await.unwrap();
            }
        },
        _ => unreachable!()
    }

    Ok(())
}
