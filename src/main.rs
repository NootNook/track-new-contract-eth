use chrono::prelude::{DateTime, Utc};
use ethers::prelude::{Block, Http, Middleware, Provider, StreamExt, H256};
use clap::{Command, Arg, value_parser};
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
    let matches = Command::new("tracksm-eth")
    .about("Tracknig new contracts on blockchain Ethereum")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .author("NootNook")
    .subcommand(
        Command::new("live")
        .about("Live pending contract on blockchain")
        .short_flag('l')
        .long_flag("live")
    )
    .subcommand(
        Command::new("history")
        .about("History of contract deploys")
        .short_flag('h')
        .long_flag("history")
        .arg(
            Arg::new("timestamp")
            .short('t')
            .long("timestamp")
            .takes_value(true)
            .conflicts_with("seconds")
            .required(true)
            .value_parser(value_parser!(u64))
            .help("History of the timestamp until the last block on the chain ")
        )
        .arg(
            Arg::new("seconds")
            .short('s')
            .long("seconds")
            .takes_value(true)
            .conflicts_with("timestamp")
            .required(true)
            .value_parser(value_parser!(u64))
            .help("History from last block to last block - seconds")
        )
    )
    .get_matches();

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
