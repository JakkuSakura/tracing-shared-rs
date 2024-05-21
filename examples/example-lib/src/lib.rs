pub use tracing_shared::setup_shared_logger_ref;

#[no_mangle]
pub fn run() {
    println!("dylib println!");
    tracing::info!("dylib tracing::info!");
    #[cfg(feature = "log")]
    log::info!("dylib log::info!");
}
