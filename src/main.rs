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
    let top = pid::get_docker_top(&args.container)?;
    let mut output: Vec<String> = vec![format!("{:>8} {:>13} {}", "HOST_PID", "CONTAINER_PID", "COMMAND")];
    for entry in top {
        let pid = pid::map_pid(entry.pid)?;
        output.push(format!("{:>8} {:>13} {}", entry.pid, pid, entry.command));
    }
    println!("{}", output.join("\n"));
    Ok(())
}
