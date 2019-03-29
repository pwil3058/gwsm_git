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
extern crate glob;
extern crate ignore;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde_json;
extern crate shlex;

extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;
extern crate pango_sys;
extern crate sourceview;

extern crate cub_diff_gui_lib;
extern crate cub_diff_lib;
#[macro_use]
extern crate pw_gix;
extern crate pw_pathux;

use gio::ApplicationExt;
use gio::ApplicationExtManual;
use gtk::prelude::MenuShellExt;
use gtk::prelude::*;

use pw_gix::gdkx::format_geometry;
use pw_gix::gtkx::paned::RememberPosition;
use pw_gix::recollections;
use pw_gix::wrapper::*;

mod action_icons;
mod branches;
mod commit;
mod config;
mod diff;
mod edit;
mod events;
mod exec;
mod fs_db;
mod icon;
mod index_file_tree;
mod remotes;
mod repos;
mod stashes;
mod submodules;
mod ws_file_tree;

fn activate(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    // TODO: mechansim to change title when directory changes
    window.set_title(&config::window_title(None));
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
    if !exec.in_repo() {
        if let Some(last_ws_dir) = recollections::recall("last:git:ws:dir") {
            exec.chdir(&last_ws_dir);
        }
    }
    let w = window.clone();
    exec.event_notifier.add_notification_cb(
        events::EV_CHANGE_DIR,
        Box::new(move |_| w.set_title(&config::window_title(None))),
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
    hbox.pack_start(&menu_bar, true, true, 0);
    menu_bar.add(&menu_item);
    menu.show_all();
    menu_item.show_all();

    let config_menu = gtk::Menu::new();
    let editor_assignment_menu_item = edit::EditorAlocationMenuItem::new();
    editor_assignment_menu_item.pwo().show_all();
    config_menu.append(&editor_assignment_menu_item.pwo());
    let auto_update_check_item = exec.auto_update_check_item();
    auto_update_check_item.show_all();
    config_menu.append(&auto_update_check_item);
    let config_menu_item = gtk::MenuItem::new_with_label("Configuration");
    config_menu_item.set_submenu(&config_menu);
    let r_menu_bar = gtk::MenuBar::new();
    hbox.pack_end(&r_menu_bar, false, false, 0);
    r_menu_bar.add(&config_menu_item);
    config_menu.show_all();
    config_menu_item.show_all();

    hbox.show_all();

    let action_hbox = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    vbox.pack_start(&action_hbox, false, false, 0);
    let submodule_parent_button = submodules::SubmoduleParentButton::new(&exec);
    action_hbox.pack_start(&submodule_parent_button.pwo(), false, false, 0);
    action_hbox.pack_start(
        &gtk::Separator::new(gtk::Orientation::Vertical),
        false,
        false,
        1,
    );
    let diff_button = diff::DiffButton::new(&exec);
    action_hbox.pack_start(&diff_button.pwo(), false, false, 0);
    let commit_button = commit::CommitButton::new(&exec);
    action_hbox.pack_start(&commit_button.pwo(), false, false, 0);
    action_hbox.pack_start(
        &gtk::Separator::new(gtk::Orientation::Vertical),
        false,
        false,
        1,
    );
    let branch_button = branches::BranchButton::new(&exec);
    action_hbox.pack_start(&branch_button.pwo(), false, false, 0);
    action_hbox.pack_start(
        &gtk::Separator::new(gtk::Orientation::Vertical),
        false,
        false,
        1,
    );
    let stash_push_button = stashes::StashPushButton::new(&exec);
    action_hbox.pack_start(&stash_push_button.pwo(), false, false, 0);
    action_hbox.pack_start(
        &gtk::Separator::new(gtk::Orientation::Vertical),
        false,
        false,
        1,
    );
    let simple_remote_buttons = remotes::SimpleRemoteActionButtons::new(&exec);
    action_hbox.pack_start(&simple_remote_buttons.pwo(), false, false, 0);
    action_hbox.pack_end(&exec.update_button, false, false, 0);

    let label = gtk::Label::new("GUI is still under construction");
    vbox.pack_start(&label, false, false, 0);
    let paned_h_1 = gtk::Paned::new(gtk::Orientation::Horizontal);
    let ws_file_tree = ws_file_tree::GitWsFsTree::new(&exec, false);
    let index_file_tree = index_file_tree::GitIndexFsTree::new(&exec);
    paned_h_1.add1(&ws_file_tree.pwo());
    paned_h_1.add2(&index_file_tree.pwo());
    paned_h_1.show_all();
    paned_h_1.set_position_from_recollections("paned_h_1:position", 200);
    let paned_h_2 = gtk::Paned::new(gtk::Orientation::Horizontal);
    paned_h_2.add1(&paned_h_1);
    let notebook = gtk::Notebook::new();
    let branches_table = branches::BranchesNameTable::new(&exec);
    notebook.add(&branches_table.pwo());
    notebook.set_tab_label_text(&branches_table.pwo(), "Branches");
    let stashes_table = stashes::StashesNameTable::new(&exec);
    notebook.add(&stashes_table.pwo());
    notebook.set_tab_label_text(&stashes_table.pwo(), "Stashes");
    let remotes_table = remotes::RemotesNameTable::new(&exec);
    notebook.add(&remotes_table.pwo());
    notebook.set_tab_label_text(&remotes_table.pwo(), "Remotes");
    notebook.add(&gtk::Label::new("Tags will go here!!"));
    notebook.set_property_enable_popup(true);
    paned_h_2.add2(&notebook);
    paned_h_2.show_all();
    let paned_v = gtk::Paned::new(gtk::Orientation::Vertical);
    paned_v.add1(&paned_h_2);
    vbox.pack_start(&paned_v, true, true, 0);
    paned_h_2.set_position_from_recollections("paned_h_2:position", 200);
    paned_v.set_position_from_recollections("paned_v:position", 200);
    let notebook = gtk::Notebook::new();
    notebook.add(&exec.pwo());
    notebook.set_tab_label_text(&exec.pwo(), "Transaction Log");
    notebook.add(&gtk::Label::new("Vte terminal will go here!!"));
    notebook.set_property_enable_popup(true);
    paned_v.add2(&notebook);
    window.add(&vbox);
    window.show_all();
}

fn main() {
    recollections::init(&config::get_config_dir_path().join("recollections"));
    edit::init_editor_assignment_table();
    let flags = gio::ApplicationFlags::empty();
    let app = gtk::Application::new("gergibus.pw.nest", flags)
        .unwrap_or_else(|err| panic!("{:?}: line {:?}: {:?}", file!(), line!(), err));
    app.connect_activate(activate);
    app.run(&[]);
}
