// logger.rs
//
// Uses swc_core::common::errors::HANDLER
// Works under Turbopack — tracing does NOT work under Turbopack.
//
// NOTE: h.warn() and h.err() emit immediately — no .emit() needed.
//
// HOW TO USE:
//   log_info!("visit_mut_module", "scanning imports");
//   log_debug!("visit_mut_module", "found index = {}", i);
//   log_warn!("visit_mut_module", "import not found");
//   log_error!("visit_mut_module", "something went wrong");

use swc_core::common::errors::HANDLER;

pub struct Logger;

impl Logger {

    /// INFO — general progress
    /// log_info!("visit_mut_module", "scanning imports")
    /// → warning: [swc-plugin] INFO  visit_mut_module: scanning imports
    pub fn info(location: &str, message: &str) {
        HANDLER.with(|h| {
            h.warn(&format!("[swc-plugin] INFO  {}: {}", location, message));
        });
    }

    /// DEBUG — variable values, indices, counts
    /// log_debug!("visit_mut_module", "found index 2")
    /// → warning: [swc-plugin] DEBUG visit_mut_module: found index 2
    pub fn debug(location: &str, message: &str) {
        HANDLER.with(|h| {
            h.warn(&format!("[swc-plugin] DEBUG {}: {}", location, message));
        });
    }

    /// WARN — unexpected but plugin still continues
    /// log_warn!("visit_mut_function", "no body found")
    /// → warning: [swc-plugin] WARN  visit_mut_function: no body found
    pub fn warn(location: &str, message: &str) {
        HANDLER.with(|h| {
            h.warn(&format!("[swc-plugin] WARN  {}: {}", location, message));
        });
    }

    /// ERROR — something went wrong, transform likely failed
    /// log_error!("process_program", "config missing")
    /// → error: [swc-plugin] ERROR process_program: config missing
    pub fn error(location: &str, message: &str) {
        HANDLER.with(|h| {
            h.err(&format!("[swc-plugin] ERROR {}: {}", location, message));
        });
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MACROS — allows format!() style string interpolation like:
//   log_info!("location", "value is {}", my_var)
//   log_debug!("location", "index = {}, name = {}", i, name)
// ─────────────────────────────────────────────────────────────────────────────

#[macro_export]
macro_rules! log_info {
    ($location:expr, $fmt:expr) => {
        crate::logger::Logger::info($location, $fmt)
    };
    ($location:expr, $fmt:expr, $($arg:tt)*) => {
        crate::logger::Logger::info($location, &format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($location:expr, $fmt:expr) => {
        crate::logger::Logger::debug($location, $fmt)
    };
    ($location:expr, $fmt:expr, $($arg:tt)*) => {
        crate::logger::Logger::debug($location, &format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($location:expr, $fmt:expr) => {
        crate::logger::Logger::warn($location, $fmt)
    };
    ($location:expr, $fmt:expr, $($arg:tt)*) => {
        crate::logger::Logger::warn($location, &format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($location:expr, $fmt:expr) => {
        crate::logger::Logger::error($location, $fmt)
    };
    ($location:expr, $fmt:expr, $($arg:tt)*) => {
        crate::logger::Logger::error($location, &format!($fmt, $($arg)*))
    };
}