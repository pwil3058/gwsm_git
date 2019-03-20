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

use pw_gix::wrapper::*;

use crate::events;
use crate::exec::ExecConsole;

pub struct SimpleRemoteActionButtons {
    h_box: gtk::Box,
    simple_pull_button: gtk::Button,
    simple_push_button: gtk::Button,
    exec_console: Rc<ExecConsole>,
}

impl_widget_wrapper!(h_box: gtk::Box, SimpleRemoteActionButtons);

impl SimpleRemoteActionButtons {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<Self> {
        let srab = Rc::new(Self {
            h_box: gtk::Box::new(gtk::Orientation::Horizontal, 2),
            simple_pull_button: gtk::Button::new_with_label("Pull"),
            simple_push_button: gtk::Button::new_with_label("Push"),
            exec_console: Rc::clone(&exec_console),
        });

        srab.simple_pull_button.set_tooltip_text("Initiate a simple default \"git pull\" operation");
        let srab_clone = Rc::clone(&srab);
        srab.simple_pull_button.connect_clicked(move |_| {
            let cmd = "git pull";
            let cursor = srab_clone.show_busy();
            let result = srab_clone.exec_console.exec_cmd(&cmd, events::EV_PULL);
            srab_clone.unshow_busy(cursor);
            srab_clone.report_any_command_problems(&cmd, &result);
        });

        srab.simple_push_button.set_tooltip_text("Initiate a simple default \"git push\" operation");
        let srab_clone = Rc::clone(&srab);
        srab.simple_push_button.connect_clicked(move |_| {
            let cmd = "git push";
            let cursor = srab_clone.show_busy();
            let result = srab_clone.exec_console.exec_cmd(&cmd, events::EV_PULL);
            srab_clone.unshow_busy(cursor);
            srab_clone.report_any_command_problems(&cmd, &result);
        });

        srab.h_box.pack_start(&srab.simple_pull_button, false, false, 0);
        srab.h_box.pack_start(&srab.simple_push_button, false, false, 0);
        srab.h_box.show_all();

        srab
    }
}
