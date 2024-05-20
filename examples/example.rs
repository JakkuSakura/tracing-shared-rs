use std::{io::Write, path::PathBuf};

fn main() {
    // build example-lib
    let dylib = build_dylib();

    // set tracing logger
    tracing_subscriber::FmtSubscriber::builder()
        .without_time()
        .init();
    // tracing::level_filters::STATIC_MAX_LEVEL
    // set log logger
    #[cfg(feature = "log")]
    simple_logger::SimpleLogger::new()
        .without_timestamps()
        .init()
        .unwrap();

    // log in the normal program
    println!("program println!");
    tracing::info!("program tracing::info!");
    #[cfg(feature = "log")]
    log::info!("program log::info!");

    // log in the dylib, see `example-lib/src/lib.rs`
    run_dylib(dylib);
}

fn run_dylib(dylib: PathBuf) {
    let dylib = unsafe { libloading::Library::new(dylib) }.expect("error loading dylib");
    let run: fn(tracing_shared::SharedLogger) = unsafe { *dylib.get(b"run").unwrap() };

    run(tracing_shared::build_shared_logger());
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
