use std::sync::{Mutex, OnceLock};

use tracing_appender::non_blocking::WorkerGuard;
#[allow(unused_imports)]
use tracing_subscriber::filter::FilterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, Registry, fmt};

pub use tracing::{
    debug, debug_span, error, error_span, info, info_span, trace, trace_span, warn, warn_span,
};

const WORKER_GUARD_LOCK: OnceLock<Mutex<WorkerGuard>> = OnceLock::new();

pub fn init(level: &str) {
    let level_filter = EnvFilter::try_from(level)
        .or_else(|_| EnvFilter::try_from_default_env())
        .unwrap_or_else(|_| EnvFilter::from("info"));

    // TODO: implement tracing-appender and flush into files here

    let ansi_layer = fmt::layer()
        .with_ansi(true)
        .with_thread_names(false)
        .with_target(level == "trace")
        .with_file(level == "trace")
        .with_line_number(level == "trace")
        .with_filter(level_filter);

    Registry::default().with(ansi_layer).init();
}
