use super::super::server;
use super::args;

pub async fn run(args: args::Arguments) -> anyhow::Result<()> {
	let port = args.port();
	server::run(port).await?;
	Ok(())
}
