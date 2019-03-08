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
use pw_pathux::str_path;

use crate::action_icons;
use crate::exec::{self, ExecConsole};

pub struct DiffButton {
    button: gtk::Button,
    exec_console: Rc<ExecConsole>,
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
            exec_console: Rc::clone(&exec_console),
        });

        let db_clone = Rc::clone(&db);
        db.button.connect_clicked(move |_| db_clone.show_diff_cb());

        db
    }

    fn show_diff_cb(&self) {
        let title = format!(
            "diff: {}",
            str_path::str_path_current_dir_or_rel_home_panic()
        );
        let dialog = self.new_dialog_with_buttons(
            Some(&title),
            gtk::DialogFlags::DESTROY_WITH_PARENT,
            &[("Close", gtk::ResponseType::Close)],
        );
        let wdtw = WdDiffTextWidget::new();
        dialog
            .get_content_area()
            .pack_start(&wdtw.pwo(), false, false, 0);
        dialog.get_content_area().show_all();
        dialog.set_default_response(gtk::ResponseType::Close);
        dialog.connect_response(|dialog, _| dialog.destroy());
        dialog.show()
    }
}

struct WdDiffTextWidget {
    v_box: gtk::Box,
    diff_rb: gtk::RadioButton,
    diff_staged_rb: gtk::RadioButton,
    diff_head_rb: gtk::RadioButton,
}

impl_widget_wrapper!(v_box: gtk::Box, WdDiffTextWidget);

impl WdDiffTextWidget {
    fn new() -> Rc<Self> {
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
        v_box.pack_start(&gtk::Label::new("diff notebook goes here"), true, true, 0);
        let wdtw = Rc::new(Self {
            v_box,
            diff_rb,
            diff_staged_rb,
            diff_head_rb,
        });
        // NB: only update when active to stop double update
        wdtw.diff_rb.connect_toggled(|rb| {
            if rb.get_active() {
                println!("update")
            }
        });
        wdtw.diff_staged_rb.connect_toggled(|rb| {
            if rb.get_active() {
                println!("update")
            }
        });
        wdtw.diff_head_rb.connect_toggled(|rb| {
            if rb.get_active() {
                println!("update")
            }
        });

        wdtw
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        assert!(false);
    }
}