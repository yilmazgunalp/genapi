use std::env;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind};
// use std::path::PathBuf;
use std::process::Command;
// use std::error::Error;

pub fn compile_api(path: &OsString) -> Result<(), RcronError> {
    // get the current dir
    let x = env::current_dir().unwrap().into_os_string();
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
                    return RcronError {
                        msg: String::from("crontab must be installed!"),
                    };
                } else {
                    return RcronError {
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
                Err(RcronError {
                    msg: String::from("crontab command failed with error decribed above this line"),
                })
            }
        }
    }
}


pub fn create_tmp_file() -> Result<(File, OsString),RcronError > {
    // create a temp file
    let tmp_file_path: OsString = OsString::from("/home/yg/ygprojects/genapi/output_api/src/main.rs");
    let mut tmp_file: fs::File =
        fs::File::create(&tmp_file_path).expect("Failed at creating tmp cron file");

   
    Ok((tmp_file, tmp_file_path))
}


#[derive(Debug, Default)]
pub struct RcronError {
    pub msg: String,
}

impl fmt::Display for RcronError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<Error> for RcronError {
    fn from(error: Error) -> Self {
        RcronError {
            msg: error.to_string(),
        }
    }
}
