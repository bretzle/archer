use crate::GlobalHotkeySet;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::fmt;

pub type EventSender<T> = Sender<T>;
pub type EventReceiver<T> = Receiver<T>;

pub struct EventChannel<T> {
	pub sender: EventSender<T>,
	pub receiver: EventReceiver<T>,
}

impl<T> Default for EventChannel<T> {
	fn default() -> Self {
		let (sender, receiver) = unbounded();

		Self { sender, receiver }
	}
}

impl<T> fmt::Debug for EventChannel<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("[..]")
	}
}

impl<T> EventChannel<T>
where
	T: 'static + Copy + Send + Sync,
{
	pub fn listen_for_hotkeys(&'static self, hotkeys: GlobalHotkeySet<T>) {
		std::thread::spawn(move || {
			for event in hotkeys.listen_for_hotkeys().unwrap() {
				self.sender.send(event.unwrap()).unwrap();
			}
		});
	}
}
