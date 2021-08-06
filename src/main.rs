// main.rs - Main file. Executes cli + server
mod cli;
mod routes;
mod server;
mod utils;

pub use structopt::StructOpt;

fn main() {
	let args = cli::Arguments::from_args();
	server::run(args.port());
}
