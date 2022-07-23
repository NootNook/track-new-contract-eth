use track_new_contract_eth::{cli, subcommand};
use chrono::prelude::{Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let matches = cli::get_parser();

  match matches.subcommand() {
    Some(("live", _)) => {
      println!("Streaming the deployment of new contracts on the Ethereum blockchain...");
      subcommand::live_pending_deploy_contract().await.unwrap();
    },
    Some(("history", history_matches)) => {
      let now = Utc::now().timestamp() as u64;
      let show_progress_status = *history_matches.get_one::<bool>("verbose").unwrap();
      if let Some(start) = history_matches.get_one::<u64>("timestamp") {
        println!("Scanning from {} to {} (timestamp)...", start, now);
        subcommand::history_deploy_contract(*start, show_progress_status).await.unwrap();
      } else if let Some(start) = history_matches.get_one::<u64>("seconds") {
        let start_timestamp = now - *start;
        println!("Scanning from {} to {} (timestamp)...", start_timestamp, now);
        subcommand::history_deploy_contract(start_timestamp, show_progress_status).await.unwrap();
      }
    },
    _ => unreachable!()
  }

  Ok(())
}
