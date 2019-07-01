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
use std::io;
use std::process::{self, Command, Output};
use std::rc::Rc;
use std::time::SystemTime;

use gtk;
use gtk::prelude::*;

use chrono::prelude::*;
use shlex;
use xml::escape;

use pw_gix::recollections;
use pw_gix::sav_state::*;
use pw_gix::timeout;
use pw_gix::wrapper::*;

use pw_pathux::str_path::*;

use crate::action_icons;
use crate::events::{self, EventNotifier};
use crate::repos;

pub struct ExecConsole {
    scrolled_window: gtk::ScrolledWindow,
    text_view: gtk::TextView,
    pub update_button: gtk::Button,
    pub event_notifier: Rc<EventNotifier>,
    pub changed_condns_notifier: Rc<ChangedCondnsNotifier>,
    pub managed_buttons: Rc<ConditionalWidgetGroups<gtk::Button>>,
    pub managed_check_buttons: Rc<ConditionalWidgetGroups<gtk::CheckButton>>,
    pub managed_menu_items: Rc<ConditionalWidgetGroups<gtk::MenuItem>>,
    auto_update: Rc<timeout::ControlledTimeoutCycle>,
}

impl_widget_wrapper!(scrolled_window: gtk::ScrolledWindow, ExecConsole);

impl ExecConsole {
    pub fn new() -> Rc<Self> {
        let changed_condns_notifier = ChangedCondnsNotifier::new(0);
        let managed_buttons = ConditionalWidgetGroups::<gtk::Button>::new(
            WidgetStatesControlled::Sensitivity,
            None,
            Some(&changed_condns_notifier),
        );
        let managed_check_buttons = ConditionalWidgetGroups::<gtk::CheckButton>::new(
            WidgetStatesControlled::Sensitivity,
            None,
            Some(&changed_condns_notifier),
        );
        let managed_menu_items = ConditionalWidgetGroups::<gtk::MenuItem>::new(
            WidgetStatesControlled::Sensitivity,
            None,
            Some(&changed_condns_notifier),
        );
        let adj: Option<&gtk::Adjustment> = None;
        let ec = Rc::new(Self {
            scrolled_window: gtk::ScrolledWindow::new(adj, adj),
            text_view: gtk::TextView::new(),
            update_button: gtk::Button::new_with_label("Update"),
            event_notifier: EventNotifier::new(),
            changed_condns_notifier: changed_condns_notifier,
            managed_buttons: managed_buttons,
            managed_check_buttons: managed_check_buttons,
            managed_menu_items: managed_menu_items,
            auto_update: timeout::ControlledTimeoutCycle::new("Auto Update", true, 10),
        });
        ec.text_view.set_editable(false);
        ec.scrolled_window
            .set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Always);
        ec.scrolled_window.add(&ec.text_view);
        ec.append_bold("% ");

        let ec_clone = Rc::clone(&ec);
        ec.auto_update.register_callback(Box::new(move || {
            ec_clone
                .event_notifier
                .notify_events(events::EV_AUTO_UPDATE);
        }));

        ec.update_button.set_image(&action_icons::update_image(32));
        ec.update_button.set_image_position(gtk::PositionType::Top);
        let ec_clone = Rc::clone(&ec);
        ec.update_button.connect_clicked(move |_| {
            ec_clone
                .event_notifier
                .notify_events(events::EV_AUTO_UPDATE)
        });

        ec.check_repo_states();

        ec
    }

    pub fn auto_update_check_item(&self) -> gtk::CheckMenuItem {
        self.auto_update.check_menu_item()
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
            escape::escape_str_attribute(text)
        );
        self.append_markup(&markup);
    }

    fn append_cmd(&self, text: &str) {
        let markup = format!(
            r###"<span foreground="black" font_family="monospace">{}</span>"###,
            escape::escape_str_attribute(text)
        );
        self.append_markup(&markup);
        self.append_markup("\n");
    }

    fn append_stdout(&self, text: &str) {
        let markup = format!(
            r###"<span foreground="black" font_family="monospace">{}</span>"###,
            escape::escape_str_attribute(text)
        );
        self.append_markup(&markup);
    }

    fn append_stderr(&self, text: &str) {
        let markup = format!(
            r###"<span foreground="#AA0000" font_family="monospace">{}</span>"###,
            escape::escape_str_attribute(text)
        );
        self.append_markup(&markup);
    }

    fn _append_stdin(&self, text: &str) {
        let markup = format!(
            r###"<span foreground="00AA00" font_family="monospace">{}</span>"###,
            escape::escape_str_attribute(text)
        );
        self.append_markup(&markup);
    }

    pub fn exec_cmd(&self, cmd: &str, events: u64) -> io::Result<Output> {
        let dt = DateTime::<Local>::from(SystemTime::now());
        self.append_bold(&format!("{}: ", dt.format("%Y-%m-%d-%H-%M-%S")));
        self.append_cmd(cmd);
        yield_to_pending_events!();
        let cmd_line = shlex::split(cmd).unwrap();
        let output = Command::new(&cmd_line[0]).args(&cmd_line[1..]).output()?;
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            self.append_stdout(&stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            self.append_stderr(&stderr);
            self.append_bold("% ");
        }
        yield_to_pending_events!();
        if output.status.success() && events != 0 {
            self.event_notifier.notify_events(events)
        }
        Ok(output)
    }

    pub fn in_repo(&self) -> bool {
        repos::is_repo_workdir(".")
    }

    pub fn check_repo_states(&self) {
        let masked_condns = repos::get_repo_condns();
        self.changed_condns_notifier
            .notify_changed_condns(masked_condns);
    }

    pub fn chdir(&self, new_dir_path: &str) {
        // TODO: add notification of events to chdir()
        self.append_cmd(&format!("chdir {}", shlex::quote(new_dir_path)));
        let mut adj_dir_path: String = new_dir_path.to_string();
        let mut adjusted = false;
        let mut in_repo = false;
        if repos::is_repo_workdir(new_dir_path) {
            in_repo = true;
        } else if let Some(path) = repos::get_repo_workdir_for_path(new_dir_path) {
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
                    if let Ok(abs_path) = ".".path_absolute() {
                        recollections::remember("last:git:ws:dir", &abs_path);
                        repos::add_to_known_repos(&abs_path).expect("saving known repos failed");
                    }
                } else {
                    self.append_bold("% ");
                }
                self.check_repo_states();
                self.event_notifier.notify_events(events::EV_CHANGE_DIR);
            }
        }
    }
}

pub fn create_files_menu(exec_console: &Rc<ExecConsole>) -> gtk::MenuItem {
    let mi = gtk::MenuItem::new_with_label("Files");
    let menu = gtk::Menu::new();
    mi.set_submenu(&menu);

    let chdir_menu_item = gtk::MenuItem::new_with_label("Open");
    let ec_clone = Rc::clone(&exec_console);
    chdir_menu_item.connect_activate(move |_| {
        if let Some(path) = ec_clone.browse_path(
            Some("Directory Path"),
            None,
            gtk::FileChooserAction::CreateFolder,
            false,
        ) {
            ec_clone.chdir(&path.to_string_lossy().to_string());
        }
    });
    menu.append(&chdir_menu_item);

    let init_menu_item = gtk::MenuItem::new_with_label("Init");
    exec_console
        .managed_menu_items
        .add_widget("init", &init_menu_item, repos::SAV_NOT_IN_REPO);
    let ec_clone = Rc::clone(&exec_console);
    init_menu_item.connect_activate(move |_| {
        let cmd = "git init";
        let result = ec_clone.exec_cmd(&cmd, events::EV_CHANGE_DIR);
        ec_clone.check_repo_states();
        ec_clone.report_any_command_problems(&cmd, &result);
    });
    menu.append(&init_menu_item);

    let exit_menu_item = gtk::MenuItem::new_with_label("Exit");
    exit_menu_item.connect_activate(move |_| {
        process::exit(0);
    });
    menu.append(&exit_menu_item);

    mi
}

pub fn create_friends_menu(exec_console: &Rc<ExecConsole>) -> gtk::MenuItem {
    let mi = gtk::MenuItem::new_with_label("Friends");
    let menu = gtk::Menu::new();
    mi.set_submenu(&menu);

    for friend in ["gitg", "gitk", "git-dag", "git-cola", "meld", "regexxer"].iter() {
        let menu_item = gtk::MenuItem::new_with_label(&friend.to_string());
        let ec_clone = Rc::clone(&exec_console);
        menu_item.connect_activate(move |_| {
            if let Err(err) = Command::new(&friend).spawn() {
                let msg = format!("Error running \"{}\"", friend);
                ec_clone.report_error(&msg, &err);
            }
        });
        menu.append(&menu_item);
    }

    mi
}
