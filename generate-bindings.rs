use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

macro_rules! error {
    ($cause:expr, $fmt:literal $(, $arg:expr)*) => {
        GenError::with_cause($cause, format!($fmt $(, $arg)*))
    };

    ($fmt:literal $(, $arg:expr)*) => {
        GenError::new(format!($fmt $(, $arg)*))
    };
}

pub fn main() {
    match generate_bindings() {
        Ok(_) => println!("done"),
        Err(err) => {
            eprintln!("error: {}", err);

            let mut source = err.source();
            while let Some(cause) = source {
                eprintln!("\tcaused by: {}", cause);
                source = cause.source();
            }

            std::process::exit(1);
        }
    };
}

fn generate_bindings() -> Result<(), GenError> {
    let os_name = std::env::consts::OS;
    let os_arch = std::env::consts::ARCH;
    let subdir = format!("{}-{}", os_name, os_arch);
    println!("generating bindings for `{}`...", subdir);

    let output_dir = Path::new("miniaudio-sys/bindings").join(subdir);
    fs::create_dir_all(&output_dir)
        .map_err(|e| error!(e, "failed to create directory `{}`", output_dir.display()))?;

    println!("generating `bindings.rs`...");
    let status = base_command()
        .arg("-o")
        .arg(output_dir.join("bindings.rs"))
        .arg("miniaudio-sys/bindings.h")
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| error!(e, "failed to run bindgen command for bindings.rs"))?
        .status;

    if !status.success() {
        return Err(error!("bindgen exited with error status {}", status));
    }

    println!("generating `bindings-with-vorbis.rs`...");
    let status = base_command()
        .arg("-o")
        .arg(output_dir.join("bindings-with-vorbis.rs"))
        .arg("miniaudio-sys/bindings-with-vorbis.h")
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| error!(e, "failed to run bindgen command for bindings.rs"))?
        .status;

    if !status.success() {
        return Err(error!("bindgen exited with error status {}", status));
    }

    Ok(())
}

fn base_command() -> Command {
    let mut cmd = Command::new("bindgen");
    cmd.args(&[
        "--verbose",
        "--no-layout-tests",
        "--use-core",
        "--size_t-is-usize",
        "--impl-debug",
        "--no-prepend-enum-name",
    ]);
    cmd.args(&["--ctypes-prefix", "libc"]);
    cmd.args(&["--rust-target", "1.36"]);
    cmd.args(&["--whitelist-type", "ma_.*"]);
    cmd.args(&["--whitelist-function", "ma_.*"]);
    cmd.args(&["--whitelist-var", "(ma|MA)_.*"]);
    cmd
}

#[derive(Debug)]
struct GenError {
    message: String,
    cause: Option<Box<dyn Error>>,
}

impl GenError {
    fn new<S>(message: S) -> GenError
    where
        S: Into<String>,
    {
        GenError {
            message: message.into(),
            cause: None,
        }
    }

    fn with_cause<S, E>(cause: E, message: S) -> GenError
    where
        S: Into<String>,
        E: Error + 'static,
    {
        GenError {
            message: message.into(),
            cause: Some(Box::new(cause)),
        }
    }
}

impl std::fmt::Display for GenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for GenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_ref().map(|e| e.as_ref())
    }
}
