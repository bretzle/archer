//! Wtm
//! Window(s) Tiling Manager
//!
//! A simple tiling manager that works natively for Windows

mod event;
mod grid;
mod window;

use crate::{
	event::{spawn_foreground_hook, spawn_track_monitor_thread, Event, HotkeyType},
	grid::Grid,
	window::{spawn_grid_window, spawn_preview_window},
};
use crossbeam_channel::select;
use once_cell::sync::OnceCell;
use std::{mem, thread};
use winapi::um::winuser::{
	SetForegroundWindow, ShowWindow, TrackMouseEvent, SW_SHOW, TME_LEAVE, TRACKMOUSEEVENT,
};
use winsapi::{EventChannel, GlobalHotkeySet, Key, Modifier, Window};

static mut INSTANCE: OnceCell<TilingManager> = OnceCell::new();

#[derive(Debug)]
pub struct TilingManager {
	channel: EventChannel<Event>,
	grid: Grid,

	margin: u8,
	padding: u8,

	preview_window: Option<Window>,
	grid_window: Option<Window>,
	track_mouse: bool,

	close_channel: EventChannel<()>,
}

impl Default for TilingManager {
	fn default() -> Self {
		Self {
			channel: Default::default(),
			grid: Default::default(),
			margin: 10,
			padding: 10,
			preview_window: Default::default(),
			grid_window: Default::default(),
			track_mouse: Default::default(),
			close_channel: EventChannel::bounded(3),
		}
	}
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
		thread::spawn(move || {
			let receiver = self.channel.receiver.clone();

			self.setup_hotkeys();

			loop {
				select! {
					recv(receiver) -> msg => {
						self.handle_event(msg.unwrap());
					}
				}
			}
		});
	}

	fn setup_hotkeys(&'static self) {
		let hotkeys = GlobalHotkeySet::new()
			.add_global_hotkey(
				Event::HotkeyPressed(HotkeyType::QuickResize),
				Modifier::Ctrl + Modifier::Alt + Key::Q,
			)
			.add_global_hotkey(
				Event::HotkeyPressed(HotkeyType::Main),
				Modifier::Ctrl + Modifier::Alt + Key::S,
			);

		self.channel.listen_for_hotkeys(hotkeys);
	}

	fn handle_event(&'static self, msg: Event) {
		let tm = unsafe { INSTANCE.get_mut().unwrap() };

		match msg {
			Event::PreviewWindow(window) => unsafe {
				tm.preview_window = Some(window);

				spawn_foreground_hook(self.close_channel.receiver.clone());

				ShowWindow(self.grid_window.as_ref().unwrap().0, SW_SHOW);
				SetForegroundWindow(self.grid_window.as_ref().unwrap().0);
			},
			Event::GridWindow(window) => {
				tm.grid_window = Some(window);

				let mut grid = unsafe { &mut INSTANCE.get_mut().unwrap().grid };

				grid.grid_window = Some(window);
				grid.active_window = Some(Window::get_foreground_window());

				spawn_track_monitor_thread(self.close_channel.receiver.clone());
				spawn_preview_window(self.close_channel.receiver.clone());
			}
			Event::HighlightZone(rect) => {
				let mut preview_window = self.preview_window.unwrap_or_default();
				let grid_window = self.grid_window.unwrap_or_default();

				preview_window.set_pos(rect, Some(grid_window));
			}
			Event::HotkeyPressed(hotkey_type) => self.handle_hotkey(hotkey_type),
			Event::TrackMouse(window) => unsafe {
				if !self.track_mouse {
					let mut event_track: TRACKMOUSEEVENT = mem::zeroed();
					event_track.cbSize = mem::size_of::<TRACKMOUSEEVENT>() as u32;
					event_track.dwFlags = TME_LEAVE;
					event_track.hwndTrack = window.0;

					TrackMouseEvent(&mut event_track);

					tm.track_mouse = true;
				}
			},
			Event::MouseLeft => tm.track_mouse = false,
			Event::ActiveWindowChange(window) => {
				if self.grid.grid_window != Some(window) && self.grid.active_window != Some(window)
				{
					tm.grid.active_window = Some(window);
				}
			}
			Event::MonitorChange => {
				tm.grid.grid_window = self.grid_window;
				tm.grid.reposition();
			}
			Event::ProfileChange(_) => todo!(),
			Event::InitializeWindows => spawn_grid_window(self.close_channel.receiver.clone()),
			Event::CloseWindows => {
				tm.preview_window.take();
				tm.grid_window.take();

				for _ in 0..4 {
					let _ = self.close_channel.sender.send(());
				}

				tm.grid.reset();
				tm.track_mouse = false;
			}
		}
	}

	fn handle_hotkey(&'static self, hotkey: HotkeyType) {
		match hotkey {
			HotkeyType::Main => {
				if self.preview_window.is_some() && self.grid_window.is_some() {
					let _ = self.channel.sender.send(Event::CloseWindows);
				} else {
					let _ = self.channel.sender.send(Event::InitializeWindows);
				}
			}
			HotkeyType::QuickResize => {
				let _ = self.channel.sender.send(Event::InitializeWindows);
				unsafe { INSTANCE.get_mut().unwrap().grid.quick_resize = true };
			}
		}
	}
}
