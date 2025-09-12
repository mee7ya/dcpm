use clap::Parser;
use dcpm::{pid, error::DCPMError};

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
    let top = dcpm::pid::get_docker_top(&args.container)?;
    for entry in top {
        let pid = dcpm::pid::map_pid(entry.pid)?;
    }
    Ok(())
}
