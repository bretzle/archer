#[derive(Debug, Copy, Clone)]
pub struct Config {
	pub height: i32,
	pub bg_color: i32,
	pub font_name: &'static str,
	pub font_size: i32,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			height: 20,
			bg_color: 0x2C2427,
			font_name: "Consolas",
			font_size: 18,
		}
	}
}
