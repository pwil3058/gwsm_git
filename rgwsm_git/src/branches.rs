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
use pw_gix::gtkx::menu::ManagedMenu;
use pw_gix::sav_state::*;
use pw_gix::wrapper::*;

use crate::action_icons;
use crate::events;
use crate::exec::ExecConsole;
use crate::repos;

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
        let mut rows: Vec<Row> = Vec::new();
        {
            let core = self.row_buffer_core.borrow();
            let mut merged_set: HashSet<&str> = HashSet::new();
            for line in core.raw_data.merged_branches_text.lines() {
                merged_set.insert(line[2..].trim_end());
            }
            for line in core.raw_data.all_branches_text.lines() {
                rows.push(extract_branch_row(&line, &merged_set))
            }
        }
        let mut core = self.row_buffer_core.borrow_mut();
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

    fn get_row_buffer(&self) -> Rc<RefCell<dyn RowBuffer<BranchesRawData>>> {
        self.branches_row_buffer.clone()
    }
}

impl BranchesNameListStore {
    pub fn new() -> BranchesNameListStore {
        Self {
            list_store: gtk::ListStore::new(&[glib::Type::String; 5]),
            branches_row_buffer: Rc::new(RefCell::new(BranchesRowBuffer::new())),
        }
    }
}

#[derive(PWO, Wrapper)]
pub struct BranchesNameTable {
    scrolled_window: gtk::ScrolledWindow,
    view: gtk::TreeView,
    list_store: RefCell<BranchesNameListStore>,
    required_map_action: Cell<RequiredMapAction>,
    exec_console: Rc<ExecConsole>,
    popup_menu: ManagedMenu,
    hovered_branch: RefCell<Option<String>>,
}

impl MapManagedUpdate<BranchesNameListStore, BranchesRawData, gtk::ListStore>
    for BranchesNameTable
{
    fn buffered_update(&self) -> Ref<'_, BranchesNameListStore> {
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
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<BranchesNameTable> {
        let list_store = RefCell::new(BranchesNameListStore::new());

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

        let table = Rc::new(BranchesNameTable {
            scrolled_window,
            view,
            list_store,
            required_map_action,
            exec_console: Rc::clone(exec_console),
            popup_menu: popup_menu,
            hovered_branch: RefCell::new(None),
        });
        let table_clone = Rc::clone(&table);
        table.exec_console.event_notifier.add_notification_cb(
            events::EV_AUTO_UPDATE
                | events::EV_BRANCHES_CHANGE
                | events::EV_COMMIT
                | events::EV_PULL
                | events::EV_PUSH,
            Box::new(move |_| table_clone.auto_update()),
        );
        let table_clone = Rc::clone(&table);
        table.view.connect_map(move |_| table_clone.on_map_action());
        let table_clone = Rc::clone(&table);
        table.exec_console.event_notifier.add_notification_cb(
            events::EV_CHANGE_DIR,
            Box::new(move |_| table_clone.repopulate()),
        );

        let table_clone = Rc::clone(&table);
        table
            .popup_menu
            .append_item(
                "checkout",
                "Checkout",
                None,
                "Switch to the selected/indicated branch",
                repos::SAV_IN_REPO + SAV_SELN_UNIQUE_OR_HOVER_OK,
            )
            .connect_activate(move |_| {
                let selection = table_clone.view.get_selection();
                let branch = if let Some((store, iter)) = selection.get_selected() {
                    store.get_value(&iter, 0).get::<String>().unwrap()
                } else {
                    table_clone.hovered_branch.borrow().clone()
                };
                if let Some(branch) = branch {
                    let cmd = format!("git checkout {}", shlex::quote(&branch));
                    let result = table_clone
                        .exec_console
                        .exec_cmd(&cmd, events::EV_BRANCHES_CHANGE | events::EV_CHECKOUT);
                    table_clone.report_any_command_problems(&cmd, &result);
                }
            });

        let table_clone = Rc::clone(&table);
        table
            .popup_menu
            .append_item(
                "merge",
                "Merge",
                None,
                "Merge the selected/indicated branch with the current branch",
                repos::SAV_IN_REPO + SAV_SELN_UNIQUE_OR_HOVER_OK,
            )
            .connect_activate(move |_| {
                let selection = table_clone.view.get_selection();
                let branch = if let Some((store, iter)) = selection.get_selected() {
                    store.get_value(&iter, 0).get::<String>().unwrap()
                } else {
                    table_clone.hovered_branch.borrow().clone()
                };
                if let Some(branch) = branch {
                    let cmd = format!("git merge {}", shlex::quote(&branch));
                    let result = table_clone
                        .exec_console
                        .exec_cmd(&cmd, events::EV_BRANCHES_CHANGE | events::EV_FILES_CHANGE);
                    table_clone.report_any_command_problems(&cmd, &result);
                }
            });

        let table_clone = Rc::clone(&table);
        table
            .popup_menu
            .append_item(
                "delete",
                "Delete",
                None,
                "Delete the selected/indicated branch",
                repos::SAV_IN_REPO + SAV_SELN_UNIQUE_OR_HOVER_OK,
            )
            .connect_activate(move |_| {
                let selection = table_clone.view.get_selection();
                let branch = if let Some((store, iter)) = selection.get_selected() {
                    store.get_value(&iter, 0).get::<String>().unwrap()
                } else {
                    table_clone.hovered_branch.borrow().clone()
                };
                if let Some(branch) = branch {
                    let cmd = format!("git branch -d {}", shlex::quote(&branch));
                    let result = table_clone
                        .exec_console
                        .exec_cmd(&cmd, events::EV_BRANCHES_CHANGE | events::EV_FILES_CHANGE);
                    table_clone.report_any_command_problems(&cmd, &result);
                }
            });

        let table_clone = table.clone();
        table.view.connect_button_press_event(move |view, event| {
            if event.get_button() == 3 {
                let branch = get_row_item_for_event!(view, event, String, 0);
                table_clone.set_hovered_branch(branch);
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

    fn set_hovered_branch(&self, branch: Option<String>) {
        let condns = self
            .view
            .get_selection()
            .get_masked_conditions_with_hover_ok(branch.is_some());
        self.popup_menu.update_condns(condns);
        *self.hovered_branch.borrow_mut() = branch;
    }
}

#[derive(PWO, Wrapper)]
pub struct BranchButton {
    button: gtk::Button,
    exec_console: Rc<ExecConsole>,
}

impl BranchButton {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<Self> {
        let button = gtk::Button::new();
        button.set_tooltip_text(Some(
            "Create a branch based on the current HEAD and (optionally) check it out",
        ));
        button.set_image(Some(&action_icons::branch_image(32)));
        button.set_image_position(gtk::PositionType::Top);
        button.set_label("branch");
        exec_console
            .managed_buttons
            .add_widget("branch", &button, repos::SAV_IN_REPO);
        let bb = Rc::new(Self {
            button: button,
            exec_console: Rc::clone(&exec_console),
        });

        let bb_clone = Rc::clone(&bb);
        bb.button
            .connect_clicked(move |_| bb_clone.create_new_branch_cb());

        bb
    }

    fn create_new_branch_cb(&self) {
        let dialog = self.new_dialog_with_buttons(
            Some("New Branch"),
            gtk::DialogFlags::DESTROY_WITH_PARENT | gtk::DialogFlags::MODAL,
            CANCEL_OK_BUTTONS,
        );
        dialog.set_default_response(gtk::ResponseType::Ok);
        let branch_name = gtk::Entry::new();
        branch_name.set_activates_default(true);
        let checkout_new_branch = gtk::CheckButton::new_with_label("Checkout?");
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        hbox.pack_start(&gtk::Label::new(Some("Name:")), false, false, 0);
        hbox.pack_start(&branch_name, true, true, 0);
        hbox.pack_start(&checkout_new_branch, false, false, 0);
        dialog.get_content_area().pack_start(&hbox, false, false, 0);
        dialog.get_content_area().show_all();
        let result = dialog.run();
        dialog.hide();
        if gtk::ResponseType::from(result) == gtk::ResponseType::Ok {
            if let Some(branch_name) = branch_name.get_text() {
                if checkout_new_branch.get_active() {
                    let cmd = format!("git checkout -b {}", branch_name);
                    let result = self
                        .exec_console
                        .exec_cmd(&cmd, events::EV_BRANCHES_CHANGE | events::EV_CHECKOUT);
                    self.report_any_command_problems(&cmd, &result);
                } else {
                    let cmd = format!("git branch {}", branch_name);
                    let result = self.exec_console.exec_cmd(&cmd, events::EV_BRANCHES_CHANGE);
                    self.report_any_command_problems(&cmd, &result);
                }
            }
        }
        dialog.destroy();
    }
}
