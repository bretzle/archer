use crate::{app_bar::RedrawReason, util::WinApiError, event::EventSender};
use std::fmt::Debug;
use winapi::shared::windef::HWND;

mod clock;

pub use clock::Clock;

pub trait Component: Debug + Send + Sync {
	fn setup(&self, window: &'static i32, channel: EventSender);
	fn draw(&self, hwnd: HWND) -> Result<(), WinApiError>;
	fn reason(&self) -> RedrawReason;
}
