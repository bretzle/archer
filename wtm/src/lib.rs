//! Wtm 
//! Window(s) Tiling Manager
//!
//! A simple tiling manager that works natively for Windows

#![allow(non_snake_case)]
#![deny(missing_docs)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

pub mod autostart;
pub mod config;
pub mod event;
pub mod grid;
pub mod hotkey;
pub mod logging;
pub mod tray;
pub mod util;
pub mod window;

use crate::{
	config::Config,
	event::{spawn_foreground_hook, spawn_track_monitor_thread},
	grid::Grid,
	hotkey::spawn_hotkey_thread,
	tray::spawn_sys_tray,
	util::{get_foreground_window, Message, Result},
	window::{spawn_grid_window, spawn_preview_window, Window},
};
use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender};
use std::{
	mem,
	sync::{Arc, Mutex},
};
use winapi::um::winuser::{
	SetForegroundWindow, ShowWindow, TrackMouseEvent, SW_SHOW, TME_LEAVE, TRACKMOUSEEVENT,
};

lazy_static! {
	/// The global `Config` instance
	pub static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::load().unwrap()));
	/// The global channel used to send [Message](util.struct.Message.html]s
	pub static ref CHANNEL: (Sender<Message>, Receiver<Message>) = unbounded();
	/// The global `Grid` instance
	pub static ref GRID: Arc<Mutex<Grid>> =
		Arc::new(Mutex::new(Grid::from(&*CONFIG.lock().unwrap())));
	/// The global Active profile
	pub static ref ACTIVE_PROFILE: Arc<Mutex<String>> = Arc::new(Mutex::new("Default".to_owned()));
}

/// Runs the program
pub fn run() -> Result {
	logging::setup()?;

	let receiver = &CHANNEL.1.clone();
	let sender = &CHANNEL.0.clone();

	let close_channel = bounded::<()>(3);

	let config = CONFIG.lock().unwrap().clone();

	unsafe {
		autostart::toggle_autostart_registry_key(config.auto_start);
	}

	// spawn_hotkey_thread(&config.hotkey, HotkeyType::Main);

	// if let Some(hotkey) = &config.hotkey_quick_resize {
	// 	spawn_hotkey_thread(hotkey, HotkeyType::QuickResize);
	// }

	// if let Some(hotkey_maximize) = &config.hotkey_maximize_toggle {
	// 	spawn_hotkey_thread(hotkey_maximize, HotkeyType::Maximize);
	// }

	// if let Some(hotkey_minimize) = &config.hotkey_minimize {
	// 	spawn_hotkey_thread(hotkey_minimize, HotkeyType::Minimize);
	// }

	for keybind in &config.keybinds {
		spawn_hotkey_thread(&keybind.hotkey, keybind.typ);
	}

	unsafe {
		spawn_sys_tray();
	}

	let mut preview_window: Option<Window> = None;
	let mut grid_window: Option<Window> = None;
	let mut track_mouse = false;

	info!("{:#?}", config);

	loop {
		select! {
			recv(receiver) -> msg => {
				match msg.unwrap() {
					Message::PreviewWindow(window) => unsafe {
						preview_window = Some(window);

						spawn_foreground_hook(close_channel.1.clone());

						ShowWindow(grid_window.as_ref().unwrap().0, SW_SHOW);
						SetForegroundWindow(grid_window.as_ref().unwrap().0);
					}
					Message::GridWindow(window) => {
						grid_window = Some(window);

						let mut grid = GRID.lock().unwrap();

						grid.grid_window = Some(window);
						grid.active_window = Some(get_foreground_window());

						spawn_track_monitor_thread(close_channel.1.clone());
						spawn_preview_window(close_channel.1.clone());
					}
					Message::HighlightZone(rect) => {
						let mut preview_window = preview_window.unwrap_or_default();
						let grid_window = grid_window.unwrap_or_default();

						preview_window.set_pos(rect, Some(grid_window));
					}
					Message::HotkeyPressed(hotkey_type) => {
						hotkey::handle(hotkey_type, &sender, &preview_window, &grid_window);
					}
					Message::TrackMouse(window) => unsafe {
						if !track_mouse {
							let mut event_track: TRACKMOUSEEVENT = mem::zeroed();
							event_track.cbSize = mem::size_of::<TRACKMOUSEEVENT>() as u32;
							event_track.dwFlags = TME_LEAVE;
							event_track.hwndTrack = window.0;

							TrackMouseEvent(&mut event_track);

							track_mouse = true;
						}
					}
					Message::MouseLeft => {
						track_mouse = false;
					}
					Message::ActiveWindowChange(window) => {
						let mut grid = GRID.lock().unwrap();

						if grid.grid_window != Some(window) && grid.active_window != Some(window) {
							grid.active_window = Some(window);
						}
					}
					Message::MonitorChange => {
						let mut grid = GRID.lock().unwrap();

						let active_window = grid.active_window;
						let previous_resize = grid.previous_resize;
						let quick_resize = grid.quick_resize;

						*grid = Grid::from(&*CONFIG.lock().unwrap());

						grid.grid_window = grid_window;
						grid.active_window = active_window;
						grid.previous_resize = previous_resize;
						grid.quick_resize = quick_resize;

						grid.reposition();
					}
					Message::ProfileChange(profile) => {
						{
							let mut active_profile = ACTIVE_PROFILE.lock().unwrap();
							*active_profile = profile.to_owned();
						}

						let mut grid = GRID.lock().unwrap();

						let active_window = grid.active_window;
						let previous_resize = grid.previous_resize;
						let quick_resize = grid.quick_resize;

						*grid = Grid::from(&*CONFIG.lock().unwrap());

						grid.grid_window = grid_window;
						grid.active_window = active_window;
						grid.previous_resize = previous_resize;
						grid.quick_resize = quick_resize;

						grid.reposition();
					}
					Message::InitializeWindows => {
						let mut grid = GRID.lock().unwrap();
						let quick_resize = grid.quick_resize;
						let previous_resize = grid.previous_resize;

						*grid = Grid::from(&*CONFIG.lock().unwrap());

						grid.quick_resize = quick_resize;
						grid.previous_resize = previous_resize;

						spawn_grid_window(close_channel.1.clone());
					}
					Message::CloseWindows => {
						preview_window.take();
						grid_window.take();

						for _ in 0..4 {
							let _ = close_channel.0.send(());
						}

						let mut grid = GRID.lock().unwrap();

						grid.reset();
						track_mouse = false;
					}
					Message::Exit => {
						break;
					}
				}
			}
		}
	}

	Ok(())
}