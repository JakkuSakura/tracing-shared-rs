use tracing::dispatcher;

#[repr(C)]
pub struct SharedLogger {
    dispatch: tracing::Dispatch,
    #[cfg(feature = "log")]
    logger: &'static dyn log::Log,
}

pub fn build_shared_logger() -> SharedLogger {
    SharedLogger {
        dispatch: dispatcher::get_default(|dispatch| dispatch.clone()),
        #[cfg(feature = "log")]
        logger: log::logger(),
    }
}

#[no_mangle]
pub extern "Rust" fn setup_shared_logger(logger: SharedLogger) {
    dispatcher::set_global_default(logger.dispatch).unwrap();
    #[cfg(feature = "log")]
    log::set_logger(logger.logger).unwrap();
    #[cfg(feature = "log")]
    log::set_max_level(log::LevelFilter::Trace);
}
