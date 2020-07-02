use crate::hot_key_manager::Keybinding;
use fs::File;
use regex::Regex;
use std::{error::Error, fs, io::Write, path::PathBuf};

pub mod hot_reloading;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
	#[serde(with = "serde_regex")]
	pub pattern: Regex,
	#[serde(default)]
	pub has_custom_titlebar: bool,
	#[serde(default = "_true")]
	pub manage: bool,
	#[serde(default)]
	pub chromium: bool,
	#[serde(default)]
	pub firefox: bool,
	#[serde(default = "_true")]
	pub remove_frame: bool,
	#[serde(default = "_workid")]
	pub workspace: i32,
}

fn _true() -> bool {
	true
}

fn _workid() -> i32 {
	-1
}

impl Default for Rule {
	fn default() -> Self {
		Self {
			pattern: Regex::new("").unwrap(),
			has_custom_titlebar: false,
			manage: true,
			remove_frame: true,
			chromium: false,
			firefox: false,
			workspace: -1,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkspaceSetting {
	pub id: i32,
	pub monitor: i32,
}

impl Default for WorkspaceSetting {
	fn default() -> Self {
		Self {
			id: -1,
			monitor: -1,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
	pub app_bar_height: i32,
	pub app_bar_bg: i32,
	pub app_bar_font: String,
	pub app_bar_font_size: i32,
	pub work_mode: bool,
	pub light_theme: bool,
	pub multi_monitor: bool,
	pub launch_on_startup: bool,
	pub margin: i32,
	pub padding: i32,
	pub remove_title_bar: bool,
	pub remove_task_bar: bool,
	pub display_app_bar: bool,
	#[serde(default = "Vec::new")]
	pub workspace_settings: Vec<WorkspaceSetting>,
	#[serde(default = "Vec::new")]
	pub keybindings: Vec<Keybinding>,
	#[serde(default = "Vec::new")]
	pub rules: Vec<Rule>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			app_bar_height: 20,
			app_bar_bg: 0x2e3440,
			app_bar_font: String::from("Consolas"),
			app_bar_font_size: 18,
			launch_on_startup: false,
			margin: 0,
			padding: 0,
			remove_title_bar: false,
			work_mode: true,
			light_theme: false,
			multi_monitor: false,
			remove_task_bar: false,
			display_app_bar: false,
			workspace_settings: Vec::new(),
			keybindings: Vec::new(),
			rules: Vec::new(),
		}
	}
}

impl Config {
	pub fn load() -> Result<Self, Box<dyn Error>> {
		let mut path = match dirs::config_dir() {
			Some(path) => path,
			None => PathBuf::new(),
		};

		path.push("wtm");

		if !path.exists() {
			debug!("Config folder doesn't exist. Creating folder.");
			fs::create_dir(path.clone())?;
		}

		path.push("config.toml");

		if !path.exists() {
			debug!("Config file doesn't exist. Creating file.");
			if let Ok(mut file) = File::create(path) {
				debug!("Initializeing config with default values");
				file.write_all(include_bytes!("../DEFAULT_CONFIG.toml"))?;
			}
		}

		// let content = fs::read_to_string(path)?;
		let content = include_str!("../DEFAULT_CONFIG.toml");
		let config = toml::from_str(&content)?;

		Ok(config)
	}
}
