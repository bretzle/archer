use crate::{app_bar::RedrawReason, util::WinApiError};
use std::{fmt::Debug, time::Duration};
use winapi::shared::windef::HWND;

mod clock;
mod date;

pub use clock::Clock;
pub use date::Date;

pub trait Component: Debug + Send + Sync {
	fn setup(&self);
	fn interval(&self) -> Duration;
	fn draw(&self, hwnd: HWND) -> Result<(), WinApiError>;
	fn reason(&self) -> RedrawReason;
}
