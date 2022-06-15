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

use git2;
use shlex;

use pw_gix::{
    gtk::{self, prelude::*},
    sav_state::*,
    wrapper::*,
};

use pw_pathux::str_path::*;

use crate::config;
use crate::exec::ExecConsole;
use crate::submodules;

pub const SAV_NOT_IN_REPO: u64 = SAV_NEXT_CONDN;
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

#[derive(PWO, Wrapper)]
pub struct OpenKnownRepoMenuItem {
    menu_item: gtk::MenuItem,
    exec_console: Rc<ExecConsole>,
}

impl OpenKnownRepoMenuItem {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<Self> {
        let ormi = Rc::new(Self {
            menu_item: gtk::MenuItem::with_label("Change Directory To ->"),
            exec_console: Rc::clone(exec_console),
        });

        let ormi_clone = Rc::clone(&ormi);
        ormi.menu_item
            .connect_enter_notify_event(move |menu_item, _| {
                let submenu = ormi_clone.build_submenu();
                menu_item.set_submenu(Some(&submenu));
                gtk::Inhibit(false)
            });

        ormi
    }

    fn build_submenu(&self) -> gtk::Menu {
        let menu = gtk::Menu::new();
        let mut table = read_known_repos_table().expect("failed to read known repos table");
        for item in table.drain(..) {
            let label = format!("{} :-> {}", shlex::quote(&item.0), shlex::quote(&item.1));
            let menu_item = gtk::MenuItem::with_label(&label);
            let exec_console = Rc::clone(&self.exec_console);
            menu_item.connect_activate(move |_| exec_console.chdir(&item.1));
            menu.append(&menu_item);
        }
        menu.show_all();

        menu
    }
}

#[derive(PWO, Wrapper)]
pub struct CloneRepoWidget {
    grid: gtk::Grid,
    src_entry: gtk::Entry,
    browse_src_btn: gtk::Button,
    as_entry: gtk::Entry,
    as_default_btn: gtk::Button,
    in_entry: gtk::Entry,
    browse_in_btn: gtk::Button,
}

impl CloneRepoWidget {
    fn new() -> Rc<Self> {
        let crw = Rc::new(Self {
            grid: gtk::Grid::new(),
            src_entry: gtk::Entry::new(),
            browse_src_btn: gtk::Button::with_label("Browse"),
            as_entry: gtk::Entry::new(),
            as_default_btn: gtk::Button::with_label("Default"),
            in_entry: gtk::Entry::new(),
            browse_in_btn: gtk::Button::with_label("Browse"),
        });

        crw.browse_src_btn
            .set_tooltip_text(Some("Browse local file system for repo to be cloned."));
        crw.browse_in_btn.set_tooltip_text(Some(
            "Browse local file system for parent directory in which to place cloned repository.",
        ));

        let src_label = gtk::Label::new(Some("Repo to clone:"));
        crw.grid.attach(&src_label, 0, 0, 1, 1);
        let as_label = gtk::Label::new(Some("As:"));
        crw.grid
            .attach_next_to(&as_label, Some(&src_label), gtk::PositionType::Bottom, 1, 1);
        let in_label = gtk::Label::new(Some("In directory:"));
        crw.grid
            .attach_next_to(&in_label, Some(&as_label), gtk::PositionType::Bottom, 1, 1);

        crw.src_entry.set_width_chars(64);
        crw.grid.attach_next_to(
            &crw.src_entry,
            Some(&src_label),
            gtk::PositionType::Right,
            4,
            1,
        );
        crw.grid.attach_next_to(
            &crw.browse_src_btn,
            Some(&crw.src_entry),
            gtk::PositionType::Right,
            1,
            1,
        );

        crw.grid.attach_next_to(
            &crw.as_entry,
            Some(&as_label),
            gtk::PositionType::Right,
            4,
            1,
        );
        crw.grid.attach_next_to(
            &crw.as_default_btn,
            Some(&crw.as_entry),
            gtk::PositionType::Right,
            1,
            1,
        );

        crw.grid.attach_next_to(
            &crw.in_entry,
            Some(&in_label),
            gtk::PositionType::Right,
            4,
            1,
        );
        crw.grid.attach_next_to(
            &crw.browse_in_btn,
            Some(&crw.in_entry),
            gtk::PositionType::Right,
            1,
            1,
        );

        crw.grid.show_all();

        let crw_clone = Rc::clone(&crw);
        crw.browse_src_btn.connect_clicked(move |_| {
            if let Some(src_dir) = crw_clone.select_dir(None, None, true, true) {
                crw_clone.src_entry.set_text(&src_dir.to_string_lossy())
            }
        });

        let crw_clone = Rc::clone(&crw);
        crw.browse_in_btn.connect_clicked(move |_| {
            if let Some(src_dir) = crw_clone.select_dir(None, None, true, true) {
                crw_clone.in_entry.set_text(&src_dir.to_string_lossy())
            }
        });

        let crw_clone = Rc::clone(&crw);
        crw.as_default_btn.connect_clicked(move |_| {
            if let Some(text) = crw_clone.source_url() {
                if let Some(default) = text.path_file_name() {
                    crw_clone.as_entry.set_text(&default)
                } else {
                    crw_clone.inform_user("Unable to determine default.", None)
                }
            } else {
                crw_clone.inform_user("Source URL is empty.", None)
            }
        });

        crw
    }

    fn source_url(&self) -> Option<String> {
        let text = self.src_entry.get_text();
        if text.len() > 0 {
            Some(text.into())
        } else {
            None
        }
    }

    fn as_name(&self) -> Option<String> {
        let text = self.as_entry.get_text();
        if text.len() > 0 {
            Some(text.into())
        } else {
            None
        }
    }

    fn in_dir(&self) -> Option<String> {
        let text = self.in_entry.get_text();
        if text.len() > 0 {
            Some(text.into())
        } else {
            None
        }
    }
}

#[derive(PWO, Wrapper)]
pub struct CloneRepoMenuItem {
    menu_item: gtk::MenuItem,
    exec_console: Rc<ExecConsole>,
}

impl CloneRepoMenuItem {
    fn new(exec_console: &Rc<ExecConsole>) -> Rc<Self> {
        let crmi = Rc::new(Self {
            menu_item: gtk::MenuItem::with_label("Clone"),
            exec_console: Rc::clone(exec_console),
        });
        let crmi_clone = Rc::clone(&crmi);
        crmi.menu_item
            .connect_activate(move |_| crmi_clone.clone_a_repo());

        crmi
    }

    fn clone_a_repo(&self) {
        let dialog = self
            .new_dialog_builder()
            .title("Clone Repository")
            .destroy_with_parent(true)
            .modal(true)
            .build();
        for button in Self::CANCEL_OK_BUTTONS.iter() {
            dialog.add_button(button.0, button.1);
        }
        dialog.set_default_response(gtk::ResponseType::Ok);
        let crw = CloneRepoWidget::new();
        dialog
            .get_content_area()
            .pack_start(crw.pwo(), true, true, 0);
        while gtk::ResponseType::from(dialog.run()) == gtk::ResponseType::Ok {
            if let Some(src_url) = crw.source_url() {
                if let Some(as_name) = crw.as_name() {
                    let in_dir = if let Some(in_dir) = crw.in_dir() {
                        in_dir
                    } else {
                        str_path_current_dir_or_panic()
                    };
                    let tgt_dir = in_dir.path_join(&as_name);
                    let cmd = format!(
                        "git clone {} {}",
                        shlex::quote(&src_url),
                        shlex::quote(&tgt_dir)
                    );
                    let result = self.exec_console.exec_cmd(&cmd, 0);
                    self.exec_console.report_any_command_problems(&cmd, &result);
                    if let Ok(ref output) = result {
                        if output.status.success() {
                            self.exec_console.chdir(&tgt_dir);
                            break;
                        }
                    }
                } else {
                    self.exec_console
                        .inform_user("A target directory name is required.", None);
                }
            } else {
                self.exec_console
                    .inform_user("A source URL is required.", None);
            }
        }
        unsafe { dialog.destroy() };
    }
}

pub fn create_workspaces_menu(exec_console: &Rc<ExecConsole>) -> gtk::MenuItem {
    let mi = gtk::MenuItem::with_label("Workspaces");
    let menu = gtk::Menu::new();
    mi.set_submenu(Some(&menu));
    menu.append(CloneRepoMenuItem::new(exec_console).pwo());
    menu.append(OpenKnownRepoMenuItem::new(exec_console).pwo());
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
