use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
	pub static ref X: Mutex<i32> = Mutex::new(0);
	pub static ref Y: Mutex<i32> = Mutex::new(0);
	pub static ref WINDOW: Mutex<i32> = Mutex::new(0);
	pub static ref HEIGHT: Mutex<i32> = Mutex::new(0);
	pub static ref WIDTH: Mutex<i32> = Mutex::new(0);
}

pub fn init() {
	todo!()
}
