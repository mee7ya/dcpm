use std::process::exit;

use clap::Parser;
use dcpm::error::DCPMError;
#[cfg(target_os = "linux")]
use dcpm::linux::get_docker_top;

#[derive(Parser, Debug)]
#[command(name = "Docker Container PID Mapper")]
#[command(version)]
#[command(about = "Provides mapping from host PID namespace to Docker container namespace", long_about = None)]
struct CLI {
    /// Container name/id
    container: String,
}

fn main() -> Result<(), DCPMError> {
    let args: CLI = CLI::parse();
    let _top = match get_docker_top(&args.container) {
        Ok(result) => result,
        Err(e) => {
            match e {
                DCPMError::ParseIntError(message) => {
                    eprintln!("{message}");
                },
                _ => eprint!("{e}")
            }
            exit(1);
        }
    };
    Ok(())
}
