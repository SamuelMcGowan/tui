use log::LevelFilter;

pub const LOG_FILE: &str = "logs/log";

pub fn init_logger() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}, {}] {}",
                record.level(),
                record.target(),
                message,
            ))
        })
        .level(LevelFilter::Trace)
        .chain(fern::log_file(LOG_FILE).expect("failed to open log file"))
        .apply()
        .expect("failed to initialize logger");
}
