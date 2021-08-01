pub fn default_port() -> i32 {
	7878
}

pub async fn run(port: i32) -> anyhow::Result<()> {
	println!("Starting server on port {}", port);

	// TODO - server

	println!("Server started");

	Ok(())
}
