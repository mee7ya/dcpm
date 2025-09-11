use std::fmt::Display;

use crate::error::DCPMError;

#[derive(Debug)]
pub struct Top {
    pub pid: usize,
    pub command: String,
}

impl TryFrom<&[&str; 2]> for Top {
    type Error = DCPMError;

    fn try_from(value: &[&str; 2]) -> Result<Self, Self::Error> {
        let pid = match usize::from_str_radix(value[0], 10) {
            Ok(result) => result,
            Err(_) => {
                return Err(Self::Error::ParseIntError(format!(
                    "Value \"{}\" can't be converted to uinteger",
                    value[0]
                )));
            }
        };
        let command = String::from(value[1]);
        Ok(Top { pid, command })
    }
}

impl Display for Top {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Top(pid: {}, command: {})", self.pid, self.command)
    }
}
