#[derive(Debug, Clone)]
pub struct Config {
	pub margin: u8,
	pub padding: u8,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			margin: 10,
			padding: 10,
		}
	}
}
