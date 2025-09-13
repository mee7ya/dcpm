use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    process::Command,
};

use regex::Regex;

use crate::error::DCPMError;

pub fn get_docker_top(container: &String) -> Result<Vec<(String, String)>, DCPMError> {
    let top = Command::new("docker")
        .args(["top", container, "-o", "pid,cmd"])
        .output()
        .map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => {
                DCPMError::ShellError(String::from("Can't access docker binary"))
            }
            _ => DCPMError::IOError(e),
        })?;
    if !top.status.success() {
        return Err(DCPMError::DockerError(
            String::from_utf8(if !top.stdout.is_empty() {
                top.stdout
            } else {
                top.stderr
            })?
            .trim_end()
            .to_string(),
        ));
    }
    let top = String::from_utf8(top.stdout)?;
    println!("{top}");

    let regex = Regex::new(r"(\d+)\s+(\S.+)\n")?;
    Ok(regex
        .captures_iter(&top)
        .skip(1)
        .map(|x| x.extract())
        .map(|(_, x): (&str, [&str; 2])| (String::from(x[0]), String::from(x[1])))
        .collect())
}

pub fn map_pid(pid: &String) -> Result<usize, DCPMError> {
    let path: String = format!("/proc/{pid}/status");
    let status_file: File = File::open(&path).map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => DCPMError::FileIOError(format!("Path not found: {path}")),
        io::ErrorKind::PermissionDenied => {
            DCPMError::FileIOError(format!("Permission denied for: {path}"))
        }
        _ => DCPMError::IOError(e),
    })?;
    let reader: BufReader<File> = BufReader::new(status_file);
    let regex = Regex::new(r"(\S+)\t(\S+)\t(\S+)")?;
    for line in reader.lines() {
        let line: String = line?;
        if line.starts_with("NSpid:") {
            let Some(caps) = regex.captures(&line) else {
                return Err(DCPMError::MapError(format!(
                    "Failed to capture container pid in line: {line}"
                )));
            };
            let (_, [_, _, container_pid]) = caps.extract();
            return Ok(usize::from_str_radix(container_pid, 10)?);
        }
    }
    return Err(DCPMError::MapError(format!("NSpid not found for {pid}")));
}
