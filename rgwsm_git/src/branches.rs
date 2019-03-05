//Copyright 2019 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>
//
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
//limitations under the License.

use std::cell::{Cell, Ref, RefCell};
use std::collections::HashSet;
use std::io::Write;
use std::process::Command;
use std::rc::Rc;

use gtk::prelude::*;

use crypto_hash::{Algorithm, Hasher};
use regex::Regex;

use pw_gix::gtkx::list_store::{
    BufferedUpdate, MapManagedUpdate, RequiredMapAction, Row, RowBuffer, RowBufferCore,
};
use pw_gix::timeout;
use pw_gix::wrapper::*;

use crate::events::{self, EventNotifier};

#[derive(Debug, Default)]
struct BranchesRawData {
    all_branches_text: String,
    merged_branches_text: String,
}

fn get_raw_data() -> (BranchesRawData, Vec<u8>) {
    let mut hasher = Hasher::new(Algorithm::SHA256);
    let abt_text: String;
    let abt_output = Command::new("git")
        .arg("branch")
        .arg("-vv")
        .output()
        .expect("getting all branches text failed");
    if abt_output.status.success() {
        hasher
            .write_all(&abt_output.stdout)
            .expect("hasher blew up!!!");
        abt_text = String::from_utf8_lossy(&abt_output.stdout).to_string();
    } else {
        abt_text = "".to_string();
    }
    let mbt_text: String;
    let mbt_output = Command::new("git")
        .arg("branch")
        .arg("--merged")
        .output()
        .expect("getting merged branches text failed");
    if mbt_output.status.success() {
        hasher
            .write_all(&mbt_output.stdout)
            .expect("hasher blew up!!!");
        mbt_text = String::from_utf8_lossy(&mbt_output.stdout).to_string();
    } else {
        mbt_text = "".to_string();
    }
    let raw_data = BranchesRawData {
        all_branches_text: abt_text,
        merged_branches_text: mbt_text,
    };
    (raw_data, hasher.finish())
}

lazy_static! {
    static ref ALL_BRANCHES_RE: Regex =
        Regex::new(r"(([^ (]+)|(\([^)]+\)))\s+([a-fA-F0-9]{7}[a-fA-F0-9]*)?\s*([^\s].*)").unwrap();
}

fn extract_branch_row(line: &str, merged_set: &HashSet<&str>) -> Row {
    let is_current = line.starts_with("*");
    let captures = ALL_BRANCHES_RE.captures(&line[2..]).unwrap();
    let name = captures.get(1).unwrap().as_str();
    let rev = captures.get(4).unwrap().as_str();
    let synopsis = captures.get(5).unwrap().as_str();
    let is_merged = merged_set.contains(name);
    let mut v = vec![];
    v.push(name.to_value());
    if is_current {
        v.push("<b><span foreground=\"green\">*</span></b>".to_value());
        v.push(format!("<b><span foreground=\"green\">{}</span></b>", name).to_value());
    } else if is_merged {
        v.push("".to_value());
        v.push(format!("<span foreground=\"green\">{}</span>", name).to_value());
    } else {
        v.push("".to_value());
        v.push(name.to_value());
    }
    v.push(rev.to_value());
    v.push(synopsis.to_value());
    v
}

struct BranchesRowBuffer {
    row_buffer_core: Rc<RefCell<RowBufferCore<BranchesRawData>>>,
}

impl BranchesRowBuffer {
    fn new() -> Self {
        let core = RowBufferCore::<BranchesRawData>::default();
        let buffer = Self {
            row_buffer_core: Rc::new(RefCell::new(core)),
        };
        buffer.init();
        buffer
    }
}

impl RowBuffer<BranchesRawData> for BranchesRowBuffer {
    fn get_core(&self) -> Rc<RefCell<RowBufferCore<BranchesRawData>>> {
        self.row_buffer_core.clone()
    }

    fn set_raw_data(&self) {
        let (raw_data, digest) = get_raw_data();
        let mut core = self.row_buffer_core.borrow_mut();
        core.set_raw_data(raw_data, digest);
    }

    fn finalise(&self) {
        let mut core = self.row_buffer_core.borrow_mut();
        let mut merged_set: HashSet<&str> = HashSet::new();
        for line in core.raw_data.merged_branches_text.lines() {
            merged_set.insert(line[2..].trim_right());
        }
        let mut rows: Vec<Row> = Vec::new();
        for line in core.raw_data.all_branches_text.lines() {
            rows.push(extract_branch_row(&line, &merged_set))
        }
        core.rows = Rc::new(rows);
        core.set_is_finalised_true();
    }
}

struct BranchesNameListStore {
    list_store: gtk::ListStore,
    branches_row_buffer: Rc<RefCell<BranchesRowBuffer>>,
}

impl BufferedUpdate<BranchesRawData, gtk::ListStore> for BranchesNameListStore {
    fn get_list_store(&self) -> gtk::ListStore {
        self.list_store.clone()
    }

    fn get_row_buffer(&self) -> Rc<RefCell<RowBuffer<BranchesRawData>>> {
        self.branches_row_buffer.clone()
    }
}

impl BranchesNameListStore {
    pub fn new() -> BranchesNameListStore {
        Self {
            list_store: gtk::ListStore::new(&[gtk::Type::String; 5]),
            branches_row_buffer: Rc::new(RefCell::new(BranchesRowBuffer::new())),
        }
    }
}

pub struct BranchesNameTable {
    view: gtk::TreeView,
    list_store: RefCell<BranchesNameListStore>,
    required_map_action: Cell<RequiredMapAction>,
    controlled_timeout_cycle: Rc<timeout::ControlledTimeoutCycle>,
}

impl_widget_wrapper!(view: gtk::TreeView, BranchesNameTable);

impl MapManagedUpdate<BranchesNameListStore, BranchesRawData, gtk::ListStore>
    for BranchesNameTable
{
    fn buffered_update(&self) -> Ref<BranchesNameListStore> {
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

impl BranchesNameTable {
    pub fn new(event_notifier: Option<&Rc<EventNotifier>>) -> Rc<BranchesNameTable> {
        let list_store = RefCell::new(BranchesNameListStore::new());

        let view = gtk::TreeView::new_with_model(&list_store.borrow().get_list_store());
        view.set_headers_visible(true);

        view.get_selection().set_mode(gtk::SelectionMode::Multiple);

        let col = gtk::TreeViewColumn::new();
        col.set_title("Name");
        col.set_expand(false);
        col.set_resizable(false);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "markup", 1);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "markup", 2);

        view.append_column(&col);

        let col = gtk::TreeViewColumn::new();
        col.set_title("Rev");
        col.set_expand(false);
        col.set_resizable(false);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", 3);

        view.append_column(&col);

        let col = gtk::TreeViewColumn::new();
        col.set_title("Synopsis");
        col.set_expand(false);
        col.set_resizable(false);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", 4);

        view.append_column(&col);

        view.show_all();

        list_store.borrow().repopulate();

        let controlled_timeout_cycle =
            timeout::ControlledTimeoutCycle::new("Auto Update", true, 10);

        let required_map_action = Cell::new(RequiredMapAction::Nothing);

        let table = Rc::new(BranchesNameTable {
            view,
            list_store,
            required_map_action,
            controlled_timeout_cycle,
        });
        let table_clone = Rc::clone(&table);
        table
            .controlled_timeout_cycle
            .register_callback(Box::new(move || table_clone.auto_update()));
        let table_clone = Rc::clone(&table);
        table.view.connect_map(move |_| table_clone.on_map_action());
        if let Some(event_notifier) = event_notifier {
            let table_clone = Rc::clone(&table);
            event_notifier.add_notification_cb(
                events::EV_CHANGE_DIR,
                Box::new(move |_| { table_clone.repopulate() })
            );
        }

        table
    }
}
