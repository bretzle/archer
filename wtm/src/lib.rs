//! Wtm
//! Window(s) Tiling Manager
//!
//! A simple tiling manager that works natively for Windows

#[macro_use]
extern crate log;

mod config;
mod event;
mod grid;
mod hotkey;
mod window;

use crate::{
	config::Config,
	event::{spawn_foreground_hook, spawn_track_monitor_thread},
	grid::Grid,
	window::{spawn_grid_window, spawn_preview_window},
};
use crossbeam_channel::{bounded, select};
use event::Event;
use hotkey::HotkeyType::{Main, QuickResize};
use once_cell::sync::OnceCell;
use std::mem;
use winapi::um::winuser::{
	SetForegroundWindow, ShowWindow, TrackMouseEvent, SW_SHOW, TME_LEAVE, TRACKMOUSEEVENT,
};
use winsapi::{EventChannel, GlobalHotkeySet, Key, Modifier, Window};

static mut INSTANCE: OnceCell<TilingManager> = OnceCell::new();

#[derive(Debug, Default)]
pub struct TilingManager {
	config: Config,
	channel: EventChannel<Event>,
	grid: Grid,
}

impl TilingManager {
	pub fn create() -> &'static mut Self {
		unsafe {
			match INSTANCE.get_mut() {
				Some(instance) => instance,
				None => {
					INSTANCE.set(TilingManager::default()).unwrap();
					INSTANCE.get_mut().unwrap()
				}
			}
		}
	}

	pub fn start(&'static self) {
		println!("Starting tiling manager");
		run();
	}
}

/// Runs the program
fn run() {
	let config = unsafe { &INSTANCE.get().unwrap().config };
	let channel = unsafe { &INSTANCE.get().unwrap().channel };
	let mut grid = unsafe { &mut INSTANCE.get_mut().unwrap().grid };

	let receiver = channel.receiver.clone();
	let sender = channel.sender.clone();

	let close_channel = bounded::<()>(3);

	let hotkeys = GlobalHotkeySet::new()
		.add_global_hotkey(
			Event::HotkeyPressed(QuickResize),
			Modifier::Ctrl + Modifier::Alt + Key::Q,
		)
		.add_global_hotkey(
			Event::HotkeyPressed(Main),
			Modifier::Ctrl + Modifier::Alt + Key::S,
		);

	channel.listen_for_hotkeys(hotkeys);

	let mut preview_window: Option<Window> = None;
	let mut grid_window: Option<Window> = None;
	let mut track_mouse = false;

	info!("{:#?}", config);

	loop {
		select! {
			recv(receiver) -> msg => {
				match msg.unwrap() {
					Event::PreviewWindow(window) => unsafe {
						preview_window = Some(window);

						spawn_foreground_hook(close_channel.1.clone());

						ShowWindow(grid_window.as_ref().unwrap().0, SW_SHOW);
						SetForegroundWindow(grid_window.as_ref().unwrap().0);
					}
					Event::GridWindow(window) => {
						grid_window = Some(window);

						let mut grid = unsafe{&mut INSTANCE.get_mut().unwrap().grid};

						grid.grid_window = Some(window);
						grid.active_window = Some(Window::get_foreground_window());

						spawn_track_monitor_thread(close_channel.1.clone());
						spawn_preview_window(close_channel.1.clone());
					}
					Event::HighlightZone(rect) => {
						let mut preview_window = preview_window.unwrap_or_default();
						let grid_window = grid_window.unwrap_or_default();

						preview_window.set_pos(rect, Some(grid_window));
					}
					Event::HotkeyPressed(hotkey_type) => {
						hotkey::handle(hotkey_type, &sender, &preview_window, &grid_window);
					}
					Event::TrackMouse(window) => unsafe {
						if !track_mouse {
							let mut event_track: TRACKMOUSEEVENT = mem::zeroed();
							event_track.cbSize = mem::size_of::<TRACKMOUSEEVENT>() as u32;
							event_track.dwFlags = TME_LEAVE;
							event_track.hwndTrack = window.0;

							TrackMouseEvent(&mut event_track);

							track_mouse = true;
						}
					}
					Event::MouseLeft => {
						track_mouse = false;
					}
					Event::ActiveWindowChange(window) => {
						let mut grid = unsafe{&mut INSTANCE.get_mut().unwrap().grid};

						if grid.grid_window != Some(window) && grid.active_window != Some(window) {
							grid.active_window = Some(window);
						}
					}
					Event::MonitorChange => {
						let mut grid = unsafe{&mut INSTANCE.get_mut().unwrap().grid};

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
					Event::ProfileChange(_) => {
						todo!()
					}
					Event::InitializeWindows => {
						let quick_resize = grid.quick_resize;
						let previous_resize = grid.previous_resize;

						*grid = Grid::from(config);

						grid.quick_resize = quick_resize;
						grid.previous_resize = previous_resize;

						spawn_grid_window(close_channel.1.clone());
					}
					Event::CloseWindows => {
						preview_window.take();
						grid_window.take();

						for _ in 0..4 {
							let _ = close_channel.0.send(());
						}

						grid.reset();
						track_mouse = false;
					}
				}
			}
		}
	}
}
