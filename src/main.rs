// main.rs - Main file. Executes cli + server
mod cli;
mod db;
mod server;
#[cfg(test)]
mod tests;
mod utils;

pub(crate) use structopt::StructOpt;

fn main() {
	let args = cli::Arguments::from_args();
	server::run(args.port.to_string());
}
