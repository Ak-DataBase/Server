// main.rs - Main file. Executes cli + server
mod cli;
mod server;

pub use structopt::StructOpt;

fn main() {
	let args = cli::Arguments::from_args();
	server::run(args.port());
}
