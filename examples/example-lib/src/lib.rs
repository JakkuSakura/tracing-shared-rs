/// can simply re-export if linked as a cdylib, i.e. .so/.dll/.dylib
///
/// or linked as a dylib. it will be linked automatically on startup
pub use tracing_shared::setup_shared_logger_ref;

#[no_mangle]
pub fn run(src: &str) {
    println!(
        "{} feature = log is enabled: {}",
        src,
        cfg!(feature = "log")
    );
    println!("{} println!", src);
    tracing::info!("{} tracing::info!", src);
    #[cfg(feature = "log")]
    log::info!("{} log::info!", src);
}

pub mod exports {
    use tracing_shared::SharedLogger;
    pub type FnSetupLogger = fn(&SharedLogger);
    pub type FnRun = fn(&str);
}
