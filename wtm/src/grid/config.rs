use crate::{util::get_active_monitor_name, ACTIVE_PROFILE};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
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

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct GridConfigKey {
	monitor: String,
	profile: String,
}

impl Default for GridConfigKey {
	fn default() -> Self {
		let monitor = unsafe { get_active_monitor_name() };
		let profile = ACTIVE_PROFILE.get().unwrap().clone();

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
		let mut config = HashMap::new();
		config.insert(GridConfigKey::default(), GridConfig::default());
		config
	}

	fn save(&self) {
		debug!("saving grid config");
	}
}
