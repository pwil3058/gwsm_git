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

extern crate crypto_hash;
#[macro_use]
extern crate lazy_static;

extern crate gio;
extern crate gtk;
extern crate pango_sys;

#[macro_use]
extern crate pw_gix;
extern crate pw_pathux;

use gio::ApplicationExt;
use gio::ApplicationExtManual;
use gtk::prelude::*;

use pw_gix::file_tree::FileTreeIfce;
use pw_gix::gdkx::format_geometry;
use pw_gix::recollections;
use pw_gix::wrapper::*;

use pw_pathux::str_path::str_path_current_dir_or_panic;

mod config;
mod fs_db;
mod ws_file_tree;

fn activate(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    // TODO: mechansim to change title when directory changes
    window.set_title(&format!("gwsm_git: {}", str_path_current_dir_or_panic()));
    if let Some(geometry) = recollections::recall("main_window:geometry") {
        window.parse_geometry(&geometry);
    } else {
        window.set_default_size(200, 200);
    };
    window.connect_configure_event(|_, event| {
        recollections::remember("main_window:geometry", &format_geometry(event));
        false
    });
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let label = gtk::Label::new("GUI is under construction");
    vbox.pack_start(&label, true, true, 0);
    let ws_file_tree = ws_file_tree::OsWsFsTree::new(false);
    vbox.pack_start(&ws_file_tree.pwo(), true, true, 0);
    window.add(&vbox);
    window.show_all();
}

fn main() {
    recollections::init(&config::get_config_dir_path().join("recollections"));
    let flags = gio::ApplicationFlags::empty();
    let app = gtk::Application::new("gergibus.pw.nest", flags)
        .unwrap_or_else(|err| panic!("{:?}: line {:?}: {:?}", file!(), line!(), err));
    app.connect_activate(activate);
    app.run(&[]);
}
