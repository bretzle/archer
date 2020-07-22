use crate::{config::Config, INSTANCE};
use crossbeam_channel::select;
use std::thread;
use winsapi::{EventChannel, GlobalHotkeySet, Key, Modifier};

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

	fn register_keybinds(&'static self) {
		let hotkeys = GlobalHotkeySet::new()
			.add_global_hotkey(Event::Quick, Modifier::Ctrl + Modifier::Alt + Key::Q)
			.add_global_hotkey(Event::Main, Modifier::Ctrl + Modifier::Alt + Key::S);

		self.channel.listen_for_hotkeys(hotkeys)
	}

	fn handle_event(&'static self, msg: Event) {
		println!("{:?}", msg);
	}
}

#[derive(Debug, Copy, Clone)]
pub enum Event {
	Main,
	Quick,
}
