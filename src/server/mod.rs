pub fn default_port() -> i32 {
	3000
}

pub fn run(port: i32) -> anyhow::Result<()> {
	println!("Starting server on port {}", port);
	Ok(())
}
