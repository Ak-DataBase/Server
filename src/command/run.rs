use super::super::server;
use super::args;

pub fn run(args: args::Arguments) -> anyhow::Result<()> {
	let port = args.port();
	server::run(port)?;
	Ok(())
}
