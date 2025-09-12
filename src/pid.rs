use std::{fs::File, io::{self, BufRead, BufReader}, process::Command};

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

pub fn map_pid(pid: usize) -> Result<usize, DCPMError> {
    let path: String = format!("/proc/{pid}/status");
    let status_file: File = File::open(&path).map_err(|e| {
        match e.kind() {
            io::ErrorKind::NotFound => {
                DCPMError::FileIOError(format!("Path not found: {path}"))
            },
            io::ErrorKind::PermissionDenied => {
                DCPMError::FileIOError(format!("Permission denied for: {path}"))
            }
            _ => DCPMError::FileIOError(format!("{e}"))
        }
    })?;
    let reader: BufReader<File> = BufReader::new(status_file);
    let regex = Regex::new(r"(\S+)\s+(\S+)\s+(\S+)\n")?;
    for line in reader.lines() {
        let line: String = line?;
        if line.starts_with("NSpid:") {
            let Some(caps) = regex.captures(&line) else {
                return Err(DCPMError::MapError(format!("Failed to capture container pid in line: {line}")));
            };
            let (_, [_, _, container_pid]) = caps.extract();
            return Ok(usize::from_str_radix(container_pid, 10)?)
        }
    }
    return Err(DCPMError::MapError(format!("NSpid not found for {pid}")));
}