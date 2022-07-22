use chrono::prelude::{DateTime, Utc};
use ethers::prelude::{Block, Http, Middleware, Provider, H256};
use std::env;

pub fn get_provider() -> Provider<Http> {
    const RPC: &'static str = env!("RPC_HTTPS_ETH");
    Provider::<Http>::try_from(RPC).expect("could not instantiate HTTP Provider")
}

pub fn format_data(hash: H256) -> String {
    let now: DateTime<Utc> = Utc::now();
    format!("{}\nhttps://etherscan.io/tx/{:#x}", now, hash)
}

pub fn get_timestamp_on_block(block: Block<H256>) -> u64 {
    block.timestamp.as_u64()
}

// Multiple ways, binary search...
//I choose a big approximation
pub async fn estimate_block_number_by_timestamp(start_timestamp: u64, latest_block: u64) -> u64 {
    const AVERAGE_MINING_TIME: u64 = 13; // https://ycharts.com/indicators/ethereum_average_block_time

    let provider = get_provider();
    let data_latest_block = provider.get_block(latest_block).await.unwrap().unwrap();
    let timestamp_latest = get_timestamp_on_block(data_latest_block);

    latest_block - ((timestamp_latest - start_timestamp) / AVERAGE_MINING_TIME)
}