use once_cell::sync::OnceCell;
use winsapi::*;

static mut CHANNEL: OnceCell<EventChannel<Event>> = OnceCell::new();

fn main() {
	let channel = unsafe { CHANNEL.get_or_init(EventChannel::default) };

	let a =
		GlobalHotkeySet::new().add_global_hotkey(Event::A, Modifier::Ctrl + Modifier::Alt + Key::Q);
	let b =
		GlobalHotkeySet::new().add_global_hotkey(Event::B, Modifier::Ctrl + Modifier::Alt + Key::S);

	channel.listen_for_hotkeys(a);
	channel.listen_for_hotkeys(b);

	let receiver = channel.receiver.clone();

	loop {
		crossbeam_channel::select! {
			recv(receiver) -> msg => {
				println!("Received event: {:?}", msg.unwrap());
			}
		}
	}
}

#[derive(Copy, Clone, Debug)]
enum Event {
	A,
	B,
}
