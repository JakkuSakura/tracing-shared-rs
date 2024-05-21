use crate::helper::FakeSubscriber;
use std::fmt::{Debug, Formatter};

mod helper;
static DATA: i32 = 0;

#[derive(Clone)]
#[repr(C)]
pub struct SharedLogger {
    data: &'static i32,
    with_log: bool,
    dispatch: tracing::Dispatch,
    tracing_level: tracing::level_filters::LevelFilter,
    #[cfg(feature = "log")]
    logger: &'static dyn log::Log,
    #[cfg(feature = "log")]
    level: log::LevelFilter,
}
impl Debug for SharedLogger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("SharedLogger");

        dbg.field("data", &format_args!("{:p}", self.data))
            .field("with_log", &self.with_log)
            .field("tracing_dispatch", &self.dispatch)
            .field("tracing_level", &self.tracing_level);
        #[cfg(feature = "log")]
        dbg.field("log_logger", &format_args!("{:p}", self.logger))
            .field("log_level", &self.level);
        dbg.finish()
    }
}
impl SharedLogger {
    pub fn new() -> Self {
        SharedLogger {
            data: &DATA,
            with_log: cfg!(feature = "log"),
            dispatch: tracing::dispatcher::get_default(|dispatch| dispatch.clone()),
            tracing_level: tracing::level_filters::LevelFilter::current(),
            #[cfg(feature = "log")]
            logger: log::logger(),
            #[cfg(feature = "log")]
            level: log::max_level(),
        }
    }
    pub fn install(&self) {
        if std::ptr::addr_eq(self.data, &DATA) {
            panic!("SharedLogger can only be installed in dynamically linked modules, don't call it here");
        }
        // set tracing level
        // see FakeSubscriber's doc
        tracing::Dispatch::new(FakeSubscriber {
            level: self.tracing_level,
        });
        tracing::dispatcher::set_global_default(self.dispatch.clone()).unwrap();
        if self.with_log {
            if !cfg!(feature = "log") {
                panic!("SharedLogger was built with log feature, but in the dylib the feature was not enabled")
            }
            #[cfg(feature = "log")]
            {
                log::set_logger(self.logger).unwrap();
                log::set_max_level(self.level);
            }
        }
    }
}
#[deprecated = "use SharedLogger::new() instead"]
pub fn build_shared_logger() -> SharedLogger {
    SharedLogger::new()
}

#[deprecated = "use setup_shared_logger_ref(&logger) instead. this is to prevent FFI boundary error"]
pub fn setup_shared_logger(logger: SharedLogger) {
    logger.install()
}
/// helper function to install the SharedLogger
/// public re-export to make it globally accessible
/// Usage:
/// ```rust,no_run
///   pub use tracing_shared::setup_shared_logger_ref;
///   use tracing_shared::SharedLogger;
///   let dylib = "library.so";
///   let dylib = unsafe { libloading::Library::new(dylib) }.expect("error loading dylib")
///   let setup_logger: extern "C" fn(&SharedLogger) =
///         unsafe { *dylib.get(b"setup_shared_logger_ref").unwrap() };
/// ```
#[no_mangle]
pub fn setup_shared_logger_ref(logger: &SharedLogger) {
    logger.install()
}
