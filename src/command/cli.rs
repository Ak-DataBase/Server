// cli.rs - The main file where args can be used
use super::super::server;
use super::args;

pub fn match_cmds(args: args::Arguments) -> anyhow::Result<()> {
	let port = args.port();
	server::run(port)?;
	Ok(())
}
