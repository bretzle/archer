use crate::{event::Event, util::*, AppBar, INSTANCE};
use log::{debug, info};
use std::{ffi::CString, thread};
use winapi::{
	shared::{
		minwindef::{HINSTANCE, LPARAM, LRESULT, UINT, WPARAM},
		windef::{HBRUSH, HDC, HWND, RECT, SIZE},
		windowsx::GET_X_LPARAM,
	},
	um::{
		libloaderapi::GetModuleHandleA,
		wingdi::{
			CreateFontIndirectA, CreateSolidBrush, GetTextExtentPoint32A, SelectObject, SetBkColor,
			SetTextColor, LOGFONTA,
		},
		winuser::{
			BeginPaint, DefWindowProcA, DispatchMessageW, DrawTextA, EndPaint, GetClientRect,
			GetDC, GetMessageW, LoadCursorA, RegisterClassA, SendMessageA, SetCursor, ShowWindow,
			TranslateMessage, DT_CENTER, DT_SINGLELINE, DT_VCENTER, IDC_ARROW, MSG, PAINTSTRUCT,
			SW_HIDE, SW_SHOW, WM_CLOSE, WM_CREATE, WM_LBUTTONDOWN, WM_PAINT, WM_SETCURSOR,
			WNDCLASSA,
		},
	},
};

#[derive(Copy, Clone, Debug)]
pub enum RedrawAppBarReason {
	Time,
}

impl Default for RedrawAppBarReason {
	fn default() -> Self {
		RedrawAppBarReason::Time
	}
}

unsafe extern "system" fn window_cb(
	hwnd: HWND,
	msg: UINT,
	w_param: WPARAM,
	l_param: LPARAM,
) -> LRESULT {
	if msg == WM_CLOSE {
		AppBar::get_mut().window = 0;
	} else if msg == WM_SETCURSOR {
		// Force a normal cursor. This probably shouldn't be done this way but whatever
		SetCursor(LoadCursorA(std::ptr::null_mut(), IDC_ARROW as *const i8));
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

		match reason {
			RedrawAppBarReason::Time => {
				draw_datetime(hwnd).expect("Failed to draw datetime");
			}
		}

		EndPaint(hwnd, &paint);
	}

	DefWindowProcA(hwnd, msg, w_param, l_param)
}

pub fn redraw(reason: RedrawAppBarReason) {
	unsafe {
		let hwnd = AppBar::get().window as HWND;

		if hwnd == 0 as HWND {
			return;
		}

		AppBar::get_mut().redraw_reason = reason;

		//TODO: handle error
		SendMessageA(hwnd, WM_PAINT, 0, 0);
	}
}

fn draw_workspaces(_hwnd: HWND) {}

pub fn set_font(dc: HDC) {
	unsafe {
		SelectObject(dc, AppBar::get().font as *mut std::ffi::c_void);
	}
}

pub fn load_font() {
	unsafe {
		let config = AppBar::config();
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

		AppBar::get_mut().font = font;
	}
}

pub fn create() {
	info!("Creating appbar");
	let name = "app_bar";
	let config = AppBar::config();

	let height = config.height;
	let display_width = AppBar::get().display.width;

	thread::spawn(|| loop {
		thread::sleep(std::time::Duration::from_millis(950));
		if AppBar::get().window == 0 {
			break;
		}
		AppBar::send_message(Event::RedrawAppBar(RedrawAppBarReason::Time))
			.expect("Failed to send redraw-app-bar event");
	});

	thread::spawn(move || unsafe {
		//TODO: Handle error
		let instance = GetModuleHandleA(std::ptr::null_mut());
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
			std::ptr::null_mut(),
			std::ptr::null_mut(),
			instance as HINSTANCE,
			std::ptr::null_mut(),
		);

		AppBar::get_mut().window = window_handle as i32;

		show();

		let mut msg: MSG = MSG::default();
		while GetMessageW(&mut msg, window_handle, 0, 0) > 0 {
			TranslateMessage(&msg);
			DispatchMessageW(&msg);
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

pub fn show() {
	let hwnd = AppBar::get().window as HWND; // Need to eager evaluate else there is a deadlock

	unsafe {
		ShowWindow(hwnd, SW_SHOW);
	}

	draw_workspaces(hwnd);
	draw_datetime(hwnd).expect("Failed to draw datetime");
}

pub fn draw_datetime(hwnd: HWND) -> Result<(), WinApiError> {
	if !hwnd.is_null() {
		let mut rect = RECT::default();

		unsafe {
			debug!("Getting the rect for the appbar");
			GetClientRect(hwnd, &mut rect).as_result()?;
			let text = format!("{}", chrono::Local::now().format("%T"));
			let text_len = text.len() as i32;
			let c_text = CString::new(text).unwrap();
			let display = INSTANCE.get().unwrap().display;
			let config = AppBar::config();

			debug!("Getting the device context");
			let hdc = GetDC(hwnd).as_result()?;

			set_font(hdc);

			let mut size = SIZE::default();

			GetTextExtentPoint32A(hdc, c_text.as_ptr(), text_len, &mut size).as_result()?;

			rect.left = display.width / 2 - (size.cx / 2) - 10;
			rect.right = display.width / 2 + (size.cx / 2) + 10;

			debug!("Setting the text color");
			//TODO: handle error
			SetTextColor(hdc, 0x00ffffff);

			debug!("Setting the background color");
			SetBkColor(hdc, config.bg_color as u32);

			debug!("Writing the time");
			DrawTextA(
				hdc,
				c_text.as_ptr(),
				text_len,
				&mut rect,
				DT_CENTER | DT_VCENTER | DT_SINGLELINE,
			)
			.as_result()?;

			let text = format!("{}", chrono::Local::now().format("%e %b %Y"));
			let text_len = text.len() as i32;
			let c_text = CString::new(text).unwrap();

			GetTextExtentPoint32A(hdc, c_text.as_ptr(), text_len, &mut size).as_result()?;

			rect.right = display.width - 10;
			rect.left = rect.right - size.cx;

			debug!("Writing the date");
			DrawTextA(
				hdc,
				c_text.as_ptr(),
				text_len,
				&mut rect,
				DT_CENTER | DT_VCENTER | DT_SINGLELINE,
			)
			.as_result()?;
		}
	}

	Ok(())
}
