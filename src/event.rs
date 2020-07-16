use crate::app_bar::RedrawAppBarReason;
use crossbeam_channel::{unbounded, Receiver, Sender};

#[derive(Debug)]
pub enum Event {
	RedrawAppBar(RedrawAppBarReason),
	__Nonexhaustive
}

pub type EventSender = Sender<Event>;
pub type EventReceiver = Receiver<Event>;

pub struct EventChannel {
	pub sender: EventSender,
	pub receiver: EventReceiver,
}

impl Default for EventChannel {
	fn default() -> Self {
		let (sender, receiver) = unbounded();

		Self { sender, receiver }
	}
}
