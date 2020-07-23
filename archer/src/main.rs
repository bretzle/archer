use wtm::TilingManager;

fn main() {
	simple_logger::init().unwrap();

	let tm = TilingManager::create();

	tm.start();
}
