pub mod args;
pub mod run;

pub use anyhow::Context;
pub use structopt::StructOpt;

pub fn main() -> anyhow::Result<()> {
	run::run(args::Arguments::from_args())?;
	Ok(())
}
