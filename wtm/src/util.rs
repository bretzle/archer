//! Utility module

mod rect;

use crate::{hotkey::HotkeyType, str_to_wide, window::Window};
use std::{error::Error, mem, process, ptr};
use winapi::{
	shared::windef::POINT,
	um::winuser::{
		GetCursorPos, GetForegroundWindow, GetMonitorInfoW, MessageBoxW, MonitorFromPoint, MB_OK,
		MONITORINFOEXW, MONITOR_DEFAULTTONEAREST,
	},
};

pub use rect::Rect;

/// Custom Result re-export
pub type Result<T = (), E = Box<dyn Error>> = std::result::Result<T, E>;

/// Messages that are sent over [CHANNEL](../struct.CHANNEL.html)
#[derive(Debug)]
pub enum Message {
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

/// Converts a str to the format that the windows api uses
#[macro_export]
macro_rules! str_to_wide {
	($str:expr) => {{
		$str.encode_utf16()
			.chain(std::iter::once(0))
			.collect::<Vec<_>>()
		}};
}

/// Gets the Active window
pub fn get_foreground_window() -> Window {
	Window(unsafe { GetForegroundWindow() })
}

/// Gets a [Rect](struct.Rect.html) over the active monitor
pub unsafe fn get_work_area() -> Rect {
	let active_monitor = {
		let mut cursor_pos: POINT = mem::zeroed();
		GetCursorPos(&mut cursor_pos);

		MonitorFromPoint(cursor_pos, MONITOR_DEFAULTTONEAREST)
	};

	let work_area: Rect = {
		let mut info: MONITORINFOEXW = mem::zeroed();
		info.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;

		GetMonitorInfoW(active_monitor, &mut info as *mut MONITORINFOEXW as *mut _);

		info.rcWork.into()
	};

	work_area
}

/// Gets the name of the active monitor
pub unsafe fn get_active_monitor_name() -> String {
	let active_monitor = {
		let mut cursor_pos: POINT = mem::zeroed();
		GetCursorPos(&mut cursor_pos);

		MonitorFromPoint(cursor_pos, MONITOR_DEFAULTTONEAREST)
	};

	let mut info: MONITORINFOEXW = mem::zeroed();
	info.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;

	GetMonitorInfoW(active_monitor, &mut info as *mut MONITORINFOEXW as *mut _);

	String::from_utf16_lossy(&info.szDevice)
}

/// Displays an error message and exits the program
pub fn report_and_exit(error_msg: &str) {
	show_msg_box(error_msg);
	process::exit(1);
}

/// Creates a message box window that shows an error message
pub fn show_msg_box(message: &str) {
	let mut message = str_to_wide!(message);

	unsafe {
		MessageBoxW(
			ptr::null_mut(),
			message.as_mut_ptr(),
			ptr::null_mut(),
			MB_OK,
		);
	}
}
