//! Hotkey module

use crate::{
	util::{get_foreground_window, report_and_exit},
	window::Window,
	Message, CHANNEL, GRID,
};
use crossbeam_channel::Sender;
use std::{mem, ptr, thread};
use winapi::um::winuser::{
	DispatchMessageW, GetKeyboardLayout, GetMessageW, RegisterHotKey, TranslateMessage,
	VkKeyScanExW, MOD_ALT, MOD_CONTROL, MOD_NOREPEAT, MOD_SHIFT, MOD_WIN, WM_HOTKEY,
};

/// The Commands that a keybind can execute
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum HotkeyType {
	/// Open the grid window and resize as many windows until executed again
	Main,
	/// Quick Resize the current window
	QuickResize,
	/// Maximizes the current window
	Maximize,
	/// Minimizes the current window
	Minimize,
}

/// A keybind
#[derive(Debug, Clone)]
pub struct Keybind {
	/// The sequence of key(s) that need to be pressed in combination to execute a command
	pub hotkey: String,
	/// The command that the keybind should execute
	pub typ: HotkeyType,
}

/// Spawn's a thread that will listen for a specific hotkey and sends a Message to execute a command
/// See [Keybind](struct.Keybind.html) and [hotkey::handle](fn.handle.html) for more information
pub fn spawn_hotkey_thread(hotkey_str: &str, hotkey_type: HotkeyType) {
	let mut hotkey: Vec<String> = hotkey_str
		.split('+')
		.map(|s| s.trim().to_string())
		.collect();

	if hotkey.len() < 2 || hotkey.len() > 5 {
		report_and_exit(&format!(
			"Invalid hotkey <{}>: Combination must be between 2 to 5 keys long.",
			hotkey_str
		));
	}

	let virtual_key_char = hotkey.pop().unwrap().chars().next().unwrap();

	let hotkey_str = hotkey_str.to_owned();
	thread::spawn(move || unsafe {
		let sender = CHANNEL.get().unwrap().0.clone();

		let result = RegisterHotKey(
			ptr::null_mut(),
			0,
			compile_modifiers(&hotkey, &hotkey_str) | MOD_NOREPEAT as u32,
			get_vkcode(virtual_key_char),
		);

		if result == 0 {
			report_and_exit(&format!("Failed to assign hot key <{}>. Either program is already running or hotkey is already assigned in another program.", hotkey_str));
		}

		let mut msg = mem::zeroed();
		while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) != 0 {
			TranslateMessage(&msg);
			DispatchMessageW(&msg);

			if msg.message == WM_HOTKEY {
				let _ = sender.send(Message::HotkeyPressed(hotkey_type));
			}
		}
	});
}

fn compile_modifiers(activators: &[String], hotkey_str: &str) -> u32 {
	let mut code: u32 = 0;
	for key in activators {
		match key.as_str() {
            "ALT" => code |= MOD_ALT as u32,
            "CTRL" => code |= MOD_CONTROL as u32,
            "SHIFT" => code |= MOD_SHIFT as u32,
            "WIN" => code |= MOD_WIN as u32,
            _ => report_and_exit(&format!("Invalid hotkey <{}>: Unidentified modifier in hotkey combination. Valid modifiers are CTRL, ALT, SHIFT, WIN.", hotkey_str))
        }
	}
	code
}

unsafe fn get_vkcode(key_char: char) -> u32 {
	let keyboard_layout = GetKeyboardLayout(0);
	let vk_code = VkKeyScanExW(key_char as u16, keyboard_layout);

	if vk_code == -1 {
		report_and_exit(&format!("Invalid key {} in hotkey combination.", key_char));
	}

	vk_code.to_be_bytes()[1] as u32
}

/// Process's commands when a keybind is pressed
pub fn handle(
	hotkey: HotkeyType,
	sender: &Sender<Message>,
	preview_window: &Option<Window>,
	grid_window: &Option<Window>,
) {
	match hotkey {
		HotkeyType::Minimize => get_foreground_window().minimize(),
		HotkeyType::Maximize => {
			let mut grid = unsafe { GRID.get_mut().unwrap() };

			let mut active_window = if grid_window.is_some() {
				grid.active_window.unwrap()
			} else {
				let active_window = get_foreground_window();
				grid.active_window = Some(active_window);
				active_window
			};

			let active_rect = active_window.rect();

			active_window.restore();

			let mut max_rect = grid.get_max_area();
			max_rect.adjust_for_border(active_window.transparent_border());

			if let Some((_, previous_rect)) = grid.previous_resize {
				if active_rect == max_rect {
					active_window.set_pos(previous_rect, None);
				} else {
					active_window.set_pos(max_rect, None);
				}
			} else {
				active_window.set_pos(max_rect, None);
			}

			grid.previous_resize = Some((active_window, active_rect));
		}
		HotkeyType::Main => {
			if preview_window.is_some() && grid_window.is_some() {
				let _ = sender.send(Message::CloseWindows);
			} else {
				let _ = sender.send(Message::InitializeWindows);
			}
		}
		HotkeyType::QuickResize => {
			let _ = sender.send(Message::InitializeWindows);
			unsafe { GRID.get_mut().unwrap().quick_resize = true };
		}
	}
}
