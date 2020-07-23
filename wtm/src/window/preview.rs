use crate::{Event, INSTANCE};
use crossbeam_channel::{select, Receiver};
use std::{mem, ptr, thread, time::Duration};
use winapi::{
	shared::{
		minwindef::{LPARAM, LRESULT, UINT, WPARAM},
		windef::HWND,
	},
	um::{
		libloaderapi::GetModuleHandleW,
		wingdi::{CreateSolidBrush, RGB},
		winuser::{
			CreateWindowExW, DefWindowProcW, DispatchMessageW, PeekMessageW, RegisterClassExW,
			SetLayeredWindowAttributes, TranslateMessage, LWA_ALPHA, WNDCLASSEXW, WS_EX_LAYERED,
			WS_EX_NOACTIVATE, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP, WS_SYSMENU, WS_VISIBLE,
		},
	},
};
use winsapi::{str_to_wide, Window};

/// Draw's a blue preview over the highlighted part of the grid
pub fn spawn_preview_window(close_msg: Receiver<()>) {
	thread::spawn(move || unsafe {
		let h_instance = GetModuleHandleW(ptr::null());

		let class_name = str_to_wide!("Wtm Zone Preview");

		let mut class = mem::zeroed::<WNDCLASSEXW>();
		class.cbSize = mem::size_of::<WNDCLASSEXW>() as u32;
		class.lpfnWndProc = Some(callback);
		class.hInstance = h_instance;
		class.lpszClassName = class_name.as_ptr();
		class.hbrBackground = CreateSolidBrush(RGB(0, 77, 128));

		RegisterClassExW(&class);

		let hwnd = CreateWindowExW(
			WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_NOACTIVATE,
			class_name.as_ptr(),
			ptr::null(),
			WS_POPUP | WS_VISIBLE | WS_SYSMENU,
			0,
			0,
			0,
			0,
			ptr::null_mut(),
			ptr::null_mut(),
			h_instance,
			ptr::null_mut(),
		);

		SetLayeredWindowAttributes(hwnd, 0, 107, LWA_ALPHA);

		let _ = INSTANCE
			.get()
			.unwrap()
			.channel
			.sender
			.clone()
			.send(Event::PreviewWindow(Window(hwnd)));

		let mut msg = mem::zeroed();
		loop {
			if PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, 1) > 0 {
				TranslateMessage(&msg);
				DispatchMessageW(&msg);
			};

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
	hwnd: HWND,
	msg: UINT,
	wparam: WPARAM,
	lparam: LPARAM,
) -> LRESULT {
	DefWindowProcW(hwnd, msg, wparam, lparam)
}
