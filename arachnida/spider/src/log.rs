use std::fs::OpenOptions;

pub use ::log::*;
use tracing_subscriber::{
    EnvFilter, Layer, Registry,
    fmt::{self, time::FormatTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

struct CustomTimer;

impl FormatTime for CustomTimer {
    fn format_time(&self, w: &mut fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = chrono::Local::now();
        write!(w, "[{}]", now.format("%d/%m/%y %H:%M:%S"))
    }
}

pub fn init_logger() {
    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("spider.log")
        .expect("Cannot create log file");

    let filter = EnvFilter::new("spider=info");
    let file_filter = EnvFilter::new("spider=info");

    Registry::default()
        .with(
            fmt::layer()
                .compact()
                .with_ansi(true)
                .with_timer(CustomTimer)
                .with_filter(filter),
        )
        .with(
            fmt::layer()
                .with_ansi(false)
                .with_writer(log_file)
                .with_timer(CustomTimer)
                .with_filter(file_filter),
        )
        .init();
}
