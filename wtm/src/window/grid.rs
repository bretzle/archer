use crate::{
	str_to_wide,
	util::{get_work_area, Rect},
	window::Window,
	Message, CHANNEL, GRID,
};
use crossbeam_channel::{select, Receiver};
use std::{mem, ptr, thread, time::Duration};
use winapi::{
	shared::{
		minwindef::{HIWORD, LOWORD, LPARAM, LRESULT, UINT, WPARAM},
		windef::HWND,
	},
	um::{
		libloaderapi::GetModuleHandleW,
		wingdi::{CreateSolidBrush, RGB},
		winuser::{
			CreateWindowExW, DefWindowProcW, DispatchMessageW, InvalidateRect, LoadCursorW,
			PeekMessageW, RegisterClassExW, SendMessageW, TranslateMessage, IDC_ARROW, VK_CONTROL,
			VK_DOWN, VK_ESCAPE, VK_F1, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_LEFT, VK_RIGHT,
			VK_SHIFT, VK_UP, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSELEAVE,
			WM_MOUSEMOVE, WM_PAINT, WNDCLASSEXW, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_POPUP,
		},
	},
};

/// Draw's the grid selection window
pub fn spawn_grid_window(close_msg: Receiver<()>) {
	thread::spawn(move || unsafe {
		let h_instance = GetModuleHandleW(ptr::null());

		let class_name = str_to_wide!("Wtm Zone Grid");

		let mut class = mem::zeroed::<WNDCLASSEXW>();
		class.cbSize = mem::size_of::<WNDCLASSEXW>() as u32;
		class.lpfnWndProc = Some(callback);
		class.hInstance = h_instance;
		class.lpszClassName = class_name.as_ptr();
		class.hbrBackground = CreateSolidBrush(RGB(44, 44, 44));
		class.hCursor = LoadCursorW(ptr::null_mut(), IDC_ARROW);

		RegisterClassExW(&class);

		let work_area = get_work_area();
		let dimensions = GRID.get().unwrap().dimensions();

		let hwnd = CreateWindowExW(
			WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
			class_name.as_ptr(),
			ptr::null(),
			WS_POPUP,
			work_area.width / 2 - dimensions.0 as i32 / 2 + work_area.x,
			work_area.height / 2 - dimensions.1 as i32 / 2 + work_area.y,
			dimensions.0 as i32,
			dimensions.1 as i32,
			ptr::null_mut(),
			ptr::null_mut(),
			h_instance,
			ptr::null_mut(),
		);

		let _ = CHANNEL.get().unwrap().0.clone().send(Message::GridWindow(Window(hwnd)));

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
	let sender = CHANNEL.get().unwrap().0.clone();

	let repaint = match msg {
		WM_PAINT => {
			GRID.get().unwrap().draw(Window(hwnd));
			false
		}
		WM_KEYDOWN => match wparam as i32 {
			VK_ESCAPE => {
				let _ = sender.send(Message::CloseWindows);
				false
			}
			VK_CONTROL => {
				GRID.get_mut().unwrap().control_down = true;
				false
			}
			VK_SHIFT => {
				GRID.get_mut().unwrap().shift_down = true;
				false
			}
			VK_RIGHT => {
				if GRID.get().unwrap().control_down {
					GRID.get_mut().unwrap().add_column();
					GRID.get_mut().unwrap().reposition();
				}
				false
			}
			VK_LEFT => {
				if GRID.get().unwrap().control_down {
					GRID.get_mut().unwrap().remove_column();
					GRID.get_mut().unwrap().reposition();
				}
				false
			}
			VK_UP => {
				if GRID.get().unwrap().control_down {
					GRID.get_mut().unwrap().add_row();
					GRID.get_mut().unwrap().reposition();
				}
				false
			}
			VK_DOWN => {
				if GRID.get().unwrap().control_down {
					GRID.get_mut().unwrap().remove_row();
					GRID.get_mut().unwrap().reposition();
				}
				false
			}
			_ => false,
		},
		WM_KEYUP => match wparam as i32 {
			VK_CONTROL => {
				GRID.get_mut().unwrap().control_down = false;
				false
			}
			VK_SHIFT => {
				GRID.get_mut().unwrap().shift_down = false;
				false
			}
			VK_F1 => {
				let _ = sender.send(Message::ProfileChange("Default"));
				false
			}
			VK_F2 => {
				let _ = sender.send(Message::ProfileChange("Profile2"));
				false
			}
			VK_F3 => {
				let _ = sender.send(Message::ProfileChange("Profile3"));
				false
			}
			VK_F4 => {
				let _ = sender.send(Message::ProfileChange("Profile4"));
				false
			}
			VK_F5 => {
				let _ = sender.send(Message::ProfileChange("Profile5"));
				false
			}
			VK_F6 => {
				let _ = sender.send(Message::ProfileChange("Profile6"));
				false
			}
			_ => false,
		},
		WM_MOUSEMOVE => {
			let x = LOWORD(lparam as u32) as i32;
			let y = HIWORD(lparam as u32) as i32;

			let _ = sender.send(Message::TrackMouse(Window(hwnd)));

			if let Some(rect) = GRID.get_mut().unwrap().highlight_tiles((x, y)) {
				let _ = sender.send(Message::HighlightZone(rect));

				true
			} else {
				false
			}
		}
		WM_LBUTTONDOWN => {
			let x = LOWORD(lparam as u32) as i32;
			let y = HIWORD(lparam as u32) as i32;

			let mut grid = GRID.get_mut().unwrap();

			let repaint = grid.select_tile((x, y));

			grid.cursor_down = true;

			repaint
		}
		WM_LBUTTONUP => {
			let mut grid = GRID.get_mut().unwrap();

			let repaint = if let Some(mut rect) = grid.selected_area() {
				if let Some(mut active_window) = grid.active_window {
					if grid.previous_resize != Some((active_window, rect)) {
						active_window.restore();

						rect.adjust_for_border(active_window.transparent_border());

						active_window.set_pos(rect, None);

						grid.previous_resize = Some((active_window, rect));

						if grid.quick_resize {
							let _ = sender.send(Message::CloseWindows);
						}
					}

					grid.unselect_all_tiles();
				}

				true
			} else {
				false
			};

			grid.cursor_down = false;

			repaint
		}
		WM_MOUSELEAVE => {
			GRID.get_mut().unwrap().unhighlight_all_tiles();

			let _ = sender.send(Message::MouseLeft);
			let _ = sender.send(Message::HighlightZone(Rect::zero()));

			true
		}
		_ => false,
	};

	if repaint {
		let dimensions = GRID.get().unwrap().dimensions();
		let rect = Rect {
			x: 0,
			y: 0,
			width: dimensions.0 as i32,
			height: dimensions.1 as i32,
		};

		InvalidateRect(hwnd, &rect.into(), 0);
		SendMessageW(hwnd, WM_PAINT, 0, 0);
	}

	DefWindowProcW(hwnd, msg, wparam, lparam)
}
