use crate::{
	event::{Event, WinEvent},
	util::PtrExt,
	AppBar, CHANNEL,
};
use log::{debug, info};
use std::{ffi::CString, ptr, thread, time::Duration};
use winapi::{
	shared::{
		minwindef::{DWORD, HINSTANCE, LPARAM, LRESULT, UINT, WPARAM},
		ntdef::LONG,
		windef::{HBRUSH, HDC, HWINEVENTHOOK, HWND},
		windowsx::GET_X_LPARAM,
	},
	um::{
		libloaderapi::GetModuleHandleA,
		wingdi::{CreateFontIndirectA, CreateSolidBrush, SelectObject, LOGFONTA},
		winuser::{
			BeginPaint, DefWindowProcA, DispatchMessageW, EndPaint, GetClientRect, GetMessageW,
			LoadCursorA, PeekMessageW, RegisterClassA, SendMessageA, SetCursor, SetWinEventHook,
			ShowWindow, TranslateMessage, EVENT_MAX, EVENT_MIN, IDC_ARROW, MSG, OBJID_WINDOW,
			PAINTSTRUCT, PM_REMOVE, SW_HIDE, SW_SHOW, WM_CLOSE, WM_CREATE, WM_LBUTTONDOWN,
			WM_PAINT, WM_SETCURSOR, WNDCLASSA,
		},
	},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum RedrawReason {
	Time,
}

impl Default for RedrawReason {
	fn default() -> Self {
		RedrawReason::Time
	}
}

unsafe extern "system" fn window_cb(
	hwnd: HWND,
	msg: UINT,
	w_param: WPARAM,
	l_param: LPARAM,
) -> LRESULT {
	if msg == WM_CLOSE {
		AppBar::get().window = 0;
	} else if msg == WM_SETCURSOR {
		// Force a normal cursor. This probably shouldn't be done this way but whatever
		SetCursor(LoadCursorA(ptr::null_mut(), IDC_ARROW as *const i8));
	} else if msg == WM_LBUTTONDOWN {
		let x = GET_X_LPARAM(l_param);
		info!("Received mouse click @ {}", x);
	} else if msg == WM_CREATE {
		info!("loading font");
		load_font();
	} else if !hwnd.is_null() && msg == WM_PAINT {
		info!("Received paint");
		let reason = AppBar::get().redraw_reason;
		debug!("Reason for paint was {:?}", reason);
		let mut paint = PAINTSTRUCT::default();

		GetClientRect(hwnd, &mut paint.rcPaint);

		BeginPaint(hwnd, &mut paint);

		let components = &AppBar::get().components;

		if let Some(component) = components.get(&reason) {
			component
				.draw(hwnd)
				.unwrap_or_else(|_| panic!("Failed to draw component: {:?}", component));
		}

		EndPaint(hwnd, &paint);
	}

	DefWindowProcA(hwnd, msg, w_param, l_param)
}

unsafe extern "system" fn handler(
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

	if AppBar::get().window == window_handle as i32 {
		return;
	}

	if let Some(event) = WinEvent::from_code(event_code, window_handle as i32) {
		CHANNEL.get().unwrap().sender.send(Event::WinEvent(event)).unwrap();
	}
}

pub fn redraw(reason: RedrawReason) {
	unsafe {
		let hwnd = AppBar::get().window as HWND;

		if hwnd == 0 as HWND {
			return;
		}

		AppBar::get().redraw_reason = reason;

		//TODO: handle error
		SendMessageA(hwnd, WM_PAINT, 0, 0);
	}
}

pub fn set_font(dc: HDC) {
	unsafe {
		SelectObject(dc, AppBar::get().font as *mut std::ffi::c_void);
	}
}

pub fn load_font() {
	unsafe {
		let config = AppBar::get().config;
		let mut logfont = LOGFONTA::default();
		let mut font_name: [i8; 32] = [0; 32];
		let app_bar_font = config.font;
		let app_bar_font_size = config.font_size;

		for (i, byte) in CString::new(app_bar_font)
			.unwrap()
			.as_bytes()
			.iter()
			.enumerate()
		{
			font_name[i] = *byte as i8;
		}

		logfont.lfHeight = app_bar_font_size;
		logfont.lfFaceName = font_name;

		let font = CreateFontIndirectA(&logfont) as i32;

		debug!("Using font {}", font);

		AppBar::get().font = font;
	}
}

pub fn create() {
	info!("Creating appbar");
	let name = "app_bar";
	let app = AppBar::get();
	let config = app.config;

	let height = config.height;
	let display_width = AppBar::get().display.width;

	let window = &app.window;
	let channel = &CHANNEL.get().unwrap().sender;

	for component in app.components.values() {
		info!("Setting up component: {:?}", component);
		component.setup(window, channel.clone());
	}

	thread::spawn(move || unsafe {
		//TODO: Handle error
		let instance = GetModuleHandleA(ptr::null_mut());
		//TODO: Handle error
		let background_brush = CreateSolidBrush(config.bg_color as u32);

		let class = WNDCLASSA {
			hInstance: instance as HINSTANCE,
			lpszClassName: name.as_ptr() as *const i8,
			lpfnWndProc: Some(window_cb),
			hbrBackground: background_brush as HBRUSH,
			..WNDCLASSA::default()
		};

		RegisterClassA(&class);

		//TODO: handle error
		let window_handle = winapi::um::winuser::CreateWindowExA(
			winapi::um::winuser::WS_EX_NOACTIVATE | winapi::um::winuser::WS_EX_TOPMOST,
			name.as_ptr() as *const i8,
			name.as_ptr() as *const i8,
			winapi::um::winuser::WS_POPUPWINDOW & !winapi::um::winuser::WS_BORDER,
			0,
			0,
			display_width,
			height,
			ptr::null_mut(),
			ptr::null_mut(),
			instance as HINSTANCE,
			ptr::null_mut(),
		);

		AppBar::get().window = window_handle as i32;

		let hwnd = show();

		for component in AppBar::get().components.values() {
			component.draw(hwnd).expect("Failed to draw datetime")
		}

		let mut msg: MSG = MSG::default();
		while GetMessageW(&mut msg, window_handle, 0, 0) > 0 {
			TranslateMessage(&msg);
			DispatchMessageW(&msg);
		}
	});

	thread::spawn(|| unsafe {
		let mut msg: MSG = MSG::default();

		debug!("Registering win event hook");

		let _hook = SetWinEventHook(
			EVENT_MIN,
			EVENT_MAX,
			ptr::null_mut(),
			Some(handler),
			0,
			0,
			0,
		)
		.as_result()
		.unwrap();

		loop {
			while PeekMessageW(&mut msg, 0 as HWND, 0, 0, PM_REMOVE) > 0 {
				TranslateMessage(&msg);
				DispatchMessageW(&msg);
			}

			thread::sleep(Duration::from_millis(5));
		}
	});
}

#[allow(dead_code)]
pub fn hide() {
	unsafe {
		let hwnd = AppBar::get().window as HWND; // Need to eager evaluate else there is a deadlock
		ShowWindow(hwnd, SW_HIDE);
	}
}

pub fn show() -> HWND {
	let hwnd = AppBar::get().window as HWND; // Need to eager evaluate else there is a deadlock

	unsafe {
		ShowWindow(hwnd, SW_SHOW);
	}

	hwnd
}
