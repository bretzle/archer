use crate::rect::Rect;
use std::mem;
use winapi::{
	shared::windef::{HMONITOR, POINT},
	um::winuser::{
		GetCursorPos, GetMonitorInfoW, MonitorFromPoint, MONITORINFOEXW, MONITOR_DEFAULTTONEAREST,
	},
};

pub struct Monitor(pub HMONITOR);

impl Monitor {
	pub fn get_active() -> Monitor {
		let active_monitor = unsafe {
			let mut cursor_pos: POINT = mem::zeroed();
			GetCursorPos(&mut cursor_pos);

			MonitorFromPoint(cursor_pos, MONITOR_DEFAULTTONEAREST)
		};
		Monitor(active_monitor)
	}

	pub fn name(&self) -> String {
		unsafe {
			let mut info: MONITORINFOEXW = mem::zeroed();
			info.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;

			GetMonitorInfoW(self.0, &mut info as *mut MONITORINFOEXW as *mut _);

			String::from_utf16_lossy(&info.szDevice)
		}
	}

	pub fn area(&self) -> Rect {
		unsafe {
			let work_area: Rect = {
				let mut info: MONITORINFOEXW = mem::zeroed();
				info.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;

				GetMonitorInfoW(self.0, &mut info as *mut MONITORINFOEXW as *mut _);

				info.rcWork.into()
			};

			work_area
		}
	}
}
