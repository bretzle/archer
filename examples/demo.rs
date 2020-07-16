use appbar::*;
use log::error;

fn main() {
	logging::setup().expect("Failed to setup logging");

	if let Err(e) = run() {
		error!("An error occured {:?}", e);
	}
}

mod logging {

	pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
		let mut builder = fern::Dispatch::new()
			.format(move |out, message, record| {
				out.finish(format_args!(
					"[{:5}][{}] {}",
					record.level(),
					record.target(),
					message
				))
			})
			.level(log::LevelFilter::Debug)
			.chain(std::io::stdout());

		#[cfg(debug_assertions)]
		{
			builder = builder
				.level_for("hyper", log::LevelFilter::Info)
				.level_for("wwm::app_bar", log::LevelFilter::Error);
		}

		builder.apply().unwrap();

		Ok(())
	}
}
