mod cli; mod subcommand;

use chrono::prelude::{Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::get_parser();

    match matches.subcommand() {
        Some(("live", _)) => subcommand::live_pending_deploy_contract().await.unwrap(),
        Some(("history", history_matches)) => {
            if let Some(timestamp) = history_matches.get_one::<u64>("timestamp") {
                subcommand::history_deploy_contract(*timestamp).await.unwrap();
            } else if let Some(start) = history_matches.get_one::<u64>("seconds") {
                let timestamp = Utc::now().timestamp() as u64 - *start;
                subcommand::history_deploy_contract(timestamp).await.unwrap();
            }
        },
        _ => unreachable!()
    }

    Ok(())
}
