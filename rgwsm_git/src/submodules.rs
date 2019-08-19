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
use std::rc::Rc;

use crypto_hash::{Algorithm, Hasher};
use git2;

use regex::Regex;

use gtk::prelude::*;

use pw_gix::wrapper::*;

use pw_pathux::str_path::*;

use crate::action_icons;
use crate::exec::ExecConsole;
use crate::repos;

pub fn is_git_submodule(dir_path: Option<&str>) -> bool {
    if let Some(dir_path) = dir_path {
        dir_path.path_join(".git").path_is_file()
    } else {
        ".git".path_is_file()
    }
}

fn find_submodule_parent() -> Option<String> {
    assert!(is_git_submodule(None));
    if let Ok(repo) = git2::Repository::discover("..") {
        if let Some(wd) = repo.workdir() {
            Some(wd.to_string_lossy().to_string())
        } else {
            None
        }
    } else {
        None
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

pub struct SubmoduleParentButton {
    button: gtk::Button,
    exec_console: Rc<ExecConsole>,
}

impl_widget_wrapper!(button: gtk::Button, SubmoduleParentButton);

impl SubmoduleParentButton {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<Self> {
        let button = gtk::Button::new();
        button.set_tooltip_text(Some(
            "Return to the working directory of this submodule's superproject.",
        ));
        button.set_image(Some(&action_icons::superproject_image(32)));
        button.set_image_position(gtk::PositionType::Top);
        button.set_label("Super");
        exec_console
            .managed_buttons
            .add_widget("chdir_parent", &button, repos::SAV_IN_SUBMODULE);
        let bb = Rc::new(Self {
            button: button,
            exec_console: Rc::clone(&exec_console),
        });

        let bb_clone = Rc::clone(&bb);
        bb.button.connect_clicked(move |_| {
            if let Some(parent_dir) = find_submodule_parent() {
                bb_clone.exec_console.chdir(&parent_dir)
            }
        });

        bb
    }
}
