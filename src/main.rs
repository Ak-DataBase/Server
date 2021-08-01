// main.rs - Main file. Executes CLI
mod command;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	command::main().await?;
	Ok(())
}
