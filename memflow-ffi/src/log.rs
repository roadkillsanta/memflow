use log::{Level, LevelFilter};
use memflow::cglue::IntError;
use memflow::error::Error;
use memflow::plugins::Inventory;
use std::num::NonZeroI32;

/// Initialize logging with selected logging level.
#[no_mangle]
pub extern "C" fn log_init(level_filter: LevelFilter) {
    simple_logger::SimpleLogger::new().init().unwrap();
    log::set_max_level(level_filter);
}

/// Logs an error with custom log level.
#[no_mangle]
pub extern "C" fn log_error(level: Level, error: i32) {
    if let Some(error) = NonZeroI32::new(error) {
        log::log!(level, "{}", <Error as IntError>::from_int_err(error));
    }
}

/// Logs an error with debug log level.
#[no_mangle]
pub extern "C" fn debug_error(error: i32) {
    log_error(Level::Debug, error)
}

/// Sets new maximum log level.
///
/// If `inventory` is supplied, the log level is also updated within all plugin instances. However,
/// if it is not supplied, plugins will not have their log levels updated, potentially leading to
/// lower performance, or less logging than expected.
#[no_mangle]
pub extern "C" fn set_max_level(level_filter: LevelFilter, inventory: Option<&Inventory>) {
    log::set_max_level(level_filter);

    if let Some(inventory) = inventory {
        inventory.update_max_log_level();
    }
}
