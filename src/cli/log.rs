use clap::ValueEnum;
use fern::colors::Color;
use fern::colors::ColoredLevelConfig;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum LogLevel {
	INFO,
	WARN,
	DEBUG,
	TRACE,
	FULL,
}

impl Default for LogLevel {
	fn default() -> Self {
		LogLevel::INFO
	}
}

pub fn init(verbosity: LogLevel) {
	let levels = ColoredLevelConfig::new()
		.error(Color::Red)
		.warn(Color::Yellow)
		.info(Color::Blue)
		.debug(Color::Magenta)
		.trace(Color::White);

	let mut logger = fern::Dispatch::new();

	logger = logger.format(move |out, message, record| {
		out.finish(format_args!(
			"{b}{time}{r} {l}{kind:<5}{r} {c}{name}{r} {l}{message}{r}",
			l = format_args!("\x1B[{}m", levels.get_color(&record.level()).to_fg_str()),
			b = format_args!("\x1B[{}m", Color::BrightBlack.to_fg_str()),
			c = format_args!("\x1B[{}m", Color::Cyan.to_fg_str()),
			r = "\x1B[0m",
			time = chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
			kind = record.level(),
			name = record.target(),
			message = message,
		))
	});

	logger = match verbosity {
		LogLevel::INFO => logger.level_for("realbase", log::LevelFilter::Info),
		LogLevel::WARN => logger.level_for("realbase", log::LevelFilter::Warn),
		LogLevel::DEBUG => logger.level_for("realbase", log::LevelFilter::Debug),
		_ => logger.level_for("realbase", log::LevelFilter::Trace),
	};

	logger = match verbosity {
		LogLevel::TRACE => logger.level(log::LevelFilter::Trace),
		_ => logger.level(log::LevelFilter::Error),
	};

	logger = logger.chain(std::io::stderr());

	logger.apply().unwrap();
}
