use crate::{config::Config, INSTANCE};
use crossbeam_channel::select;
use std::thread;
use winsapi::EventChannel;

#[derive(Debug, Default)]
pub struct WTM {
	config: Config,
	channel: EventChannel<Event>,
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
		thread::spawn(move || {
			let receiver = self.channel.receiver.clone();

			self.register_keybinds();

			loop {
				select! {
					recv(receiver) -> msg => {
						self.handle_event(msg.unwrap());
					}
				}
			}
		});
	}

	fn register_keybinds(&'static self) {}

	fn handle_event(&'static self, msg: Event) {}
}

pub enum Event {}
