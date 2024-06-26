use example_lib::exports::{FnRun, FnSetupLogger, FnSetupTokio};
use std::time::Duration;
use std::{io::Write, path::PathBuf};
use tracing_shared::SharedLogger;

fn main() {
    // build example-lib
    let cdylib = build_dylib();

    // set tracing logger
    tracing_subscriber::FmtSubscriber::builder()
        .without_time()
        .init();

    // log in the normal program
    println!("program println!");
    tracing::info!("program tracing::info!");
    // log was supported in tracing_subscriber
    #[cfg(feature = "log")]
    log::info!("program log::info!");
    // test if tokio task spawns properly
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            let logger = SharedLogger::new();
            // log in the rust dylib
            run_dylib(&logger);
            // log in the cdylib, see `example-lib/src/lib.rs`
            run_cdylib(cdylib, &logger);
            // wait for spawned tasks to finish
            tokio::time::sleep(Duration::from_secs(1)).await
        })
}

fn run_dylib(logger: &SharedLogger) {
    example_lib::setup_shared_logger_ref(&logger);
    let _guard = example_lib::setup_shared_tokio_ref(&logger);
    example_lib::run("dylib");
    println!("drop guard");
    drop(_guard)
}
fn run_cdylib(dylib: PathBuf, logger: &SharedLogger) {
    let dylib = unsafe { libloading::Library::new(dylib) }.expect("error loading dylib");
    let setup_logger: FnSetupLogger = unsafe { *dylib.get(b"setup_shared_logger_ref").unwrap() };
    setup_logger(&logger);
    let setup_tokio: FnSetupTokio = unsafe { *dylib.get(b"setup_shared_tokio_ref").unwrap() };
    let _guard = setup_tokio(&logger);
    let run: FnRun = unsafe { *dylib.get(b"run").unwrap() };
    run("cdylib");
    println!("drop guard");
    drop(_guard)
}
fn build_dylib() -> PathBuf {
    print!("building `example-lib`...");
    std::io::stdout().flush().unwrap();

    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build")
        .arg("--manifest-path")
        .arg("examples/example-lib/Cargo.toml")
        .arg("--message-format")
        .arg("json");

    #[cfg(feature = "log")]
    cmd.arg("--features=log");

    let output = cmd.output().unwrap();
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("{error}");
        panic!("dylib build failed");
    }

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines().rev() {
        if line.starts_with(r#"{"reason":"compiler-artifact""#) {
            let files_start = r#""filenames":[""#;
            let i = line.find(files_start).unwrap();
            let line = &line[i + files_start.len()..];
            let i = line.find('"').unwrap();
            let dylib = &line[..i];

            println!(" done");

            return PathBuf::from(dylib);
        }
    }
    panic!("failed to find get dylib output");
}
