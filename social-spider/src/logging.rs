use fern::colors::{ColoredLevelConfig, Color};

pub fn apply_dispatch() {
    let colors = ColoredLevelConfig::new()
        .debug(Color::Cyan)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} {}: {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("scraper", log::LevelFilter::Warn)
        .level_for("html5ever", log::LevelFilter::Warn)
        .level_for("selectors", log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .apply().expect("Failed to create logging dispatch (fern)!");
}
