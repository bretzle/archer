//! Event module

use crate::{hotkey::HotkeyType, util::get_active_monitor_name, INSTANCE};
use crossbeam_channel::{select, Receiver};
use std::{mem, ptr, thread, time::Duration};
use winapi::{
	shared::{
		minwindef::DWORD,
		windef::{HWINEVENTHOOK, HWND},
	},
	um::{
		winnt::LONG,
		winuser::{
			DispatchMessageW, PeekMessageW, SetWinEventHook, TranslateMessage,
			EVENT_SYSTEM_FOREGROUND, WINEVENT_OUTOFCONTEXT,
		},
	},
};
use winsapi::{Rect, Window};

/// Messages that are sent over [CHANNEL](../struct.CHANNEL.html)
#[derive(Debug)]
pub enum Event {
	///
	PreviewWindow(Window),
	///
	GridWindow(Window),
	/// Highlight the hovered area over the grid window
	HighlightZone(Rect),
	/// A registered hotkey was pressed
	HotkeyPressed(HotkeyType),
	/// Tracks the mouse over the grid window
	TrackMouse(Window),
	/// The active window changed
	ActiveWindowChange(Window),
	/// A different profile was activated
	ProfileChange(&'static str),
	/// The active monitor changed
	MonitorChange,
	/// Mouse left the Grid window
	MouseLeft,
	/// Draw the grid window
	InitializeWindows,
	/// Close the windows drawn by wtm
	CloseWindows,
}

// TODO figure out what this does
///
pub fn spawn_foreground_hook(close_msg: Receiver<()>) {
	thread::spawn(move || unsafe {
		SetWinEventHook(
			EVENT_SYSTEM_FOREGROUND,
			EVENT_SYSTEM_FOREGROUND,
			ptr::null_mut(),
			Some(callback),
			0,
			0,
			WINEVENT_OUTOFCONTEXT,
		);

		let mut msg = mem::zeroed();
		loop {
			if PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, 1) > 0 {
				TranslateMessage(&msg);
				DispatchMessageW(&msg);
			};

			select! {
				recv(close_msg) -> _ => break,
				default(Duration::from_millis(10)) => {}
			}
		}
	});
}

/// Keeps track of which monitor is active
pub fn spawn_track_monitor_thread(close_msg: Receiver<()>) {
	thread::spawn(move || unsafe {
		let sender = INSTANCE.get().unwrap().channel.sender.clone();

		let mut previous_monitor = get_active_monitor_name();

		loop {
			let current_monitor = get_active_monitor_name();

			if current_monitor != previous_monitor {
				previous_monitor = current_monitor.clone();

				let _ = sender.send(Event::MonitorChange);
			}

			select! {
				recv(close_msg) -> _ => {
					break;
				}
				default(Duration::from_millis(10)) => {}
			}
		}
	});
}

unsafe extern "system" fn callback(
	_: HWINEVENTHOOK,
	_event: DWORD,
	hwnd: HWND,
	_: LONG,
	_: LONG,
	_: DWORD,
	_: DWORD,
) {
	let sender = INSTANCE.get().unwrap().channel.sender.clone();
	let _ = sender.send(Event::ActiveWindowChange(Window(hwnd)));
}
