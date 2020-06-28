#[cfg(feature = "color")]
use fern::colors::{Color, ColoredLevelConfig};

pub fn setup() {
	fern::Dispatch::new()
		.format(|out, message, record| {
			#[cfg(feature = "color")]
			let colors = ColoredLevelConfig::new()
				.trace(Color::BrightWhite)
				.debug(Color::Magenta)
				.info(Color::Cyan)
				.warn(Color::Yellow)
				.error(Color::Red);

			#[cfg(feature = "color")]
			let level = colors.color(record.level());
			#[cfg(not(feature = "color"))]
			let level = record.level();

			out.finish(format_args!(
				"[{}][{}][{:<5}] {}",
				chrono::Local::now().format("%H:%M:%S"),
				record.target(),
				level,
				message
			))
		})
		.level(log::LevelFilter::Trace)
		.chain(std::io::stdout())
		.apply()
		.unwrap();
}
