use clap::{Command, Arg, ArgMatches, value_parser};

pub fn get_parser() -> ArgMatches{
    Command::new("tracksm-eth")
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
    .get_matches()
}

