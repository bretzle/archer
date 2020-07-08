use log::Level;

pub fn setup() -> Result<(), log::SetLoggerError> {
	fern::Dispatch::new()
		.format(|out, message, record| {
			out.finish(format_args!(
				"[{}][{:<14}][{:<5}] {}",
				chrono::Local::now().format("%H:%M:%S"),
				record.target(),
				level(record.level()),
				message
			))
		})
		.level(log::LevelFilter::Trace)
		.chain(std::io::stdout())
		.apply()
}

#[cfg(not(debug_assertions))]
fn level(level: Level) -> Level {
	level
}

#[cfg(debug_assertions)]
fn level(level: Level) -> fern::colors::WithFgColor<Level> {
	use fern::colors::{Color, ColoredLevelConfig};

	let colors = ColoredLevelConfig::new()
		.trace(Color::BrightWhite)
		.debug(Color::Magenta)
		.info(Color::Cyan)
		.warn(Color::Yellow)
		.error(Color::Red);

	colors.color(level)
}
