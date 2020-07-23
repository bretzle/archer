//! Wtm's Config implementation

/// Wtm's Config
#[derive(Debug, Clone)]
pub struct Config {
	/// Distance between windows in pixels
	pub margin: u8,
	/// Distance between monitor egde and windows in pixels
	pub padding: u8,
	/// Should the program start automatically
	pub auto_start: bool,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			margin: 10,
			padding: 10,
			auto_start: false,
		}
	}
}
