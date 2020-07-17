use crate::{app_bar::RedrawAppBarReason, util::WinApiError};
use std::fmt::Debug;
use winapi::shared::windef::HWND;

mod clock;

pub use clock::Clock;

pub trait Component: Debug {
	fn setup(&self);
	fn draw(&self, hwnd: HWND) -> Result<(), WinApiError>;
	fn reason(&self) -> RedrawAppBarReason;
}
