use crate::{event::Event, tile_grid::SplitDirection, util, CHANNEL, CONFIG, WORK_MODE};
use key::Key;
use lazy_static::lazy_static;
use modifier::Modifier;
use num_traits::FromPrimitive;
use std::sync::atomic::{AtomicBool, Ordering};
use strum_macros::EnumString;
use winapi::{
	shared::windef::HWND,
	um::winuser::{
		DispatchMessageW, PeekMessageW, RegisterHotKey, TranslateMessage, UnregisterHotKey, MSG,
		PM_REMOVE, WM_HOTKEY,
	},
};

pub mod key;
pub mod modifier;

pub type Command = String;

#[derive(Serialize, Deserialize, Clone, Copy, EnumString, PartialEq, Debug)]
pub enum Direction {
	Left,
	Right,
	Up,
	Down,
}

#[derive(Serialize, Deserialize, Display, Debug, Clone, PartialEq)]
pub enum KeybindingType {
	CloseTile,
	Quit,
	ChangeWorkspace(i32),
	ToggleFloatingMode,
	ToggleWorkMode,
	MoveWorkspaceToMonitor(i32),
	ToggleFullscreen,
	Launch(Command),
	Focus(Direction),
	Swap(Direction),
	MoveToWorkspace(i32),
	Split(SplitDirection),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Keybinding {
	#[serde(rename = "type")]
	pub typ: KeybindingType,
	pub key: Key,
	#[serde(default = "Modifier::default")]
	pub modifier: Modifier,
	#[serde(default)]
	registered: bool,
}

lazy_static! {
	static ref UNREGISTER: AtomicBool = AtomicBool::new(false);
}

fn unregister_keybindings<'a>(keybindings: impl Iterator<Item = &'a mut Keybinding>) {
	for kb in keybindings {
		if kb.registered {
			let key = kb.key as u32;
			let modifier = kb.modifier as u32;
			let id = key + modifier;

			kb.registered = false;

			info!(
				"Unregistering Keybinding({}+{}, {})",
				format!("{:?}", kb.modifier).replace(" | ", "+"),
				kb.key,
				kb.typ
			);

			unsafe {
				UnregisterHotKey(0 as HWND, id as i32);
			}
		}
	}
}

fn register_keybindings<'a>(keybindings: impl Iterator<Item = &'a mut Keybinding>) {
	for kb in keybindings {
		if !kb.registered {
			let key = kb.key as u32;
			let modifier = kb.modifier as u32;
			let id = key + modifier;

			kb.registered = true;

			info!(
				"Registering Keybinding({}+{}, {})",
				format!("{:?}", kb.modifier).replace(" | ", "+"),
				kb.key,
				kb.typ
			);

			unsafe {
				util::winapi_nullable_to_result(RegisterHotKey(
					0 as HWND, id as i32, modifier, key,
				))
				.expect("Failed to register keybinding");
			}
		}
	}
}

pub fn register() -> Result<(), Box<dyn std::error::Error>> {
	std::thread::spawn(|| {
		let mut keybindings = CONFIG.lock().unwrap().keybindings.clone();
		let mut msg: MSG = MSG::default();

		while UNREGISTER.load(Ordering::SeqCst) {
			debug!("Waiting for other thread get cleaned up");
			// as long as another thread gets unregistered we cant start a new one
			std::thread::sleep(std::time::Duration::from_millis(10))
		}

		if WORK_MODE.load(Ordering::SeqCst) {
			register_keybindings(keybindings.iter_mut());
		} else {
			register_keybindings(
				keybindings
					.iter_mut()
					.filter(|kb| kb.typ == KeybindingType::ToggleWorkMode),
			);
		}

		unsafe {
			loop {
				if UNREGISTER.load(Ordering::SeqCst) {
					debug!("Unregistering hot key manager");
					unregister_keybindings(keybindings.iter_mut());
					UNREGISTER.store(false, Ordering::SeqCst);
					break;
				}

				while PeekMessageW(&mut msg, 0 as HWND, 0, 0, PM_REMOVE) > 0 {
					TranslateMessage(&msg);
					DispatchMessageW(&msg);

					if msg.message == WM_HOTKEY {
						let modifier = (msg.lParam & 0xffff) as u32;

						if let Some(key) = Key::from_isize(msg.lParam >> 16) {
							for kb in &keybindings {
								if kb.key == key && kb.modifier as u32 == modifier {
									CHANNEL
										.sender
										.clone()
										.send(Event::Keybinding(kb.clone()))
										.expect("Failed to send key event");
								}
							}
						}
					}
				}

				let work_mode = WORK_MODE.load(Ordering::SeqCst);
				if !work_mode {
					unregister_keybindings(
						keybindings
							.iter_mut()
							.filter(|kb| kb.typ != KeybindingType::ToggleWorkMode),
					);
				} else {
					register_keybindings(keybindings.iter_mut());
				}

				std::thread::sleep(std::time::Duration::from_millis(5));
			}
		}
	});

	Ok(())
}

pub fn unregister() {
	UNREGISTER.store(true, Ordering::SeqCst);
}
