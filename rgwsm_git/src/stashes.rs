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

use std::rc::Rc;

use gtk;
use gtk::prelude::*;

use shlex;

use pw_gix::gtkx::dialog::*;
use pw_gix::wrapper::*;

use crate::action_icons;
use crate::events;
use crate::exec::{self, ExecConsole};

pub struct StashPushWidget {
    v_box: gtk::Box,
    keep_index_ch_btn: gtk::CheckButton,
    include_untracked_ch_btn: gtk::CheckButton,
    all_ch_btn: gtk::CheckButton,
    text_view: gtk::TextView,
}

impl_widget_wrapper!(v_box: gtk::Box, StashPushWidget);

impl StashPushWidget {
    pub fn new() -> Rc<Self> {
        let ctw = Rc::new(Self {
            v_box: gtk::Box::new(gtk::Orientation::Vertical, 0),
            keep_index_ch_btn: gtk::CheckButton::new_with_label("--keep-index"),
            include_untracked_ch_btn: gtk::CheckButton::new_with_label("--include-untracked"),
            all_ch_btn: gtk::CheckButton::new_with_label("--all"),
            text_view: gtk::TextView::new(),
        });

        ctw.v_box.pack_start(&ctw.keep_index_ch_btn, false, false, 0);
        ctw.v_box.pack_start(&ctw.include_untracked_ch_btn, false, false, 0);
        ctw.v_box.pack_start(&ctw.all_ch_btn, false, false, 0);
        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        h_box.pack_start(&gtk::Label::new("Message"), false, false, 0);
        ctw.v_box.pack_start(&h_box, false, false, 0);
        let adj: Option<&gtk::Adjustment> = None;
        let scrolled_window = gtk::ScrolledWindow::new(adj, adj);
        scrolled_window.add(&ctw.text_view);
        ctw.v_box.pack_start(&scrolled_window, true, true, 0);
        ctw.v_box.show_all();

        ctw
    }

    pub fn get_message(&self) -> Option<String> {
        let buffer = self.text_view.get_buffer()
            .expect("get_buffer() failed");
        let start = buffer.get_start_iter();
        let end = buffer.get_end_iter();
        if let Some(text) = buffer.get_text(&start, &end, false) {
            if text.len() > 0 {
                Some(text.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct StashPushButton {
    button: gtk::Button,
    exec_console: Rc<ExecConsole>,
}

impl_widget_wrapper!(button: gtk::Button, StashPushButton);

impl StashPushButton {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<Self> {
        let button = gtk::Button::new();
        button.set_tooltip_text(Some(
            "Push the current state on to the stash stack",
        ));
        button.set_image(&action_icons::stash_push_image(32));
        button.set_image_position(gtk::PositionType::Top);
        button.set_label("stash");
        exec_console
            .managed_buttons
            .add_widget("stash push", &button, exec::SAV_IN_REPO);
        let bb = Rc::new(Self {
            button: button,
            exec_console: Rc::clone(&exec_console),
        });

        let bb_clone = Rc::clone(&bb);
        bb.button
            .connect_clicked(move |_| bb_clone.stash_push_cb());

        bb
    }

    fn stash_push_cb(&self) {
        let dialog = self.new_dialog_with_buttons(
            Some("Stash Current State"),
            gtk::DialogFlags::DESTROY_WITH_PARENT | gtk::DialogFlags::MODAL,
            CANCEL_OK_BUTTONS,
        );
        dialog.set_default_response(gtk::ResponseType::Ok);
        dialog.set_size_from_recollections("create:stash:dialog", (600, 330));
        let stash_push_widget = StashPushWidget::new();
        dialog.get_content_area().pack_start(&stash_push_widget.pwo(), true, true, 0);
        dialog.get_content_area().show_all();
        let result = dialog.run();
        dialog.hide();
        if gtk::ResponseType::from(result) == gtk::ResponseType::Ok {
            let mut cmd = "git stash push".to_string();
            if stash_push_widget.keep_index_ch_btn.get_active() {
                cmd.push_str(" --keep-index");
            }
            if stash_push_widget.include_untracked_ch_btn.get_active() {
                cmd.push_str(" --include-untracked");
            }
            if stash_push_widget.all_ch_btn.get_active() {
                cmd.push_str(" --all");
            }
            if let Some(text) = stash_push_widget.get_message() {
                cmd.push_str(&format!(" -m {}", shlex::quote(&text)));
            }
            let cursor = self.show_busy();
            let result = self.exec_console.exec_cmd(&cmd, events::EV_STASHES_CHANGE + events::EV_FILES_CHANGE);
            self.unshow_busy(cursor);
            self.report_any_command_problems(&cmd, &result);
        }
        dialog.destroy();
    }
}
