#[macro_use]
extern crate log;

mod logging;

fn main() {
	logging::setup();

	error!("hi");
	warn!("hi");
	info!("hi");
	debug!("hi");
	trace!("hi");
}
