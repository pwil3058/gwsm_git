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

use std::process::Command;
use std::rc::Rc;

use pw_gix::{
    gtk::{self, prelude::*},
    sourceview::{self, ViewExt},
    wrapper::*,
};

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

pub fn last_commit_message() -> String {
    let output = Command::new("git")
        .arg("log")
        .arg("-n")
        .arg("1")
        .arg("--pretty=format:%s%n%n%b")
        .output()
        .expect("getting last commit message text failed");
    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        "".to_string()
    }
}

#[derive(PWO)]
pub struct MessageWidget {
    v_box: gtk::Box,
    text_view: sourceview::View,
}

impl MessageWidget {
    pub fn new(title: &str) -> Rc<Self> {
        let mw = Rc::new(Self {
            v_box: gtk::Box::new(gtk::Orientation::Vertical, 0),
            text_view: sourceview::View::new(),
        });

        let h_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        h_box.pack_start(&gtk::Label::new(Some(title)), false, false, 0);
        h_box.pack_start(
            &gtk::Box::new(gtk::Orientation::Horizontal, 0),
            true,
            true,
            0,
        );
        mw.v_box.pack_start(&h_box, false, false, 0);

        mw.text_view.set_monospace(true);
        mw.text_view.set_show_right_margin(true);
        mw.text_view.set_right_margin_position(71);
        mw.text_view.connect_populate_popup(|view, widget| {
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
        scrolled_window.add(&mw.text_view);
        mw.v_box.pack_start(&scrolled_window, true, true, 0);

        mw
    }

    pub fn get_message(&self) -> Option<String> {
        let buffer = self.text_view.get_buffer().expect("get_buffer() failed");
        let start = buffer.get_start_iter();
        let end = buffer.get_end_iter();
        let text = buffer
            .get_text(&start, &end, false)
            .expect("get_text() failed");
        if text.len() > 0 {
            Some(text.to_string())
        } else {
            None
        }
    }
}
