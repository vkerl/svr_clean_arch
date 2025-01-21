use env_logger::fmt::TimestampPrecision;
use log::LevelFilter;

pub fn init_logger() {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .format_timestamp(Some(TimestampPrecision::Millis))
        .format_timestamp_millis()
        .format(|buf, record| {
            use chrono::Local;
            use std::io::Write;
            let local_time = Local::now();
            writeln!(
                buf,
                "{} {} [{}] {}",
                local_time.format("%Y-%m-%d %H:%M:%S%.3f %z"),
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.args()
            )
        })
        .init();
}
