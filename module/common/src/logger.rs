use yansi::Paint;
use chrono::Local;
use log::{self, LevelFilter};
use env_logger::Builder;
use std::env;
use std::io::Write;


pub fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        let mut style = formatter.style();
        style.set_bold(true);

        let tar = Paint::blue("Json tools").bold();

        match record.level() {
            log::Level::Info => writeln!(
                formatter,
                "{} {} ({}): {}",
                tar,
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                Paint::blue("Info").bold(),
                Paint::blue(record.args()).wrap()
            ),
            log::Level::Trace => writeln!(
                formatter,
                "{} {} ({}): {}",
                tar,
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                Paint::magenta("Trace").bold(),
                Paint::magenta(record.args()).wrap()
            ),
            log::Level::Error => writeln!(
                formatter,
                "{} {} ({}): {}",
                tar,
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                Paint::red("Error").bold(),
                Paint::red(record.args()).wrap()
            ),
            log::Level::Warn => writeln!(
                formatter,
                "{} {} ({}): {}",
                tar,
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                Paint::yellow("Warning").bold(),
                Paint::yellow(record.args()).wrap()
            ),
            log::Level::Debug => writeln!(
                formatter,
                "{} {} ({}): {}",
                tar,
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                Paint::blue("Debug").bold(),
                Paint::blue(record.args()).wrap()
            ),
        }
    });

    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse_filters(&var);
    } else {
        // if no RUST_LOG provided, default to logging at the Warn level
        builder.filter(None, LevelFilter::Info);
    }

    builder.init();
}
