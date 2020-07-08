#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
	if let Err(e) = wtm::run() {
		log::error!("{:?}", e);
	}
}
