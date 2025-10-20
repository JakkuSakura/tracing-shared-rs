#![doc = include_str!("../README.md")]

use crate::helper::FakeSubscriber;
use std::fmt::{Debug, Formatter};
use tracing::subscriber::{set_global_default, NoSubscriber};

mod helper;
fn get_data() -> u64 {
    set_global_default::<NoSubscriber> as *const () as u64
}
pub const FEATURE_LOG: u16 = (cfg!(feature = "log") as u16) << 0;
fn get_features() -> u16 {
    let mut feature = 0;
    feature |= FEATURE_LOG;
    feature
}
fn check_features(features: u16) {
    let log_enabled = features & (1 << 0);
    if log_enabled ^ FEATURE_LOG != 0 {
        panic!(
            "`feature = log` mismatch: executable = {}, dylib = {}",
            log_enabled != 0,
            FEATURE_LOG != 0
        );
    }
}
#[derive(Clone)]
#[repr(C)]
pub struct SharedLogger {
    data: u64,
    features: u16,
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

        dbg.field("data", &format_args!("{:p}", self.data as *const ()))
            .field("features", &self.features)
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
            data: get_data(),
            features: get_features(),
            dispatch: tracing::dispatcher::get_default(|dispatch| dispatch.clone()),
            tracing_level: tracing::level_filters::LevelFilter::current(),
            #[cfg(feature = "log")]
            logger: log::logger(),
            #[cfg(feature = "log")]
            level: log::max_level(),
        }
    }
    #[inline(never)]
    pub fn install(&self) {
        if std::ptr::addr_eq(&self.data, get_data() as *const ()) {
            panic!("SharedLogger can only be installed in dynamically linked modules, don't call it here");
        }
        check_features(self.features);
        // set tracing level
        // see FakeSubscriber's doc
        tracing::Dispatch::new(FakeSubscriber {
            level: self.tracing_level,
        });
        // it fails for dylib, but it's not necessary to setup for tracing
        let _ = tracing::dispatcher::set_global_default(self.dispatch.clone());

        #[cfg(feature = "log")]
        {
            // it fails for dylib, but it's not necessary to setup for tracing
            let _ = log::set_logger(self.logger);
            log::set_max_level(self.level);
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
///   let dylib = unsafe { libloading::Library::new(dylib) }.expect("error loading dylib");
///   let setup_logger: extern "Rust" fn(&SharedLogger) =
///         unsafe { *dylib.get(b"setup_shared_logger_ref").unwrap() };
/// ```
#[no_mangle]
#[inline(never)]
pub fn setup_shared_logger_ref(logger: &SharedLogger) {
    logger.install()
}
