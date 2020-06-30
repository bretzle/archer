use fs::{read_to_string, File};
use std::io::Write;
use std::{error::Error, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
	pub display_app_bar: bool,
	pub remove_title_bar: bool,
	pub remove_task_bar: bool,
	pub work_mode: bool,
	pub margin: i32,
	pub padding: i32,
	pub app_bar_height: i32,
	pub rules: Vec<Rule>,
	pub keybindings: Vec<Keybind>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keybind {}

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
			if let Ok(mut file) = File::create(path.clone()) {
				debug!("Initializeing config with default values");
				file.write_all(include_bytes!("../DEFAULT_CONFIG.toml"))?;
			}
		}

		let content = read_to_string(path)?;
		let config = toml::from_str(&content)?;

		Ok(config)
	}
}
