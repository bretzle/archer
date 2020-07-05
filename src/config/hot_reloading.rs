use crate::{event::Event, CHANNEL};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;

pub fn start() {
	std::thread::spawn(|| {
		let (tx, rx) = channel();

		let mut watcher = watcher(tx, std::time::Duration::from_millis(10))
			.expect("Failed to spawn file watcher");

		let mut path = dirs::config_dir().expect("Failed to get config dir");

		path.push("wtm");
		path.push("config.yaml");

		// watcher
		// 	.watch(path, RecursiveMode::NonRecursive)
		// 	.expect("Failed to watch config directory");

		watcher
			.watch("../DEFAULT_CONFIG.toml", RecursiveMode::NonRecursive)
			.expect("Failed to watch config directory");

		loop {
			match rx.recv() {
				Ok(ev) => {
					if let DebouncedEvent::Write(_) = ev {
						debug!("detected config change");
						CHANNEL
							.sender
							.clone()
							.send(Event::ReloadConfig)
							.expect("Failed to send ReloadConfig event");
					}
				}
				Err(e) => error!("watch error: {:?}", e),
			}
		}
	});
}
