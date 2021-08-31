// cli.rs - Argument parsing
use crate::server::DEFAULT_PORT;
use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq, Clone)]
pub struct Arguments {
	// Arg list
	pub arguments: Vec<String>,

	// Port (optional)
	#[structopt(long, short, default_value = DEFAULT_PORT)]
	pub port: i32
}
