use crate::{
	event::{Event, WinEvent},
	INSTANCE,
};
use log::{debug, info};
use std::ptr;
use winapi::{
	shared::{
		minwindef::{DWORD, LPARAM, LRESULT, UINT, WPARAM},
		ntdef::LONG,
		windef::{HWINEVENTHOOK, HWND},
		windowsx::{GET_X_LPARAM, GET_Y_LPARAM},
	},
	um::winuser::{
		BeginPaint, DefWindowProcA, EndPaint, GetClientRect, LoadCursorA, SetCursor, IDC_ARROW,
		OBJID_WINDOW, PAINTSTRUCT, WM_CLOSE, WM_CREATE, WM_LBUTTONDOWN, WM_PAINT, WM_SETCURSOR,
	},
};
use winsapi::{DeviceContext, Font};

pub unsafe extern "system" fn window_cb(
	hwnd: HWND,
	msg: UINT,
	w_param: WPARAM,
	l_param: LPARAM,
) -> LRESULT {
	if msg == WM_CLOSE {
		INSTANCE.get_mut().unwrap().window = None;
	} else if msg == WM_SETCURSOR {
		// Force a normal cursor. This probably shouldn't be done this way but whatever
		SetCursor(LoadCursorA(ptr::null_mut(), IDC_ARROW as *const i8));
	} else if msg == WM_LBUTTONDOWN {
		let x = GET_X_LPARAM(l_param);
		let y = GET_Y_LPARAM(l_param);
		info!("Received click @ ({}, {})", x, y);
	} else if msg == WM_CREATE {
		info!("loading font");
		let app = INSTANCE.get_mut().unwrap();
		app.font = Font::create(app.config.font_name, app.config.font_size).unwrap();
	} else if !hwnd.is_null() && msg == WM_PAINT {
		let reason = &INSTANCE.get().unwrap().redraw_reason;
		debug!("Reason for paint was {:?}", reason);
		let mut paint = PAINTSTRUCT::default();

		GetClientRect(hwnd, &mut paint.rcPaint);

		BeginPaint(hwnd, &mut paint);

		let app = &INSTANCE.get().unwrap();
		let components = &app.components;

		if let Some(component) = components.get(reason) {
			component
				.draw(
					app.draw_data.as_ref().unwrap(),
					DeviceContext::new(hwnd).unwrap(),
				)
				.unwrap_or_else(|_| panic!("Failed to draw component: {:?}", component));
		}

		EndPaint(hwnd, &paint);
	}

	DefWindowProcA(hwnd, msg, w_param, l_param)
}

pub unsafe extern "system" fn win_event_handler(
	_: HWINEVENTHOOK,
	event_code: DWORD,
	window_handle: HWND,
	object_type: LONG,
	_: LONG,
	_: DWORD,
	_: DWORD,
) {
	if object_type != OBJID_WINDOW {
		return;
	}

	let app = INSTANCE.get().unwrap();

	if app.window.unwrap() == window_handle as i32 {
		return;
	}

	if let Some(event) = WinEvent::from_code(event_code, window_handle as i32) {
		app.channel.sender.send(Event::WinEvent(event)).unwrap();
	}
}
