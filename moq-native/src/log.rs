use clap::Parser;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Clone, Default)]
pub struct Args {
	#[arg(long, short, action = clap::ArgAction::Count)]
	pub verbose: u8,

	#[arg(long, short, action = clap::ArgAction::Count, conflicts_with = "verbose")]
	pub quiet: u8,
}

impl Args {
	pub fn level(&self) -> LevelFilter {
		// Default to INFO, go up or down based on -q or -v counts
		match self.verbose {
			0 => match self.quiet {
				0 => LevelFilter::INFO,
				1 => LevelFilter::ERROR,
				_ => LevelFilter::OFF,
			},
			1 => LevelFilter::DEBUG,
			_ => LevelFilter::TRACE,
		}
	}

	pub fn init(&self) {
		let filter = EnvFilter::new(self.level().to_string()).add_directive("h2=warn".parse().unwrap());

		let logger = tracing_subscriber::FmtSubscriber::builder()
			.with_writer(std::io::stderr)
			.with_env_filter(filter)
			.finish();

		tracing::subscriber::set_global_default(logger).unwrap();
	}
}