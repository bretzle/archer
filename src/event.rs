use crate::app_bar::RedrawAppBarReason;
use crossbeam_channel::unbounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;

#[derive(Debug)]
pub enum Event {
	// TODO
	// Keybinding(Keybinding),
	// WinEvent(WinEvent),
	RedrawAppBar(RedrawAppBarReason),
	ReloadConfig,
	Exit,
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
