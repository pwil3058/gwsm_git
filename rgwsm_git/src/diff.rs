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

use gtk;
use gtk::prelude::*;

use crypto_hash::{Algorithm, Hasher};

use cub_diff_gui_lib::diff::DiffPlusNotebook;
use cub_diff_lib::diff::DiffPlusParser;
use cub_diff_lib::lines::*;
use pw_gix::gtkx::window::RememberGeometry;
use pw_gix::wrapper::*;

use crate::action_icons;
use crate::config;
use crate::events;
use crate::exec::{self, ExecConsole};

pub struct DiffButton {
    button: gtk::Button,
    window: gtk::Window,
    wdtw: Rc<WdDiffTextWidget>,
    _exec_console: Rc<ExecConsole>,
}

impl_widget_wrapper!(button: gtk::Button, DiffButton);

impl DiffButton {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<Self> {
        let button = gtk::Button::new();
        button.set_tooltip_text(Some(
            "View diffs for the working directory or staged files.",
        ));
        button.set_image(&action_icons::diff_image(32));
        button.set_image_position(gtk::PositionType::Top);
        button.set_label("diff");
        exec_console
            .managed_buttons
            .add_widget("diff", &button, exec::SAV_IN_REPO);

        let db = Rc::new(Self {
            button: button,
            window: gtk::Window::new(gtk::WindowType::Toplevel),
            wdtw: WdDiffTextWidget::new(exec_console),
            _exec_console: Rc::clone(&exec_console),
        });
        db.wdtw.repopulate();
        db.window
            .set_geometry_from_recollections("ws::diff:display", (400, 600));
        db.window.set_destroy_with_parent(true);
        db.window.set_title(&config::window_title(Some("diff")));
        db.window.connect_delete_event(move |w, _| {
            w.hide_on_delete();
            gtk::Inhibit(true)
        });
        db.window.add(&db.wdtw.pwo());
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

struct WdDiffTextWidget {
    v_box: gtk::Box,
    diff_rb: gtk::RadioButton,
    diff_staged_rb: gtk::RadioButton,
    diff_head_rb: gtk::RadioButton,
    diff_notebook: Rc<DiffPlusNotebook>,
    current_digest: RefCell<Vec<u8>>,
    exec_console: Rc<ExecConsole>,
    diff_plus_parser: DiffPlusParser,
}

impl_widget_wrapper!(v_box: gtk::Box, WdDiffTextWidget);

impl WdDiffTextWidget {
    fn new(exec_console: &Rc<ExecConsole>) -> Rc<Self> {
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let diff_rb = gtk::RadioButton::new_with_label("git diff");
        let diff_staged_rb =
            gtk::RadioButton::new_with_label_from_widget(&diff_rb, "git diff --staged");
        let diff_head_rb = gtk::RadioButton::new_with_label_from_widget(&diff_rb, "git diff HEAD");
        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        h_box.pack_start(&diff_rb, false, false, 0);
        h_box.pack_start(&diff_staged_rb, false, false, 0);
        h_box.pack_start(&diff_head_rb, false, false, 0);
        v_box.pack_start(&h_box, false, false, 0);
        let diff_notebook = DiffPlusNotebook::new(1);
        h_box.pack_end(&diff_notebook.tws_count_display().pwo(), false, false, 0);
        v_box.pack_start(&diff_notebook.pwo(), true, true, 0);
        let wdtw = Rc::new(Self {
            v_box,
            diff_rb,
            diff_staged_rb,
            diff_head_rb,
            diff_notebook: diff_notebook,
            current_digest: RefCell::new(Vec::new()),
            exec_console: Rc::clone(&exec_console),
            diff_plus_parser: DiffPlusParser::new(),
        });
        // NB: only update when active to stop double update
        let wdtw_clone = Rc::clone(&wdtw);
        wdtw.diff_rb.connect_toggled(move |rb| {
            if rb.get_active() {
                wdtw_clone.update();
            }
        });
        let wdtw_clone = Rc::clone(&wdtw);
        wdtw.diff_staged_rb.connect_toggled(move |rb| {
            if rb.get_active() {
                wdtw_clone.update();
            }
        });
        let wdtw_clone = Rc::clone(&wdtw);
        wdtw.diff_head_rb.connect_toggled(move |rb| {
            if rb.get_active() {
                wdtw_clone.update();
            }
        });
        let wdtw_clone = Rc::clone(&wdtw);
        wdtw.exec_console.event_notifier.add_notification_cb(
            events::EV_AUTO_UPDATE | events::EV_CHECKOUT | events::EV_FILES_CHANGE,
            Box::new(move |_| {
                wdtw_clone.update();
            }),
        );
        let wdtw_clone = Rc::clone(&wdtw);
        wdtw.exec_console.event_notifier.add_notification_cb(
            events::EV_CHANGE_DIR,
            Box::new(move |_| {
                wdtw_clone.repopulate();
            }),
        );

        wdtw
    }

    fn get_diff_text(&self) -> (String, Vec<u8>) {
        let mut cmd = Command::new("git");
        cmd.arg("diff").arg("--no-ext-diff").arg("-M");
        if self.diff_staged_rb.get_active() {
            cmd.arg("--staged");
        } else if self.diff_head_rb.get_active() {
            cmd.arg("HEAD");
        }
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
            Ok(ref diff_pluses) => self.diff_notebook.repopulate(&diff_pluses),
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
                Ok(ref diff_pluses) => self.diff_notebook.update(&diff_pluses),
                Err(err) => {
                    self.diff_notebook.update(&vec![]);
                    self.report_error("Malformed diff text", &err);
                }
            }
        }
    }
}
