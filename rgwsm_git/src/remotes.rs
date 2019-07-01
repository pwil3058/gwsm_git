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

use std::cell::{Cell, Ref, RefCell};
use std::io::Write;
use std::process::Command;
use std::rc::Rc;

use gtk;
use gtk::prelude::*;

use crypto_hash::{Algorithm, Hasher};
use regex::Regex;

use pw_gix::gtkx::list_store::{
    BufferedUpdate, MapManagedUpdate, RequiredMapAction, Row, RowBuffer, RowBufferCore,
};
use pw_gix::gtkx::menu::ManagedMenu;
use pw_gix::sav_state::*;
use pw_gix::wrapper::*;

use crate::events;
use crate::exec::ExecConsole;
use crate::repos;

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

        srab.simple_pull_button
            .set_tooltip_text("Initiate a simple default \"git pull\" operation");
        srab.exec_console.managed_buttons.add_widget(
            "simple pull",
            &srab.simple_pull_button,
            repos::SAV_IN_REPO,
        );
        let srab_clone = Rc::clone(&srab);
        srab.simple_pull_button.connect_clicked(move |_| {
            let cmd = "git pull";
            let cursor = srab_clone.show_busy();
            let result = srab_clone.exec_console.exec_cmd(&cmd, events::EV_PULL);
            srab_clone.unshow_busy(cursor);
            srab_clone.report_any_command_problems(&cmd, &result);
        });

        srab.simple_push_button
            .set_tooltip_text("Initiate a simple default \"git push\" operation");
        srab.exec_console.managed_buttons.add_widget(
            "simple push",
            &srab.simple_push_button,
            repos::SAV_IN_REPO,
        );
        let srab_clone = Rc::clone(&srab);
        srab.simple_push_button.connect_clicked(move |_| {
            let cmd = "git push";
            let cursor = srab_clone.show_busy();
            let result = srab_clone.exec_console.exec_cmd(&cmd, events::EV_PULL);
            srab_clone.unshow_busy(cursor);
            srab_clone.report_any_command_problems(&cmd, &result);
        });

        srab.h_box
            .pack_start(&srab.simple_pull_button, false, false, 0);
        srab.h_box
            .pack_start(&srab.simple_push_button, false, false, 0);
        srab.h_box.show_all();

        srab
    }
}

fn get_raw_data() -> (String, Vec<u8>) {
    let mut hasher = Hasher::new(Algorithm::SHA256);
    let text: String;
    let output = Command::new("git")
        .arg("remote")
        .arg("-v")
        .output()
        .expect("getting all remotes text failed");
    if output.status.success() {
        hasher.write_all(&output.stdout).expect("hasher blew up!!!");
        text = String::from_utf8_lossy(&output.stdout).to_string();
    } else {
        text = "".to_string();
    }
    (text, hasher.finish())
}

lazy_static! {
    static ref VREMOTE_RE: Regex = Regex::new(r"(\S+)\s+(\S+)\s*(\S*)").unwrap();
}

struct RemotesRowBuffer {
    row_buffer_core: Rc<RefCell<RowBufferCore<String>>>,
}

impl RemotesRowBuffer {
    fn new() -> Self {
        let core = RowBufferCore::<String>::default();
        let buffer = Self {
            row_buffer_core: Rc::new(RefCell::new(core)),
        };
        buffer.init();
        buffer
    }
}

impl RowBuffer<String> for RemotesRowBuffer {
    fn get_core(&self) -> Rc<RefCell<RowBufferCore<String>>> {
        self.row_buffer_core.clone()
    }

    fn set_raw_data(&self) {
        let (raw_data, digest) = get_raw_data();
        let mut core = self.row_buffer_core.borrow_mut();
        core.set_raw_data(raw_data, digest);
    }

    fn finalise(&self) {
        let mut rows: Vec<Row> = Vec::new();
        {
            let core = self.row_buffer_core.borrow();
            let mut name: &str = "";
            let mut inbound_url: &str = "";
            for (i, line) in core.raw_data.lines().enumerate() {
                let captures = VREMOTE_RE.captures(&line).unwrap();
                if i % 2 == 0 {
                    name = captures.get(1).unwrap().as_str();
                    inbound_url = captures.get(2).unwrap().as_str();
                } else {
                    let outbound_url = captures.get(2).unwrap().as_str();
                    let row = vec![
                        name.to_value(),
                        inbound_url.to_value(),
                        outbound_url.to_value(),
                    ];
                    rows.push(row);
                }
            }
        }
        let mut core = self.row_buffer_core.borrow_mut();
        core.rows = Rc::new(rows);
        core.set_is_finalised_true();
    }
}

struct RemotesNameListStore {
    list_store: gtk::ListStore,
    remotes_row_buffer: Rc<RefCell<RemotesRowBuffer>>,
}

impl BufferedUpdate<String, gtk::ListStore> for RemotesNameListStore {
    fn get_list_store(&self) -> gtk::ListStore {
        self.list_store.clone()
    }

    fn get_row_buffer(&self) -> Rc<RefCell<dyn RowBuffer<String>>> {
        self.remotes_row_buffer.clone()
    }
}

impl RemotesNameListStore {
    pub fn new() -> RemotesNameListStore {
        Self {
            list_store: gtk::ListStore::new(&[gtk::Type::String; 3]),
            remotes_row_buffer: Rc::new(RefCell::new(RemotesRowBuffer::new())),
        }
    }
}

pub struct RemotesNameTable {
    scrolled_window: gtk::ScrolledWindow,
    view: gtk::TreeView,
    list_store: RefCell<RemotesNameListStore>,
    required_map_action: Cell<RequiredMapAction>,
    exec_console: Rc<ExecConsole>,
    popup_menu: ManagedMenu,
    hovered_remote: RefCell<Option<String>>,
}

impl_widget_wrapper!(scrolled_window: gtk::ScrolledWindow, RemotesNameTable);

impl MapManagedUpdate<RemotesNameListStore, String, gtk::ListStore> for RemotesNameTable {
    fn buffered_update(&self) -> Ref<'_, RemotesNameListStore> {
        self.list_store.borrow()
    }

    fn is_mapped(&self) -> bool {
        self.view.get_mapped()
    }

    fn get_required_map_action(&self) -> RequiredMapAction {
        self.required_map_action.get()
    }

    fn set_required_map_action(&self, action: RequiredMapAction) {
        self.required_map_action.set(action);
    }
}

impl RemotesNameTable {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<RemotesNameTable> {
        let list_store = RefCell::new(RemotesNameListStore::new());

        let view = gtk::TreeView::new_with_model(&list_store.borrow().get_list_store());
        view.set_headers_visible(true);

        view.get_selection().set_mode(gtk::SelectionMode::Single);

        let col = gtk::TreeViewColumn::new();
        col.set_title("Name");
        col.set_expand(false);
        col.set_resizable(false);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", 0);

        view.append_column(&col);

        let col = gtk::TreeViewColumn::new();
        col.set_title("Inbound URL");
        col.set_expand(false);
        col.set_resizable(false);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", 1);

        view.append_column(&col);

        let col = gtk::TreeViewColumn::new();
        col.set_title("Outbound URL");
        col.set_expand(false);
        col.set_resizable(false);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", 2);

        view.append_column(&col);

        view.show_all();

        list_store.borrow().repopulate();

        let required_map_action = Cell::new(RequiredMapAction::Nothing);

        let popup_menu = ManagedMenu::new(
            WidgetStatesControlled::Sensitivity,
            Some(&view.get_selection()),
            Some(&exec_console.changed_condns_notifier),
            &vec![],
        );

        let adj: Option<&gtk::Adjustment> = None;
        let scrolled_window = gtk::ScrolledWindow::new(adj, adj);
        scrolled_window.add(&view);

        let table = Rc::new(RemotesNameTable {
            scrolled_window,
            view,
            list_store,
            required_map_action,
            exec_console: Rc::clone(exec_console),
            popup_menu: popup_menu,
            hovered_remote: RefCell::new(None),
        });
        let table_clone = Rc::clone(&table);
        table.exec_console.event_notifier.add_notification_cb(
            events::EV_AUTO_UPDATE | events::EV_REMOTES_CHANGE,
            Box::new(move |_| table_clone.auto_update()),
        );
        let table_clone = Rc::clone(&table);
        table.view.connect_map(move |_| table_clone.on_map_action());
        let table_clone = Rc::clone(&table);
        table.exec_console.event_notifier.add_notification_cb(
            events::EV_CHANGE_DIR,
            Box::new(move |_| table_clone.repopulate()),
        );

        let table_clone = table.clone();
        table.view.connect_button_press_event(move |view, event| {
            if event.get_button() == 3 {
                let remote = get_row_item_for_event!(view, event, String, 0);
                table_clone.set_hovered_remote(remote);
                table_clone.popup_menu.popup_at_event(event);
                return Inhibit(true);
            } else if event.get_button() == 2 {
                table_clone.view.get_selection().unselect_all();
                return Inhibit(true);
            }
            Inhibit(false)
        });

        table
    }

    fn set_hovered_remote(&self, remote: Option<String>) {
        let condns = self
            .view
            .get_selection()
            .get_masked_conditions_with_hover_ok(remote.is_some());
        self.popup_menu.update_condns(condns);
        *self.hovered_remote.borrow_mut() = remote;
    }
}
