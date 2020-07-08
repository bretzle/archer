use crate::{common::Rect, hotkey::HotkeyType, window::Window};
use std::error::Error;

pub type Result<T = (), E = Box<dyn Error>> = std::result::Result<T, E>;

pub enum Message {
	PreviewWindow(Window),
	GridWindow(Window),
	HighlightZone(Rect),
	HotkeyPressed(HotkeyType),
	TrackMouse(Window),
	ActiveWindowChange(Window),
	ProfileChange(&'static str),
	MonitorChange,
	MouseLeft,
	InitializeWindows,
	CloseWindows,
	Exit,
}

#[macro_export]
macro_rules! str_to_wide {
	($str:expr) => {{
		$str.encode_utf16()
			.chain(std::iter::once(0))
			.collect::<Vec<_>>()
		}};
}
