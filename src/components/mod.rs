use std::fmt::Debug;
use winapi::shared::windef::HWND;

pub mod clock;

pub trait Component: Debug {
	fn setup(&self);
	fn draw(&self, hwnd: HWND);
}
