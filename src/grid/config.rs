use crate::{common::get_active_monitor_name, ACTIVE_PROFILE};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub(super) struct GridConfig {
	pub rows: usize,
	pub columns: usize,
}

impl Default for GridConfig {
	fn default() -> Self {
		GridConfig {
			rows: 2,
			columns: 2,
		}
	}
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct GridConfigKey {
	monitor: String,
	profile: String,
}

impl Default for GridConfigKey {
	fn default() -> Self {
		let monitor = unsafe { get_active_monitor_name() };
		let profile = ACTIVE_PROFILE.lock().unwrap().clone();

		GridConfigKey { monitor, profile }
	}
}

pub(super) type GridConfigs = HashMap<GridConfigKey, GridConfig>;
pub(super) trait GridCache {
	fn load() -> GridConfigs;
	fn save(&self);
}

impl GridCache for GridConfigs {
	fn load() -> GridConfigs {
		if let Some(mut config_path) = dirs::config_dir() {
			config_path.push("wtm");
			config_path.push("cache");

			if !config_path.exists() {
				let _ = fs::create_dir_all(&config_path);
			}

			config_path.push("grid.ron");

			if let Ok(file) = fs::File::open(config_path) {
				if let Ok(config) = ron::de::from_reader(file) {
					return config;
				}
			}
		}

		let mut config = HashMap::new();
		config.insert(GridConfigKey::default(), GridConfig::default());
		config
	}

	fn save(&self) {
		debug!("saving grid config");

		if let Some(mut config_path) = dirs::config_dir() {
			config_path.push("wtm");
			config_path.push("cache");
			config_path.push("grid.ron");

			if let Ok(serialized) = ron::ser::to_string(&self) {
				let _ = fs::write(config_path, serialized);
			}
		}
	}
}
