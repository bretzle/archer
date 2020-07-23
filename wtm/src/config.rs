//! Wtm's Config implementation

use crate::{
	hotkey::{HotkeyType, Keybind},
	Result,
};

/// Wtm's Config
#[derive(Debug, Clone)]
pub struct Config {
	/// Distance between windows in pixels
	pub margin: u8,
	/// Distance between monitor egde and windows in pixels
	pub padding: u8,
	/// Should the program start automatically
	pub auto_start: bool,
	/// A vector of keybinds
	pub keybinds: Vec<Keybind>,
}

impl Config {
	pub fn load() -> Result<Self> {
		Ok(Config {
			margin: 10,
			padding: 10,
			auto_start: false,
			keybinds: vec![
				Keybind {
					hotkey: String::from("CTRL+ALT+S"),
					typ: HotkeyType::Main,
				},
				Keybind {
					hotkey: String::from("CTRL+ALT+Q"),
					typ: HotkeyType::QuickResize,
				},
			],
		})
	}
}
