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

use std::cell::RefCell;
use std::io::Write;
use std::process::Command;
use std::rc::Rc;

use crypto_hash::{Algorithm, Hasher};

use cub_diff_lib::diff::DiffPlusParser;
use cub_diff_lib::lines::*;
use cub_diff_lib_gtk::diff::DiffPlusNotebook;

use pw_gix::{
    gtk::{self, prelude::*},
    gtkx::window::RememberGeometry,
    sourceview::{self, ViewExt},
    wrapper::*,
};

use crate::action_icons;
use crate::config;
use crate::events;
use crate::exec::ExecConsole;
use crate::message::last_commit_message;
use crate::repos;

#[derive(PWO)]
pub struct CommitButton {
    button: gtk::Button,
    window: gtk::Window,
    commit_widget: Rc<CommitWidget>,
    _exec_console: Rc<ExecConsole>,
}

impl CommitButton {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<Self> {
        let button = gtk::Button::new();
        button.set_tooltip_text(Some("Open the commit widget."));
        button.set_image(Some(&action_icons::commit_image(32)));
        button.set_image_position(gtk::PositionType::Top);
        button.set_label("commit");
        exec_console
            .managed_buttons
            .add_widget("commit", &button, repos::SAV_IN_REPO);

        let db = Rc::new(Self {
            button,
            window: gtk::Window::new(gtk::WindowType::Toplevel),
            commit_widget: CommitWidget::new(exec_console),
            _exec_console: Rc::clone(exec_console),
        });
        db.window
            .set_geometry_from_recollections("commit:window", (700, 600));
        db.window.set_destroy_with_parent(true);
        db.window.set_title(&config::window_title(Some("commit")));
        db.window.connect_delete_event(move |w, _| {
            w.hide_on_delete();
            Inhibit(true)
        });
        db.window.add(db.commit_widget.pwo());
        db.window.show_all();
        db.window.hide();

        let db_clone = Rc::clone(&db);
        exec_console.event_notifier.add_notification_cb(
            events::EV_CHANGE_DIR,
            Box::new(move |_| {
                db_clone
                    .window
                    .set_title(&config::window_title(Some("diff")));
            }),
        );

        let db_clone = Rc::clone(&db);
        db.button
            .connect_clicked(move |_| db_clone.window.present());

        db
    }
}

#[derive(PWO, Wrapper)]
struct IndexDiffWidget {
    v_box: gtk::Box,
    diff_notebook: Rc<DiffPlusNotebook>,
    current_digest: RefCell<Vec<u8>>,
    exec_console: Rc<ExecConsole>,
    diff_plus_parser: DiffPlusParser,
}

impl IndexDiffWidget {
    fn new(exec_console: &Rc<ExecConsole>) -> Rc<Self> {
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        h_box.pack_start(&gtk::Label::new(Some("Diffs")), false, false, 0);
        v_box.pack_start(&h_box, false, false, 0);
        let diff_notebook = DiffPlusNotebook::new(1);
        h_box.pack_end(diff_notebook.tws_count_display().pwo(), false, false, 0);
        v_box.pack_start(diff_notebook.pwo(), true, true, 0);
        let idw = Rc::new(Self {
            v_box,
            diff_notebook,
            current_digest: RefCell::new(Vec::new()),
            exec_console: Rc::clone(exec_console),
            diff_plus_parser: DiffPlusParser::new(),
        });

        let idw_clone = Rc::clone(&idw);
        idw.exec_console.event_notifier.add_notification_cb(
            events::EV_AUTO_UPDATE | events::EV_CHECKOUT | events::EV_FILES_CHANGE,
            Box::new(move |_| {
                idw_clone.update();
            }),
        );

        let idw_clone = Rc::clone(&idw);
        idw.exec_console.event_notifier.add_notification_cb(
            events::EV_CHANGE_DIR,
            Box::new(move |_| {
                idw_clone.repopulate();
            }),
        );

        idw.repopulate();

        idw
    }

    fn get_diff_text(&self) -> (String, Vec<u8>) {
        let mut cmd = Command::new("git");
        cmd.arg("diff")
            .arg("--no-ext-diff")
            .arg("-M")
            .arg("--staged");
        let output = cmd.output().expect("\"git diff\" blew up");
        if output.status.success() {
            let mut hasher = Hasher::new(Algorithm::SHA256);
            hasher.write_all(&output.stdout).expect("hasher blew up!!!");
            (
                String::from_utf8_lossy(&output.stdout).to_string(),
                hasher.finish(),
            )
        } else {
            ("".to_string(), vec![])
        }
    }

    fn repopulate(&self) {
        let (text, new_digest) = self.get_diff_text();
        *self.current_digest.borrow_mut() = new_digest;
        let lines = Lines::from_string(&text);
        match self.diff_plus_parser.parse_lines(&lines) {
            Ok(ref diff_pluses) => self.diff_notebook.repopulate(diff_pluses),
            Err(err) => {
                self.diff_notebook.repopulate(&vec![]);
                self.report_error("Malformed diff text", &err);
            }
        }
    }

    fn update(&self) {
        let (text, new_digest) = self.get_diff_text();
        let go_ahead = new_digest != *self.current_digest.borrow();
        if go_ahead {
            *self.current_digest.borrow_mut() = new_digest;
            let lines = Lines::from_string(&text);
            match self.diff_plus_parser.parse_lines(&lines) {
                Ok(ref diff_pluses) => self.diff_notebook.update(diff_pluses),
                Err(err) => {
                    self.diff_notebook.update(&vec![]);
                    self.report_error("Malformed diff text", &err);
                }
            }
        }
    }
}

#[derive(PWO, Wrapper)]
struct CommitWidget {
    v_box: gtk::Box,
    text_view: sourceview::View,
    index_diff_widget: Rc<IndexDiffWidget>,
    exec_console: Rc<ExecConsole>,
    exec_button: gtk::Button,
    amend_option_button: gtk::CheckButton,
    signoff_option_button: gtk::CheckButton,
}

fn get_name_and_email_string() -> String {
    let output = Command::new("git")
        .arg("config")
        .arg("user.name")
        .output()
        .expect("\"git config user.name\" blew up");
    let name = if output.status.success() && !output.stdout.is_empty() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        "user name".to_string()
    };
    let output = Command::new("git")
        .arg("config")
        .arg("user.email")
        .output()
        .expect("\"git config user.email\" blew up");
    let email = if output.status.success() && !output.stdout.is_empty() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        "user@somewhere".to_string()
    };
    format!("{} <{}>", name.trim_end(), email.trim_end())
}

fn insert_acked_by_at_cursor(buffer: &gtk::TextBuffer) {
    let text = format!("Acked-by: {}", get_name_and_email_string());
    buffer.insert_at_cursor(&text)
}

fn insert_signed_off_by_at_cursor(buffer: &gtk::TextBuffer) {
    let text = format!("Signed-off-by: {}", get_name_and_email_string());
    buffer.insert_at_cursor(&text)
}

impl CommitWidget {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<Self> {
        let cw = Rc::new(Self {
            v_box: gtk::Box::new(gtk::Orientation::Vertical, 0),
            text_view: sourceview::View::new(),
            index_diff_widget: IndexDiffWidget::new(exec_console),
            exec_console: Rc::clone(exec_console),
            exec_button: gtk::Button::new(),
            amend_option_button: gtk::CheckButton::with_label("--amend"),
            signoff_option_button: gtk::CheckButton::with_label("--signoff"),
        });

        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        h_box.pack_start(&gtk::Label::new(Some("Message")), false, false, 0);
        h_box.pack_start(
            &gtk::Box::new(gtk::Orientation::Horizontal, 0),
            true,
            true,
            0,
        );
        h_box.pack_end(&cw.exec_button, false, false, 0);
        h_box.pack_end(&cw.amend_option_button, false, false, 0);
        h_box.pack_end(&cw.signoff_option_button, false, false, 0);
        cw.v_box.pack_start(&h_box, false, false, 0);
        cw.exec_button.set_label("Commit");
        cw.exec_console.managed_buttons.add_widget(
            "exec_commit",
            &cw.exec_button,
            repos::SAV_IN_REPO,
        );
        cw.exec_console.managed_check_buttons.add_widget(
            "amend_option",
            &cw.amend_option_button,
            repos::SAV_IN_REPO,
        );
        cw.exec_console.managed_check_buttons.add_widget(
            "signoff_option",
            &cw.signoff_option_button,
            repos::SAV_IN_REPO,
        );

        let cw_clone = Rc::clone(&cw);
        cw.amend_option_button.connect_toggled(move |cb| {
            if cb.get_active() {
                let buffer = cw_clone
                    .text_view
                    .get_buffer()
                    .expect("get_buffer() failed");
                buffer.insert_at_cursor(&last_commit_message());
            }
        });

        let cw_clone = Rc::clone(&cw);
        cw.exec_button.connect_clicked(move |_| {
            let buffer = cw_clone
                .text_view
                .get_buffer()
                .expect("get_buffer() failed");
            let start = buffer.get_start_iter();
            let end = buffer.get_end_iter();
            let text = buffer
                .get_text(&start, &end, false)
                .expect("get_text() failed");
            if text.len() > 0 {
                let cmd = if cw_clone.signoff_option_button.get_active() {
                    if cw_clone.amend_option_button.get_active() {
                        format!("git commit --signoff --amend -m {}", shlex::quote(&text))
                    } else {
                        format!("git commit --signoff -m {}", shlex::quote(&text))
                    }
                } else if cw_clone.amend_option_button.get_active() {
                    format!("git commit --amend -m {}", shlex::quote(&text))
                } else {
                    format!("git commit -m {}", shlex::quote(&text))
                };
                let result = cw_clone.exec_console.exec_cmd(&cmd, events::EV_COMMIT);
                if let Ok(ref output) = result {
                    if output.status.success() {
                        buffer.set_text("")
                    }
                }
                cw_clone.amend_option_button.set_active(false);
                cw_clone.report_any_command_problems(&cmd, &result);
            } else {
                cw_clone.warn_user("Commit message is empty!", None);
            }
        });

        cw.text_view.set_monospace(true);
        cw.text_view.set_show_right_margin(true);
        cw.text_view.set_right_margin_position(71);
        cw.text_view.connect_populate_popup(|view, widget| {
            if let Ok(ref menu) = widget.clone().downcast::<gtk::Menu>() {
                let mi = gtk::MenuItem::with_label("Insert Acked-by");
                let buffer = view.get_buffer().unwrap();
                mi.connect_activate(move |_| insert_acked_by_at_cursor(&buffer));
                menu.append(&mi);
                let mi = gtk::MenuItem::with_label("Insert Signed-off-by");
                let buffer = view.get_buffer().unwrap();
                mi.connect_activate(move |_| insert_signed_off_by_at_cursor(&buffer));
                menu.append(&mi);
                menu.show_all();
            }
        });

        let adj: Option<&gtk::Adjustment> = None;
        let scrolled_window = gtk::ScrolledWindow::new(adj, adj);
        scrolled_window.add(&cw.text_view);
        cw.v_box.pack_start(&scrolled_window, true, true, 0);
        cw.v_box
            .pack_start(cw.index_diff_widget.pwo(), true, true, 0);
        cw.v_box.show_all();

        cw
    }
}
