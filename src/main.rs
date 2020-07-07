#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use common::{get_foreground_window, Rect};
use config::Config;
use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender};
use event::*;
use grid::Grid;
use hotkey::{spawn_hotkey_thread, HotkeyType};
use std::{
	error::Error,
	mem,
	sync::{Arc, Mutex},
};
use tray::spawn_sys_tray;
use winapi::um::winuser::{
	SetForegroundWindow, ShowWindow, TrackMouseEvent, SW_SHOW, TME_LEAVE, TRACKMOUSEEVENT,
};
use window::*;

mod autostart;
mod common;
mod config;
mod event;
mod grid;
mod hotkey;
mod logging;
mod tray;
mod window;

pub type Result<T = (), E = Box<dyn Error>> = std::result::Result<T, E>;

pub enum Message {
	PreviewWindow(Window),
	GridWindow(Window),
	HighlightZone(Rect),
	HotkeyPressed(HotkeyType),
	TrackMouse(Window),
	ActiveWindowChange(Window),
	ProfileChange(&'static str),
	MonitorChange,
	MouseLeft,
	InitializeWindows,
	CloseWindows,
	Exit,
}

lazy_static! {
	static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::load().unwrap()));
	static ref CHANNEL: (Sender<Message>, Receiver<Message>) = unbounded();
	static ref GRID: Arc<Mutex<Grid>> = Arc::new(Mutex::new(Grid::from(&*CONFIG.lock().unwrap())));
	static ref ACTIVE_PROFILE: Arc<Mutex<String>> = Arc::new(Mutex::new("Default".to_owned()));
}

#[macro_export]
macro_rules! str_to_wide {
	($str:expr) => {{
		$str.encode_utf16()
			.chain(std::iter::once(0))
			.collect::<Vec<_>>()
		}};
}

fn main() -> Result {
	logging::setup()?;

	let receiver = &CHANNEL.1.clone();
	let sender = &CHANNEL.0.clone();

	let close_channel = bounded::<()>(3);

	let config = dbg!(CONFIG.lock().unwrap().clone());

	unsafe {
		autostart::toggle_autostart_registry_key(config.auto_start);
	}

	spawn_hotkey_thread(&config.hotkey, HotkeyType::Main);

	if let Some(hotkey) = &config.hotkey_quick_resize {
		spawn_hotkey_thread(hotkey, HotkeyType::QuickResize);
	}

	if let Some(hotkey_maximize) = &config.hotkey_maximize_toggle {
		spawn_hotkey_thread(hotkey_maximize, HotkeyType::Maximize);
	}

	unsafe {
		spawn_sys_tray();
	}

	let mut preview_window: Option<Window> = None;
	let mut grid_window: Option<Window> = None;
	let mut track_mouse = false;

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
						if hotkey_type == HotkeyType::Maximize {
							let mut grid = GRID.lock().unwrap();

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

						} else if preview_window.is_some() && grid_window.is_some() {
							let _ = sender.send(Message::CloseWindows);
						} else {
							let _ = sender.send(Message::InitializeWindows);

							if hotkey_type == HotkeyType::QuickResize {
								GRID.lock().unwrap().quick_resize = true;
							}
						}
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
