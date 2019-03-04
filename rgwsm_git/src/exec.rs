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

use std::env;
use std::process::Command;
use std::rc::Rc;
use std::string::FromUtf8Error;
use std::time::SystemTime;

use gtk;
use gtk::prelude::*;

use chrono::prelude::*;
use git2;
use shlex;

use pw_gix::sav_state::*;
use pw_gix::wrapper::*;

use pw_pathux::str_path::*;

use bab::enotify::EventNotifier;

pub const SAV_NOT_IN_REPO: u64 = SAV_SELN_MASK + 1;
pub const SAV_IN_REPO: u64 = SAV_SELN_MASK << 1;
pub const SAV_REPO_MASK: u64 = SAV_NOT_IN_REPO | SAV_IN_REPO;

fn is_repo_workdir(dir_path: &str) -> bool {
    git2::Repository::open(&dir_path.path_absolute().unwrap()).is_ok()
}

fn get_repo_workdir_for_path(dir_path: &str) -> Option<String> {
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

pub struct ExecConsole {
    text_view: gtk::TextView,
    pub event_notifier: Rc<EventNotifier>,
    pub changed_condns_notifier: Rc<ChangedCondnsNotifier>,
}

impl_widget_wrapper!(text_view: gtk::TextView, ExecConsole);

impl ExecConsole {
    pub fn new() -> Rc<Self> {
        let ec = Rc::new(Self {
            text_view: gtk::TextView::new(),
            event_notifier: Rc::new(EventNotifier::default()),
            changed_condns_notifier: ChangedCondnsNotifier::new(),
        });
        ec.append_bold("% ");
        ec
    }

    fn append_markup(&self, markup: &str) {
        let bfr = self
            .text_view
            .get_buffer()
            .expect("failed to find text buffer");
        let mut model_iter = bfr.get_end_iter();
        bfr.insert_markup(&mut model_iter, markup);
        if let Some(eobuf) = bfr.create_mark("eobuf", &bfr.get_end_iter(), false) {
            self.text_view
                .scroll_to_mark(&eobuf, 0.001, false, 0.0, 0.0);
        };
    }

    fn append_bold(&self, text: &str) {
        let markup = format!(
            r###"<span foreground="black" weight="bold" font_family="monospace">{}</span>"###,
            text
        );
        self.append_markup(&markup);
    }

    fn append_cmd(&self, text: &str) {
        let markup = format!(
            r###"<span foreground="black" font_family="monospace">{}</span>"###,
            text
        );
        self.append_markup(&markup);
        self.append_markup("\n");
    }

    fn append_stdout(&self, text: &str) {
        let markup = format!(
            r###"<span foreground="black" font_family="monospace">{}</span>"###,
            text
        );
        self.append_markup(&markup);
    }

    fn append_stderr(&self, text: &str) {
        let markup = format!(
            r###"<span foreground="AA0000" font_family="monospace">{}</span>"###,
            text
        );
        self.append_markup(&markup);
    }

    fn append_stdin(&self, text: &str) {
        let markup = format!(
            r###"<span foreground="00AA00" font_family="monospace">{}</span>"###,
            text
        );
        self.append_markup(&markup);
    }

    pub fn exec_cmd(&self, cmd: &str, events: u64) {
        let dt = DateTime::<Local>::from(SystemTime::now());
        self.append_bold(&format!("{}: ", dt.format("%Y-%m-%d-%H-%M-%S")));
        self.append_cmd(cmd);
        let cmd_line = shlex::split(cmd).unwrap();
        match Command::new(&cmd_line[0]).args(&cmd_line[1..]).output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                self.append_stdout(&stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                self.append_stderr(&stderr);
                self.append_bold("% ");
                if output.status.success() {
                    if events != 0 {
                        self.event_notifier.notify_events(events)
                    }
                } else {
                    let msg = format!("\"{}\": failed.", cmd);
                    self.warn_user(&msg, Some(&stderr));
                }
            }
            Err(err) => {
                let msg = format!("\"{}\": blew up!", cmd);
                self.report_error(&msg, &err);
            }
        }
    }

    pub fn chdir(&self, new_dir_path: &str) {
        self.append_cmd(&format!("chdir {}", shlex::quote(new_dir_path)));
        let mut adj_dir_path: String = new_dir_path.to_string();
        let mut adjusted = false;
        let mut in_repo = false;
        if is_repo_workdir(new_dir_path) {
            in_repo = true;
        } else if let Some(path) = get_repo_workdir_for_path(new_dir_path) {
            adj_dir_path = path;
            adjusted = true;
            in_repo = true;
        }
        match env::set_current_dir(&adj_dir_path) {
            Err(err) => {
                let stderr = format!("{}\n", err.to_string());
                self.append_stderr(&stderr);
                self.append_bold("% ");
                let msg = format!("chdir {} failed", shlex::quote(new_dir_path));
                self.report_error(&msg, &err);
            }
            Ok(_) => {
                if in_repo {
                    let masked_condns = MaskedCondns {
                        condns: SAV_IN_REPO,
                        mask: SAV_REPO_MASK,
                    };
                    self.changed_condns_notifier
                        .notify_changed_condns(masked_condns);
                    if adjusted {
                        let string = format!(
                            "Now in valid repo directory: {}.\n",
                            shlex::quote(&adj_dir_path)
                        );
                        self.append_stdout(&string);
                        self.append_bold("% ");
                        self.inform_user(&string, None);
                    } else {
                        self.append_stdout("Now in valid repo directory.\n");
                        self.append_bold("% ");
                    }
                } else {
                    let masked_condns = MaskedCondns {
                        condns: SAV_NOT_IN_REPO,
                        mask: SAV_REPO_MASK,
                    };
                    self.changed_condns_notifier
                        .notify_changed_condns(masked_condns);
                    self.append_bold("% ");
                }
            }
        }
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
