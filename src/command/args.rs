// args.rs - Argument parsing
use super::super::server::default_port;
use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq, Clone)]
pub struct Arguments {
	// Arg list
	pub arguments: Vec<String>,

	// Port (optional)
	#[structopt(long, short)]
	pub port: Option<i32>,
}

impl Arguments {
	pub fn port(&self) -> i32 {
		match self.port {
			None => default_port(),
			Some(x) => x,
		}
	}
}
