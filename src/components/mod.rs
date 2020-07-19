use crate::{app_bar::RedrawReason, event::EventSender, util::WinApiError};
use std::fmt::Debug;
use winapi::shared::windef::HWND;

mod clock;
mod date;

pub use clock::Clock;
pub use date::Date;

pub trait Component: Debug + Send + Sync {
	fn setup(&self, window: &'static i32, channel: EventSender);
	fn draw(&self, hwnd: HWND) -> Result<(), WinApiError>;
	fn reason(&self) -> RedrawReason;
}
