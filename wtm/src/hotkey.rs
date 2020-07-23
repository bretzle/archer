//! Hotkey module

use crate::{Event, INSTANCE};
use crossbeam_channel::Sender;
use winsapi::Window;

/// The Commands that a keybind can execute
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum HotkeyType {
	/// Open the grid window and resize as many windows until executed again
	Main,
	/// Quick Resize the current window
	QuickResize,
}

/// Process's commands when a keybind is pressed
pub fn handle(
	hotkey: HotkeyType,
	sender: &Sender<Event>,
	preview_window: &Option<Window>,
	grid_window: &Option<Window>,
) {
	match hotkey {
		HotkeyType::Main => {
			if preview_window.is_some() && grid_window.is_some() {
				let _ = sender.send(Event::CloseWindows);
			} else {
				let _ = sender.send(Event::InitializeWindows);
			}
		}
		HotkeyType::QuickResize => {
			let _ = sender.send(Event::InitializeWindows);
			unsafe { INSTANCE.get_mut().unwrap().grid.quick_resize = true };
		}
	}
}
