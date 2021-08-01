pub mod args;
pub mod run;

pub use anyhow::Context;
pub use structopt::StructOpt;

pub async fn main() -> anyhow::Result<()> {
	run::run(args::Arguments::from_args()).await?;
	Ok(())
}
