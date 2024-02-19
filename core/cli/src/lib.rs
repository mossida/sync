use std::path::PathBuf;

use clap::{Parser, Subcommand};
use cnf::PKG_NAME;
use start::StartCommandArgs;

mod start;

#[derive(Debug, Parser)]
#[command(name = "Sync command-line interface", bin_name = PKG_NAME)]
#[command(version, about, long_about = None)]
struct Cli {
	#[arg(short, long, value_name = "FILE")]
	config: Option<PathBuf>,
	#[command(subcommand)]
	command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
	#[command(about = "Starts the server")]
	Start(StartCommandArgs),
}

pub async fn init() -> miette::Result<()> {
	let args = Cli::parse();
	cnf::init(args.config)?;

	match args.command {
		Commands::Start(args) => start::init(args).await?,
	};

	Ok(())
}
