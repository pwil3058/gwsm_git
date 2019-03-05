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

use std::io::Write;
use std::process::Command;

use crypto_hash::{Algorithm, Hasher};
use lazy_static;
use regex::Regex;

use pw_pathux::str_path::*;

pub fn is_git_submodule(dir_path: Option<&str>) -> bool {
    if let Some(dir_path) = dir_path {
        dir_path.path_join(".git").path_is_file()
    } else {
        ".git".path_is_file()
    }
}

pub fn submodule_count() -> usize {
    let output = Command::new("git")
        .arg("submodule")
        .arg("status")
        .arg("--recursive")
        .output()
        .expect("submodule_count() failed");
    if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
            .to_string()
            .lines()
            .count()
    } else {
        0
    }
}

fn _get_submodule_paths_rawdata() -> (String, Vec<u8>) {
    let mut hasher = Hasher::new(Algorithm::SHA256);
    let text: String;
    let output = Command::new("git")
        .arg("submodule")
        .arg("status")
        .arg("--recursive")
        .output()
        .expect("getting all branches text failed");
    if output.status.success() {
        hasher.write_all(&output.stdout).expect("hasher blew up!!!");
        text = String::from_utf8_lossy(&output.stdout).to_string();
    } else {
        text = "".to_string();
    }
    (text, hasher.finish())
}

lazy_static! {
    static ref _SUBMODULE_PATH_RE: Regex =
        Regex::new(r"\s*([a-fA-F0-9]+)\s+([^(]+)(\s+\S*)?").unwrap();
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        assert!(false);
    }
}
