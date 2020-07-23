//! Utility module

use std::mem;
use winapi::{
	shared::windef::POINT,
	um::winuser::{
		GetCursorPos, GetMonitorInfoW, MonitorFromPoint, MONITORINFOEXW, MONITOR_DEFAULTTONEAREST,
	},
};
use winsapi::Rect;

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
