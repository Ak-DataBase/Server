pub mod args;

pub use anyhow::Context;
pub use structopt::StructOpt;

use crate::server;

pub fn main() -> anyhow::Result<()> {
	let args = args::Arguments::from_args();
	let port = args.port();
	server::run(port)?;
	Ok(())
}
