pub use ::log::*;
use tracing_subscriber::{
    Registry,
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
    Registry::default()
        .with(
            fmt::layer()
                .compact()
                .with_file(false)
                .with_ansi(true)
                .with_timer(CustomTimer),
        )
        .init();
}
