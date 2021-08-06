// cli.rs - Argument parsing
use crate::server::DEFAULT_PORT;
use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq, Clone)]
pub struct Arguments {
	// Arg list
	pub arguments: Vec<String>,

	// Port (optional)
	#[structopt(long, short)]
	pub port: Option<i32>
}

impl Arguments {
	pub fn port(&self) -> i32 {
		match self.port {
			None => DEFAULT_PORT,
			Some(x) => x
		}
	}
}
