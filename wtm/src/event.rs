//! Event module

use crate::{util::get_active_monitor_name, window::Window, Message, CHANNEL};
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
		let sender = &CHANNEL.0.clone();

		let mut previous_monitor = get_active_monitor_name();

		loop {
			let current_monitor = get_active_monitor_name();

			if current_monitor != previous_monitor {
				previous_monitor = current_monitor.clone();

				let _ = sender.send(Message::MonitorChange);
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
	let sender = &CHANNEL.0.clone();
	let _ = sender.send(Message::ActiveWindowChange(Window(hwnd)));
}
