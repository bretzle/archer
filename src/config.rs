
use crate::{hotkey::Keybind, Result};
use std::{
	fs::{self, File},
	io::Write,
	path::PathBuf,
};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
	pub margin: u8,
	pub padding: u8,
	pub auto_start: bool,
	pub keybinds: Vec<Keybind>,
}

impl Config {
	/// Loads the config
	#[cfg(not(feature = "dev-cfg"))]
	pub fn load() -> Result<Self> {
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

		let content = fs::read_to_string(path)?;
		let config = toml::from_str(&content)?;

		Ok(config)
	}

	/// Will use the default config
	/// Useful for testing new features / commands
	#[cfg(feature = "dev-cfg")]
	pub fn load() -> Result<Self> {
		let content = include_str!("../DEFAULT_CONFIG.toml");
		let config = toml::from_str(&content).unwrap();
		Ok(config)
	}

	pub fn toggle_autostart() {
		todo!()
	}
}
