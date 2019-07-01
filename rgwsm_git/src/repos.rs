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
use std::fs::File;
use std::io::{self, Read, Write};
use std::rc::Rc;

use gtk;
use gtk::prelude::*;

use git2;
use shlex;

use pw_gix::sav_state::*;
use pw_gix::wrapper::*;

use pw_pathux::str_path::*;

use crate::config;
use crate::exec::ExecConsole;
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

fn known_repos_table_filepath() -> PathBuf {
    let mut pathbuf = config::get_config_dir_path();
    pathbuf.push("known_repos_table");
    pathbuf
}

fn read_known_repos_table() -> Result<Vec<(String, String)>, KRTError> {
    let mut file = File::open(known_repos_table_filepath())?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let mut v: Vec<(String, String)> = serde_json::from_str(&buffer)?;
    // Prune any repos that no longer exist.
    let mut pruned = vec![];
    for item in v.drain(..) {
        if is_repo_workdir(&item.1) {
            pruned.push(item);
        }
    }
    Ok(pruned)
}

fn write_known_repos_table(table: &[(String, String)]) -> Result<usize, KRTError> {
    let data = serde_json::to_string(table)?;
    let mut file = File::create(known_repos_table_filepath())?;
    let nbytes = file.write(data.as_bytes())?;
    Ok(nbytes)
}

pub fn init_known_repos_table() {
    if !known_repos_table_filepath().is_file() {
        write_known_repos_table(&vec![]).expect("failed to initialize editor assignment table");
    }
}

pub fn add_to_known_repos(repo_path: &str) -> Result<(), KRTError> {
    if is_repo_workdir(repo_path) {
        if let Some(dir_name) = repo_path.path_file_name() {
            let new_entry = (dir_name, repo_path.to_string());
            let mut known_repos = read_known_repos_table()?;
            let result = known_repos.binary_search(&new_entry);
            if let Err(insert_index) = result {
                known_repos.insert(insert_index, new_entry);
            }
            write_known_repos_table(&known_repos)?;
        }
    }
    Ok(())
}

pub struct OpenKnownRepoMenuItem {
    menu_item: gtk::MenuItem,
    exec_console: Rc<ExecConsole>,
}

impl_widget_wrapper!(menu_item: gtk::MenuItem, OpenKnownRepoMenuItem);

impl OpenKnownRepoMenuItem {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<Self> {
        let ormi = Rc::new(Self {
            menu_item: gtk::MenuItem::new_with_label("Change Directory To ->"),
            exec_console: Rc::clone(exec_console),
        });

        let ormi_clone = Rc::clone(&ormi);
        ormi.menu_item
            .connect_enter_notify_event(move |menu_item, _| {
                let submenu = ormi_clone.build_submenu();
                menu_item.set_submenu(&submenu);
                gtk::Inhibit(false)
            });

        ormi
    }

    fn build_submenu(&self) -> gtk::Menu {
        let menu = gtk::Menu::new();
        let mut table = read_known_repos_table().expect("failed to read known repos table");
        for item in table.drain(..) {
            let label = format!("{} :-> {}", shlex::quote(&item.0), shlex::quote(&item.1));
            let menu_item = gtk::MenuItem::new_with_label(&label);
            let exec_console = Rc::clone(&self.exec_console);
            menu_item.connect_activate(move |_| exec_console.chdir(&item.1));
            menu.append(&menu_item);
        }
        menu.show_all();

        menu
    }
}

pub fn create_workspaces_menu(exec_console: &Rc<ExecConsole>) -> gtk::MenuItem {
    let mi = gtk::MenuItem::new_with_label("Workspaces");
    let menu = gtk::Menu::new();
    mi.set_submenu(&menu);
    menu.append(&OpenKnownRepoMenuItem::new(exec_console).pwo());
    mi
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
