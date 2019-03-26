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
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use glob::{Pattern, PatternError};
use serde_json;

use crate::config;

#[derive(Debug)]
pub enum ETError {
    IOError(io::Error),
    GlobError(PatternError),
    JsonError(serde_json::Error),
}

impl fmt::Display for ETError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ETError is here!")
    }
}

impl Error for ETError {
    fn description(&self) -> &str {
        match self {
            ETError::IOError(_) => "I/O Error accessing editor assignment table",
            ETError::GlobError(_) => "Glob Pattern Error accessing editor assignment table",
            ETError::JsonError(_) => "Serde Json Error accessing editor assignment table",
        }

    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ETError::IOError(err) => Some(err),
            ETError::GlobError(err) => Some(err),
            ETError::JsonError(err) => Some(err),
        }
    }
}

impl From<io::Error> for ETError {
    fn from(error: io::Error) -> Self {
        ETError::IOError(error)
    }
}

impl From<PatternError> for ETError {
    fn from(error: PatternError) -> Self {
        ETError::GlobError(error)
    }
}

impl From<serde_json::Error> for ETError {
    fn from(error: serde_json::Error) -> Self {
        ETError::JsonError(error)
    }
}

#[cfg(target_family = "unix")]
const PATH_SEP: char = ':';
#[cfg(target_os = "windows")]
const PATH_SEP: char = ';';

#[cfg(target_family = "unix")]
const DEFAULT_EDITOR: &str = "gedit";
#[cfg(target_os = "windows")]
const DEFAULT_EDITOR: &str = "notepad";

pub fn default_editor() -> String {
    if let Ok(editor) = env::var("VISUAL") {
        editor
    } else {
        DEFAULT_EDITOR.to_string()
    }
}

fn editor_assignment_table_filepath() -> PathBuf {
    let mut pathbuf = config::get_config_dir_path();
    pathbuf.push("editor_assignment_table");
    pathbuf
}

fn read_editor_assignment_table() -> Result<Vec<(String, String)>, ETError> {
    let mut file = File::open(editor_assignment_table_filepath())?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let v: Vec<(String, String)> = serde_json::from_str(&buffer)?;
    Ok(v)
}

fn write_editor_assignment_table(table: &[(String, String)]) -> Result<usize, ETError> {
    let data = serde_json::to_string(table)?;
    let mut file = File::create(editor_assignment_table_filepath())?;
    let nbytes = file.write(data.as_bytes())?;
    Ok(nbytes)
}

pub fn init_editor_assignment_table() {
    if !editor_assignment_table_filepath().is_file() {
        write_editor_assignment_table(&vec![]).expect("failed to initialize editor assignment table");
    }
}

pub fn get_assigned_editor(file_path: &str) -> Result<String, ETError> {
    let editor_assignment_table = read_editor_assignment_table()?;
    for (globs, editor) in editor_assignment_table.iter() {
        for glob in globs.split(PATH_SEP) {
            let pattern = Pattern::new(glob)?;
            if pattern.matches(file_path) {
                return Ok(editor.to_string());
            }
        }
    }
    Ok(default_editor())
}
