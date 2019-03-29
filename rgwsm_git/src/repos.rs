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

use std::error::Error;
use std::fmt;

use git2;

use pw_gix::sav_state::*;

use pw_pathux::str_path::*;

use crate::submodules;

pub const SAV_NOT_IN_REPO: u64 = SAV_SELN_MASK + 1;
pub const SAV_IN_REPO: u64 = SAV_NOT_IN_REPO << 1;
pub const SAV_NOT_IN_SUBMODULE: u64 = SAV_NOT_IN_REPO << 2;
pub const SAV_IN_SUBMODULE: u64 = SAV_NOT_IN_REPO << 3;
pub const SAV_NOT_HAS_SUBMODULES: u64 = SAV_NOT_IN_REPO << 4;
pub const SAV_HAS_SUBMODULES: u64 = SAV_NOT_IN_REPO << 5;
pub const SAV_REPO_STATE_MASK: u64 = SAV_NOT_IN_REPO
    | SAV_IN_REPO
    | SAV_NOT_IN_SUBMODULE
    | SAV_IN_SUBMODULE
    | SAV_NOT_HAS_SUBMODULES
    | SAV_HAS_SUBMODULES;

pub fn is_repo_workdir(dir_path: &str) -> bool {
    git2::Repository::open(&dir_path.path_absolute().unwrap()).is_ok()
}

pub fn get_repo_workdir_for_path(dir_path: &str) -> Option<String> {
    if let Ok(repo) = git2::Repository::discover(&dir_path.path_absolute().unwrap()) {
        if let Some(path) = repo.workdir() {
            Some(path.to_string_lossy().to_string())
        } else {
            None
        }
    } else {
        None
    }
}

pub fn get_repo_condns() -> MaskedCondns {
    let mut condns: u64;
    if is_repo_workdir(".") {
        condns = SAV_IN_REPO;
        if submodules::is_git_submodule(None) {
            condns |= SAV_IN_SUBMODULE;
        } else {
            condns |= SAV_NOT_IN_SUBMODULE;
        }
        if submodules::submodule_count() > 0 {
            condns |= SAV_HAS_SUBMODULES;
        } else {
            condns |= SAV_NOT_HAS_SUBMODULES;
        }
    } else {
        condns = SAV_NOT_IN_REPO | SAV_NOT_IN_SUBMODULE | SAV_NOT_HAS_SUBMODULES;
    }
    MaskedCondns {
        condns: condns,
        mask: SAV_REPO_STATE_MASK,
    }
}

#[derive(Debug)]
pub enum KRTError {
    IOError(io::Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for KRTError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KRTError is here!")
    }
}

impl Error for KRTError {
    fn description(&self) -> &str {
        match self {
            KRTError::IOError(_) => "I/O Error accessing known repos table",
            KRTError::JsonError(_) => "Serde Json Error accessing known repos table",
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            KRTError::IOError(err) => Some(err),
            KRTError::JsonError(err) => Some(err),
        }
    }
}

impl From<io::Error> for KRTError {
    fn from(error: io::Error) -> Self {
        KRTError::IOError(error)
    }
}

impl From<serde_json::Error> for KRTError {
    fn from(error: serde_json::Error) -> Self {
        KRTError::JsonError(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_repo_workdir_works() {
        println!("{} -> {:?}", "..", "..".path_absolute());
        assert!(!is_repo_workdir("."));
        assert!(!is_repo_workdir("../src"));
        assert!(is_repo_workdir(".."));
    }

    #[test]
    fn get_repo_workdir_for_path_works() {
        assert!(get_repo_workdir_for_path(".").is_some());
        assert!(get_repo_workdir_for_path("..").is_some());
        assert!(get_repo_workdir_for_path("../..").is_none());
    }
}