use crate::Component;
use winapi::shared::windef::HWND;

#[derive(Debug, Default)]
pub struct Clock {}

impl Component for Clock {
	fn setup(&self) {
		todo!()
	}

	fn draw(&self, _hwnd: HWND) {
		println!("Drawing clock");
	}
}
