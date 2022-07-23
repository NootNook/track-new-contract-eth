use crate::utils::{get_provider, format_data, estimate_block_number_by_timestamp};
use ethers::providers::{Middleware, StreamExt};

pub async fn live_pending_deploy_contract() -> Result<(), Box<dyn std::error::Error>> {
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

pub async fn history_deploy_contract(start_timestamp: u64) -> Result<(), Box<dyn std::error::Error>> {
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