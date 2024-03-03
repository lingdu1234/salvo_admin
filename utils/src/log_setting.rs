use configs::CFG;
use time::format_description::well_known::Rfc3339;
use tracing::Level;
use tracing_subscriber::fmt::{
    self,
    format::{Compact, Format},
    time::LocalTime,
};

// 设置日志格式
pub fn set_log_format() -> Format<Compact, LocalTime<Rfc3339>> {
    fmt::format()
        .with_level(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_timer(LocalTime::rfc_3339()) // use RFC 3339 timestamps
        .compact()
}

// 设置日志级别
pub fn set_log_level() -> Level {
    match CFG.log.log_level.as_str() {
        "TRACE" => tracing::Level::TRACE,
        "DEBUG" => tracing::Level::DEBUG,
        "INFO" => tracing::Level::INFO,
        "WARN" => tracing::Level::WARN,
        "ERROR" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    }
}
