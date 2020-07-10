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

pub type Result<T = (), E = Box<dyn Error>> = std::result::Result<T, E>;

#[derive(Debug)]
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

#[macro_export]
macro_rules! str_to_wide {
	($str:expr) => {{
		$str.encode_utf16()
			.chain(std::iter::once(0))
			.collect::<Vec<_>>()
		}};
}

pub fn get_foreground_window() -> Window {
	let hwnd = unsafe { GetForegroundWindow() };
	Window(hwnd)
}

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

pub fn report_and_exit(error_msg: &str) {
	show_msg_box(error_msg);
	process::exit(1);
}

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
