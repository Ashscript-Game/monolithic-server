use fern::colors::{Color, ColoredLevelConfig};
use std::time::SystemTime;

pub fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .debug(Color::Cyan)
        .error(Color::Red);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}{}\x1B[0m",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                colors.color(record.level()),
                record.target(),
                format_args!(
                    "\x1B[{}m",
                    colors.get_color(&record.level()).to_fg_str()
                ),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        //.chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}