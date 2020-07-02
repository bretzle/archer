use crate::{app_bar, util, Event, CHANNEL};
use lazy_static::lazy_static;
use std::sync::Mutex;
use winapi::{
	shared::{
		minwindef::DWORD,
		ntdef::LONG,
		windef::{HWINEVENTHOOK, HWND},
	},
	um::winuser::{
		DispatchMessageW, PeekMessageW, SetWinEventHook, TranslateMessage, EVENT_MAX, EVENT_MIN,
		EVENT_OBJECT_DESTROY, EVENT_OBJECT_HIDE, EVENT_OBJECT_SHOW, EVENT_SYSTEM_FOREGROUND, MSG,
		OBJID_WINDOW, PM_REMOVE,
	},
};

static mut HOOK: Option<HWINEVENTHOOK> = None;

lazy_static! {
	static ref UNREGISTER: Mutex<bool> = Mutex::new(false);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WinEventType {
	Destroy,
	Hide,
	Show(bool),
	FocusChange,
}

impl WinEventType {
	fn from_u32(v: u32) -> Option<Self> {
		if v == EVENT_OBJECT_DESTROY {
			Some(Self::Destroy)
		} else if v == EVENT_OBJECT_SHOW {
			Some(Self::Show(false))
		} else if v == EVENT_SYSTEM_FOREGROUND {
			Some(Self::FocusChange)
		} else if v == EVENT_OBJECT_HIDE {
			Some(Self::Hide)
		} else {
			None
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct WinEvent {
	pub typ: WinEventType,
	pub hwnd: i32,
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

	if app_bar::WINDOWS
		.lock()
		.unwrap()
		.values()
		.any(|v| *v == window_handle as i32)
	{
		return;
	}

	let win_event_type = match WinEventType::from_u32(event_code) {
		Some(event) => event,
		None => return,
	};

	let event = Event::WinEvent(WinEvent {
		typ: win_event_type,
		hwnd: window_handle as i32,
	});

	CHANNEL.sender.clone().send(event).unwrap();
}

pub fn register() -> Result<(), util::WinApiResultError> {
	std::thread::spawn(|| unsafe {
		let mut msg: MSG = MSG::default();

		debug!("Registering win event hook");

		let hook = util::winapi_ptr_to_result(SetWinEventHook(
			EVENT_MIN,
			EVENT_MAX,
			std::ptr::null_mut(),
			Some(handler),
			0,
			0,
			0,
		))
		.unwrap();

		HOOK = Some(hook);

		loop {
			while PeekMessageW(&mut msg, 0 as HWND, 0, 0, PM_REMOVE) > 0 {
				TranslateMessage(&msg);
				DispatchMessageW(&msg);
			}

			if *UNREGISTER.lock().unwrap() {
				debug!("Win event hook unregistered");
				*UNREGISTER.lock().unwrap() = false;
				break;
			}

			std::thread::sleep(std::time::Duration::from_millis(5));
		}
	});

	Ok(())
}

pub fn unregister() -> Result<(), util::WinApiResultError> {
	debug!("Unregistering win event hook");

	*UNREGISTER.lock().unwrap() = true;

	Ok(())
}
