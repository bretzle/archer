use crate::poly_bar::RedrawReason;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::fmt::{self, Debug};
use winapi::um::winuser::{
	EVENT_OBJECT_DESTROY, EVENT_OBJECT_HIDE, EVENT_OBJECT_SHOW, EVENT_SYSTEM_FOREGROUND,
};

#[derive(Debug)]
pub enum Event {
	RedrawAppBar(RedrawReason),
	WinEvent(WinEvent),
	__Nonexhaustive,
}

pub type EventSender = Sender<Event>;
pub type EventReceiver = Receiver<Event>;

pub struct EventChannel {
	pub sender: EventSender,
	pub receiver: EventReceiver,
}

impl Default for EventChannel {
	fn default() -> Self {
		let (sender, receiver) = unbounded();

		Self { sender, receiver }
	}
}

impl fmt::Debug for EventChannel {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("[..]")
	}
}

#[derive(Debug, Copy, Clone)]
pub enum WinEventType {
	Destroy,
	Hide,
	Show,
	FocusChange,
}

#[derive(Debug, Copy, Clone)]
pub struct WinEvent {
	pub typ: WinEventType,
	pub hwnd: i32,
}

impl WinEvent {
	pub fn from_code(code: u32, hwnd: i32) -> Option<Self> {
		if code == EVENT_OBJECT_DESTROY {
			Some(Self {
				typ: WinEventType::Destroy,
				hwnd,
			})
		} else if code == EVENT_OBJECT_SHOW {
			Some(Self {
				typ: WinEventType::Show,
				hwnd,
			})
		} else if code == EVENT_SYSTEM_FOREGROUND {
			Some(Self {
				typ: WinEventType::FocusChange,
				hwnd,
			})
		} else if code == EVENT_OBJECT_HIDE {
			Some(Self {
				typ: WinEventType::Hide,
				hwnd,
			})
		} else {
			None
		}
	}
}
