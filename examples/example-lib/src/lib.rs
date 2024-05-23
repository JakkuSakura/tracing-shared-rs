/// can simply re-export if linked as a cdylib, i.e. .so/.dll/.dylib
///
/// or linked as a dylib. it will be linked automatically on startup
pub use tracing_shared::setup_shared_logger_ref;
pub use tracing_shared::setup_shared_tokio_ref;

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
    #[cfg(feature = "tokio")]
    let src = src.to_string();
    #[cfg(feature = "tokio")]
    tokio::task::spawn(async move { println!("{} println! from tokio task", src) });
}

pub mod exports {
    use tracing_shared::{SharedLogger, TokioEnterGuard};
    pub type FnSetupLogger = fn(&SharedLogger);
    #[cfg(feature = "tokio")]
    pub type FnSetupTokio = fn(&SharedLogger) -> Option<TokioEnterGuard>;
    pub type FnRun = fn(&str);
}
