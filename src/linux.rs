use std::process::Command;

use regex::Regex;

use crate::{error::DCPMError, structs::Top};

pub fn get_docker_top(container: &String) -> Result<Vec<Top>, DCPMError> {
    let top = Command::new("docker")
        .args(["top", container, "-o", "pid,comm"])
        .output()?;
    if !top.status.success() {
        return Err(DCPMError::DockerError(String::from_utf8(
            if !top.stdout.is_empty() {
                top.stdout
            } else {
                top.stderr
            },
        )?));
    }
    let top = String::from_utf8(top.stdout)?;

    let regex = Regex::new(r"(\S+)\s+(\S+)\n")?;
    regex
        .captures_iter(&top)
        .skip(1)
        .map(|x| x.extract())
        .map(|(_, x): (&str, [&str; 2])| Top::try_from(&x))
        .collect()
}

pub fn map_pid() {
    todo!();
}
