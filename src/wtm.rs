use crate::{config::Config, INSTANCE};

#[derive(Debug, Default)]
pub struct WTM {
	config: Config,
}

impl WTM {
	pub fn create() -> &'static mut Self {
		unsafe {
			match INSTANCE.get_mut() {
				Some(instance) => instance,
				None => {
					INSTANCE.set(WTM::default()).unwrap();
					INSTANCE.get_mut().unwrap()
				}
			}
		}
	}

	pub fn start(&'static mut self) {
		loop {}
	}
}
