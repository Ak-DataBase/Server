// main.rs - Main file. Executes CLI
mod command;
mod server;

fn main() -> anyhow::Result<()> {
	command::main()?;
	Ok(())
}
