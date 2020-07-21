use crate::{
	components::Component,
	config::Config,
	display::Display,
	event::{Event, EventChannel},
	util, INSTANCE,
};
use crossbeam_channel::select;
use log::{debug, info};
use std::{collections::HashMap, ptr, thread, time::Duration};
use system::*;
use winapi::{
	shared::{
		minwindef::HINSTANCE,
		windef::{HBRUSH, HWND},
	},
	um::{
		libloaderapi::GetModuleHandleA,
		wingdi::CreateSolidBrush,
		winuser::{
			CreateWindowExA, DispatchMessageW, GetMessageW, PeekMessageW, RegisterClassA,
			SendMessageA, SetWinEventHook, ShowWindow, TranslateMessage, EVENT_MAX, EVENT_MIN, MSG,
			PM_REMOVE, SW_HIDE, SW_SHOW, WM_PAINT, WNDCLASSA,
		},
	},
};
use winsapi::{DeviceContext, Font, PtrExt, WinApiError, WinApiResult};

mod system;

pub type RedrawReason = String;

pub fn redraw(reason: RedrawReason) -> WinApiResult<()> {
	let ret = unsafe {
		let app = INSTANCE.get_mut().unwrap();
		app.redraw_reason = reason;
		let hwnd = app.window.unwrap();
		SendMessageA(hwnd as HWND, WM_PAINT, 0, 0) as i32
	};

	if ret == 0 {
		Ok(())
	} else {
		Err(WinApiError::Err(ret))
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
			Some(win_event_handler),
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

#[derive(Debug, Default)]
pub struct AppBar {
	display: Display,
	config: Config,
	window: Option<i32>,
	font: Font,
	redraw_reason: RedrawReason,
	components: HashMap<RedrawReason, Box<dyn Component>>,
	channel: EventChannel,
	draw_data: Option<DrawData>,
}

impl AppBar {
	pub fn create() -> &'static mut Self {
		unsafe {
			match INSTANCE.get_mut() {
				Some(instance) => instance,
				None => {
					INSTANCE.set(AppBar::default()).unwrap();
					INSTANCE.get_mut().unwrap()
				}
			}
		}
	}

	pub fn with_component(&'static mut self, component: Box<dyn Component>) -> &'static mut Self {
		if self
			.components
			.insert(component.reason(), component)
			.is_some()
		{
			panic!("Two components can not have the same reason");
		}
		self
	}

	pub fn start(&'static self) {
		thread::spawn(move || {
			let receiver = self.channel.receiver.clone();

			create();

			loop {
				select! {
					recv(receiver) -> msg => {
						Self::handle_event(msg.unwrap());
					}
				}
			}
		});
	}

	fn handle_event(msg: Event) {
		match msg {
			Event::RedrawAppBar(reason) => redraw(reason).unwrap(),
			Event::WinEvent(_) => {
				if util::is_fullscreen() {
					hide();
				} else {
					show();
				}
			}
			_ => {}
		}
	}

	fn init_draw_data(&'static mut self) {
		self.draw_data = Some(DrawData {
			display: &self.display,
			bg_color: &self.config.bg_color,
			font: &self.font,
		})
	}
}

#[derive(Debug)]
pub struct DrawData {
	pub display: &'static Display,
	pub bg_color: &'static i32,
	pub font: &'static Font,
}
