use clap::Parser;
use dcpm::{error::DCPMError, pid};

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
    let top: Vec<(String, String)> = pid::get_docker_top(&args.container)?;
    let mut output: Vec<String> = vec![format!(
        "{:>8} {:>13} {}",
        "HOST_PID", "CONTAINER_PID", "COMMAND"
    )];
    for (host_pid, command) in top {
        let container_pid = pid::map_pid(&host_pid)?;
        output.push(format!("{:>8} {:>13} {}", host_pid, container_pid, command));
    }
    println!("{}", output.join("\n"));
    Ok(())
}
