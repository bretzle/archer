//! Wtm
//! Window(s) Tiling Manager
//!
//! A simple tiling manager that works natively for Windows

#[macro_use]
extern crate log;

pub mod config;
pub mod event;
pub mod grid;
pub mod hotkey;
pub mod util;
pub mod window;

use crate::{
	config::Config,
	event::{spawn_foreground_hook, spawn_track_monitor_thread},
	grid::Grid,
	hotkey::spawn_hotkey_thread,
	util::{get_foreground_window, Message, Result},
	window::{spawn_grid_window, spawn_preview_window, Window},
};
use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender};
use once_cell::sync::OnceCell;
use std::mem;
use winapi::um::winuser::{
	SetForegroundWindow, ShowWindow, TrackMouseEvent, SW_SHOW, TME_LEAVE, TRACKMOUSEEVENT,
};

static CONFIG: OnceCell<Config> = OnceCell::new();
static CHANNEL: OnceCell<(Sender<Message>, Receiver<Message>)> = OnceCell::new();
static mut GRID: OnceCell<Grid> = OnceCell::new();

/// Runs the program
pub fn run() -> Result {
	let config = CONFIG.get_or_init(|| Config::load().unwrap());
	let channel = CHANNEL.get_or_init(unbounded);
	let mut grid = unsafe {
		GRID.set(Grid::from(config)).unwrap();
		GRID.get_mut().unwrap()
	};

	let receiver = channel.1.clone();
	let sender = channel.0.clone();

	let close_channel = bounded::<()>(3);

	for keybind in &config.keybinds {
		spawn_hotkey_thread(&keybind.hotkey, keybind.typ);
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

						let mut grid = unsafe{GRID.get_mut().unwrap()};

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
						let mut grid = unsafe{GRID.get_mut().unwrap()};

						if grid.grid_window != Some(window) && grid.active_window != Some(window) {
							grid.active_window = Some(window);
						}
					}
					Message::MonitorChange => {
						let mut grid = unsafe{GRID.get_mut().unwrap()};

						let active_window = grid.active_window;
						let previous_resize = grid.previous_resize;
						let quick_resize = grid.quick_resize;

						*grid = Grid::from(config);

						grid.grid_window = grid_window;
						grid.active_window = active_window;
						grid.previous_resize = previous_resize;
						grid.quick_resize = quick_resize;

						grid.reposition();
					}
					Message::ProfileChange(_) => {
						todo!()
					}
					Message::InitializeWindows => {
						let quick_resize = grid.quick_resize;
						let previous_resize = grid.previous_resize;

						*grid = Grid::from(config);

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
