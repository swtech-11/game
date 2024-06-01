use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init() {
    // env::set_var("RUST_LOG", "INFO");
    let mut builder = Builder::new();
    builder
        .format(|buf, record| {
            let level = match record.level() {
                log::Level::Error => "\x1b[31mERROR\x1b[0m",
                log::Level::Warn => "\x1b[33mWARN\x1b[0m",
                log::Level::Info => "\x1b[32mINFO\x1b[0m",
                log::Level::Debug => "\x1b[34mDEBUG\x1b[0m",
                log::Level::Trace => "\x1b[35mTRACE\x1b[0m",
            };
            let target = format!("\x1b[4;30m{}\x1b[0m", record.target());
            writeln!(buf, "{} {} {}", level, record.args(), target)
        })
        .filter(None, LevelFilter::Info)
        .init();
}
