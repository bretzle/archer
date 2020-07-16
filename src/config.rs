#[derive(Clone)]
pub struct Config {
	pub app_bar_height: i32,
	pub app_bar_bg: i32,
	pub app_bar_font: String,
	pub app_bar_font_size: i32,
	pub app_bar_workspace_bg: i32,
	pub work_mode: bool,
	pub launch_on_startup: bool,
	pub margin: i32,
	pub padding: i32,
	pub remove_title_bar: bool,
	pub remove_task_bar: bool,
	pub display_app_bar: bool,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			app_bar_height: 20,
			app_bar_bg: 0x2c2427,
			app_bar_font: String::from("Consolas"),
			app_bar_font_size: 18,
			app_bar_workspace_bg: 0x161616,
			launch_on_startup: false,
			margin: 0,
			padding: 0,
			remove_title_bar: true,
			work_mode: true,
			remove_task_bar: true,
			display_app_bar: true,
		}
	}
}

impl Config {
	/// Creates a new default config.
	pub fn new() -> Self {
		Self::default()
	}
}
