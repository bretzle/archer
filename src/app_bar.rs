use crate::{
	event::{Event, WinEvent},
	INSTANCE,
};
use log::{debug, info};
use std::{ptr, thread, time::Duration};
use winapi::{
	shared::{
		minwindef::{DWORD, HINSTANCE, LPARAM, LRESULT, UINT, WPARAM},
		ntdef::LONG,
		windef::{HBRUSH, HWINEVENTHOOK, HWND},
		windowsx::{GET_X_LPARAM, GET_Y_LPARAM},
	},
	um::{
		libloaderapi::GetModuleHandleA,
		wingdi::CreateSolidBrush,
		winuser::{
			BeginPaint, CreateWindowExA, DefWindowProcA, DispatchMessageW, EndPaint, GetClientRect,
			GetMessageW, LoadCursorA, PeekMessageW, RegisterClassA, SendMessageA, SetCursor,
			SetWinEventHook, ShowWindow, TranslateMessage, EVENT_MAX, EVENT_MIN, IDC_ARROW, MSG,
			OBJID_WINDOW, PAINTSTRUCT, PM_REMOVE, SW_HIDE, SW_SHOW, WM_CLOSE, WM_CREATE,
			WM_LBUTTONDOWN, WM_PAINT, WM_SETCURSOR, WNDCLASSA,
		},
	},
};
use winsapi::{DeviceContext, Font, PtrExt};

pub type RedrawReason = String;

unsafe extern "system" fn window_cb(
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

	let app = INSTANCE.get().unwrap();

	if app.window.unwrap() == window_handle as i32 {
		return;
	}

	if let Some(event) = WinEvent::from_code(event_code, window_handle as i32) {
		app.channel.sender.send(Event::WinEvent(event)).unwrap();
	}
}

pub fn redraw(reason: RedrawReason) {
	unsafe {
		let hwnd = INSTANCE.get().unwrap().window.unwrap() as HWND;

		if hwnd == 0 as HWND {
			return;
		}

		INSTANCE.get_mut().unwrap().redraw_reason = reason;

		//TODO: handle error
		SendMessageA(hwnd, WM_PAINT, 0, 0);
	}
}

pub fn create() {
	info!("Creating appbar");
	let name = "app_bar";
	let app = unsafe { INSTANCE.get().unwrap() };
	let config = app.config;

	let height = config.height;
	let display_width = app.display.width;

	let window = &app.window;
	let channel = &app.channel.sender;

	app.components.values().for_each(|component| {
		info!("Setting up component: {:?}", component);

		thread::spawn(move || loop {
			thread::sleep(component.interval());
			if window.is_none() {
				break;
			}
			channel
				.send(Event::RedrawAppBar(component.reason()))
				.expect("Failed to send redraw event");
		});
	});

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
		let window_handle = CreateWindowExA(
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

		let app = INSTANCE.get_mut().unwrap();

		app.window = Some(window_handle as i32);
		app.init_draw_data();

		let app = INSTANCE.get().unwrap();

		let draw_data = app.draw_data.as_ref().unwrap();

		let hwnd = show();

		for component in app.components.values() {
			component
				.draw(draw_data, DeviceContext::new(hwnd).unwrap())
				.expect("Failed to draw datetime")
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

pub fn hide() {
	unsafe {
		let hwnd = INSTANCE.get().unwrap().window.unwrap() as HWND; // Need to eager evaluate else there is a deadlock
		ShowWindow(hwnd, SW_HIDE);
	}
}

pub fn show() -> HWND {
	unsafe {
		let hwnd = INSTANCE.get().unwrap().window.unwrap() as HWND; // Need to eager evaluate else there is a deadlock
		ShowWindow(hwnd, SW_SHOW);
		hwnd
	}
}
