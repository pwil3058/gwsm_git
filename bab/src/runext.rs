// Copyright 2019 Peter Williams <pwil3058@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::convert::From;
use std::io;
use std::process::Command;
use std::string::FromUtf8Error;

use shlex;

#[derive(Debug)]
pub enum CmdError {
    IOError(io::Error),
    FromUtf8Error(FromUtf8Error),
}

impl From<io::Error> for CmdError {
    fn from(error: io::Error) -> Self {
        CmdError::IOError(error)
    }
}

impl From<FromUtf8Error> for CmdError {
    fn from(error: FromUtf8Error) -> Self {
        CmdError::FromUtf8Error(error)
    }
}

#[derive(Debug)]
pub enum StdType {
    Text(String),
    Binary(Vec<u8>),
}

#[derive(Debug)]
pub struct CmdResult {
    error_code: u64,
    std_out: StdType,
    std_err: String,
}

pub fn run_cmd<S>(cmd: &str, decode_stdout: bool, sanitize_stderr: S) -> Result<CmdResult, CmdError>
where
    S: Fn(&str) -> String,
{
    let cmd_line = shlex::split(cmd).unwrap();
    let output = Command::new(&cmd_line[0]).args(&cmd_line[1..]).output()?;
    let error_code = if output.status.success() { 0 } else { 1 };
    let std_err = sanitize_stderr(&String::from_utf8(output.stderr)?);
    let std_out = if decode_stdout {
        StdType::Text(String::from_utf8(output.stdout)?)
    } else {
        StdType::Binary(output.stdout)
    };
    Ok(CmdResult {
        error_code,
        std_out,
        std_err,
    })
}

pub fn run_get_cmd<S>(
    cmd: &str,
    decode_stdout: bool,
    trim_right: bool,
    sanitize_stderr: S,
) -> Result<CmdResult, CmdError>
where
    S: Fn(&str) -> String,
{
    if trim_right {
        let result = run_cmd(cmd, decode_stdout, sanitize_stderr)?;
        let std_out = match result.std_out {
            StdType::Text(text) => StdType::Text(text.trim_right().to_string()),
            StdType::Binary(data) => StdType::Binary(data),
        };
        Ok(CmdResult {
            error_code: result.error_code,
            std_out: std_out,
            std_err: result.std_err,
        })
    } else {
        run_cmd(cmd, decode_stdout, sanitize_stderr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_cmd_works() {
        println!("{:?}", run_cmd("ls", true, |s| s.to_string()));
        println!(
            "true: {:?}",
            run_get_cmd("ls", true, true, |s| s.to_string())
        );
        println!(
            "false: {:?}",
            run_get_cmd("ls", true, false, |s| s.to_string())
        );
    }
}
