use std::ffi::OsString;
use std::fmt;

use std::io::{Error, ErrorKind};
use std::process::Command;

pub fn compile_api(path: &OsString) -> Result<(), GenapiError> {
    let output_binary_path: OsString = OsString::from("/home/yg/ygprojects/genapi/output_binary/");
    // build the rocket api at the given path
    let status = Command::new("cargo")
        .current_dir(path)
        .arg("build")
        .arg("--out-dir")
        .arg(output_binary_path)
        .arg("-Z")
        .arg("unstable-options")
        .status()
        .map_err({
            |error| {
                if error.kind() == ErrorKind::NotFound {
                    return GenapiError {
                        msg: String::from("crontab must be installed!"),
                    };
                } else {
                    return GenapiError {
                        msg: String::from(format!("Problem running crontab command: {:?}", error)),
                    };
                }
            }
        });

    match status {
        Err(error) => return Err(error),
        Ok(exitstatus) => {
            if exitstatus.success() {
                println!(" has been added to crontab!",);
                return Ok(());
            } else {
                Err(GenapiError {
                    msg: String::from("crontab command failed with error decribed above this line"),
                })
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct GenapiError {
    pub msg: String,
}

impl fmt::Display for GenapiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<Error> for GenapiError {
    fn from(error: Error) -> Self {
        GenapiError {
            msg: error.to_string(),
        }
    }
}
