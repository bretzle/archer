#[derive(Debug, Copy, Clone)]
pub struct Config {
	pub app_bar_height: i32,
	pub app_bar_bg: i32,
	pub app_bar_font: &'static str,
	pub app_bar_font_size: i32,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			app_bar_height: 20,
			app_bar_bg: 0x2C2427,
			app_bar_font: "Consolas",
			app_bar_font_size: 18,
		}
	}
}

impl Config {
	/// Creates a new default config.
	pub fn new() -> Self {
		Self::default()
	}
}
