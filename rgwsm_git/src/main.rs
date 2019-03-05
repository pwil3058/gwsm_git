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

extern crate chrono;
extern crate crypto_hash;
extern crate git2;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate shlex;

extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;
extern crate pango_sys;

#[macro_use]
extern crate pw_gix;
extern crate pw_pathux;

use gio::ApplicationExt;
use gio::ApplicationExtManual;
use gtk::prelude::*;
use gtk::prelude::MenuShellExt;

use pw_gix::file_tree::FileTreeIfce;
use pw_gix::gdkx::format_geometry;
use pw_gix::gtkx::paned::RememberPosition;
use pw_gix::recollections;
use pw_gix::wrapper::*;

use pw_pathux::str_path::str_path_current_dir_or_panic;

mod branches;
mod config;
mod exec;
mod events;
mod fs_db;
mod icon;
mod submodules;
mod ws_file_tree;

fn activate(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    // TODO: mechansim to change title when directory changes
    window.set_title(&format!("gwsm_git: {}", str_path_current_dir_or_panic()));
    let app_icon = icon::rgwsmgit_pixbuf();
    window.set_icon(Some(&app_icon));
    if let Some(geometry) = recollections::recall("main_window:geometry") {
        window.parse_geometry(&geometry);
    } else {
        window.set_default_size(200, 200);
    };
    window.connect_configure_event(|_, event| {
        recollections::remember("main_window:geometry", &format_geometry(event));
        false
    });
    let exec = exec::ExecConsole::new();
    let w = window.clone();
    exec.event_notifier.add_notification_cb(
        events::EV_CHANGE_DIR,
        Box::new(move |_| {w.set_title(&format!("gwsm_git: {}", str_path_current_dir_or_panic()))})
    );
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    vbox.pack_start(&hbox, false, false, 0);
    let menu = gtk::Menu::new();
    menu.append(&exec.chdir_menu_item);
    exec.chdir_menu_item.show();
    let menu_item = gtk::MenuItem::new_with_label("Files");
    menu_item.set_submenu(&menu);
    let menu_bar = gtk::MenuBar::new();
    menu_bar.show();
    hbox.pack_start(&menu_bar, false, false, 0);
    menu_bar.add(&menu_item);
    menu.show_all();
    menu_item.show_all();
    hbox.show_all();
    let label = gtk::Label::new("GUI is under construction");
    vbox.pack_start(&label, false, false, 0);
    let paned_h = gtk::Paned::new(gtk::Orientation::Horizontal);
    let ws_file_tree = ws_file_tree::GitWsFsTree::new(false);
    paned_h.add1(&ws_file_tree.pwo());
    let notebook = gtk::Notebook::new();
    let branches_table = branches::BranchesNameTable::new();
    notebook.add(&branches_table.pwo());
    notebook.set_tab_label_text(&branches_table.pwo(), "Branches");
    notebook.add(&gtk::Label::new("Tags will go here!!"));
    notebook.set_property_enable_popup(true);
    paned_h.add2(&notebook);
    let paned_v = gtk::Paned::new(gtk::Orientation::Vertical);
    paned_v.add1(&paned_h);
    vbox.pack_start(&paned_v, true, true, 0);
    paned_h.set_position_from_recollections("paned_h:position", 200);
    paned_v.set_position_from_recollections("paned_v:position", 200);
    paned_v.add2(&exec.pwo());
    window.add(&vbox);
    window.show_all();
    exec.exec_cmd("ls", 0);
}

fn main() {
    recollections::init(&config::get_config_dir_path().join("recollections"));
    let flags = gio::ApplicationFlags::empty();
    let app = gtk::Application::new("gergibus.pw.nest", flags)
        .unwrap_or_else(|err| panic!("{:?}: line {:?}: {:?}", file!(), line!(), err));
    app.connect_activate(activate);
    app.run(&[]);
}
