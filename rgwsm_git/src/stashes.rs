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
use shlex;

use cub_diff_gui_lib::diff::DiffPlusNotebook;
use cub_diff_lib::diff::DiffPlusParser;
use cub_diff_lib::lines::*;

use pw_gix::gtkx::dialog::*;
use pw_gix::gtkx::list_store::{
    BufferedUpdate, MapManagedUpdate, RequiredMapAction, Row, RowBuffer, RowBufferCore,
};
use pw_gix::gtkx::menu::ManagedMenu;
use pw_gix::sav_state::*;
use pw_gix::wrapper::*;

use crate::action_icons;
use crate::config;
use crate::events;
use crate::exec::ExecConsole;
use crate::repos;

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

        ctw.v_box
            .pack_start(&ctw.keep_index_ch_btn, false, false, 0);
        ctw.v_box
            .pack_start(&ctw.include_untracked_ch_btn, false, false, 0);
        ctw.v_box.pack_start(&ctw.all_ch_btn, false, false, 0);
        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        h_box.pack_start(&gtk::Label::new("Message"), false, false, 0);
        ctw.v_box.pack_start(&h_box, false, false, 0);
        let adj: Option<&gtk::Adjustment> = None;
        let scrolled_window = gtk::ScrolledWindow::new(adj, adj);
        scrolled_window.add(&ctw.text_view);
        ctw.v_box.pack_start(&scrolled_window, true, true, 0);
        ctw.v_box.show_all();

        let ctw_clone = Rc::clone(&ctw);
        ctw.include_untracked_ch_btn.connect_clicked(move |button| {
            if button.get_active() {
                ctw_clone.all_ch_btn.set_active(false)
            }
        });

        let ctw_clone = Rc::clone(&ctw);
        ctw.all_ch_btn.connect_clicked(move |button| {
            if button.get_active() {
                ctw_clone.include_untracked_ch_btn.set_active(false)
            }
        });

        ctw
    }

    pub fn get_message(&self) -> Option<String> {
        let buffer = self.text_view.get_buffer().expect("get_buffer() failed");
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
        button.set_tooltip_text(Some("Push the current state on to the stash stack"));
        button.set_image(&action_icons::stash_push_image(32));
        button.set_image_position(gtk::PositionType::Top);
        button.set_label("stash");
        exec_console
            .managed_buttons
            .add_widget("stash push", &button, repos::SAV_IN_REPO);
        let bb = Rc::new(Self {
            button: button,
            exec_console: Rc::clone(&exec_console),
        });

        let bb_clone = Rc::clone(&bb);
        bb.button.connect_clicked(move |_| bb_clone.stash_push_cb());

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
        dialog
            .get_content_area()
            .pack_start(&stash_push_widget.pwo(), true, true, 0);
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
            let result = self
                .exec_console
                .exec_cmd(&cmd, events::EV_STASHES_CHANGE + events::EV_FILES_CHANGE);
            self.unshow_busy(cursor);
            self.report_any_command_problems(&cmd, &result);
        }
        dialog.destroy();
    }
}

fn get_raw_data() -> (String, Vec<u8>) {
    let mut hasher = Hasher::new(Algorithm::SHA256);
    let text: String;
    let output = Command::new("git")
        .arg("stash")
        .arg("list")
        .output()
        .expect("getting stashes list text failed");
    if output.status.success() {
        hasher.write_all(&output.stdout).expect("hasher blew up!!!");
        text = String::from_utf8_lossy(&output.stdout).to_string();
    } else {
        text = "".to_string();
    }
    (text, hasher.finish())
}

lazy_static! {
    static ref STASH_RE: Regex =
        Regex::new(r"^(stash@\{\d+\}):\s*([^:]+):(.*)").expect("STASH regex creation failed");
}

struct StashesRowBuffer {
    row_buffer_core: Rc<RefCell<RowBufferCore<String>>>,
}

impl StashesRowBuffer {
    fn new() -> Self {
        let core = RowBufferCore::<String>::default();
        let buffer = Self {
            row_buffer_core: Rc::new(RefCell::new(core)),
        };
        buffer.init();
        buffer
    }
}

impl RowBuffer<String> for StashesRowBuffer {
    fn get_core(&self) -> Rc<RefCell<RowBufferCore<String>>> {
        self.row_buffer_core.clone()
    }

    fn set_raw_data(&self) {
        let (raw_data, digest) = get_raw_data();
        let mut core = self.row_buffer_core.borrow_mut();
        core.set_raw_data(raw_data, digest);
    }

    fn finalise(&self) {
        let mut core = self.row_buffer_core.borrow_mut();
        let mut rows: Vec<Row> = Vec::new();
        for line in core.raw_data.lines() {
            let captures = STASH_RE.captures(&line).unwrap();
            let row = vec![
                captures.get(1).unwrap().as_str().to_value(),
                captures.get(2).unwrap().as_str().to_value(),
                captures.get(3).unwrap().as_str().to_value(),
            ];
            rows.push(row);
        }
        core.rows = Rc::new(rows);
        core.set_is_finalised_true();
    }
}

struct StashesNameListStore {
    list_store: gtk::ListStore,
    stashes_row_buffer: Rc<RefCell<StashesRowBuffer>>,
}

impl BufferedUpdate<String, gtk::ListStore> for StashesNameListStore {
    fn get_list_store(&self) -> gtk::ListStore {
        self.list_store.clone()
    }

    fn get_row_buffer(&self) -> Rc<RefCell<dyn RowBuffer<String>>> {
        self.stashes_row_buffer.clone()
    }
}

impl StashesNameListStore {
    pub fn new() -> StashesNameListStore {
        Self {
            list_store: gtk::ListStore::new(&[gtk::Type::String; 3]),
            stashes_row_buffer: Rc::new(RefCell::new(StashesRowBuffer::new())),
        }
    }
}

pub struct StashesNameTable {
    scrolled_window: gtk::ScrolledWindow,
    view: gtk::TreeView,
    list_store: RefCell<StashesNameListStore>,
    required_map_action: Cell<RequiredMapAction>,
    exec_console: Rc<ExecConsole>,
    popup_menu: ManagedMenu,
    hovered_stash: RefCell<Option<String>>,
}

impl_widget_wrapper!(scrolled_window: gtk::ScrolledWindow, StashesNameTable);

impl MapManagedUpdate<StashesNameListStore, String, gtk::ListStore> for StashesNameTable {
    fn buffered_update(&self) -> Ref<'_, StashesNameListStore> {
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

impl StashesNameTable {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<StashesNameTable> {
        let list_store = RefCell::new(StashesNameListStore::new());

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
        col.set_title("Branch");
        col.set_expand(false);
        col.set_resizable(false);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", 1);

        view.append_column(&col);

        let col = gtk::TreeViewColumn::new();
        col.set_title("Commit");
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

        let table = Rc::new(StashesNameTable {
            scrolled_window,
            view,
            list_store,
            required_map_action,
            exec_console: Rc::clone(exec_console),
            popup_menu: popup_menu,
            hovered_stash: RefCell::new(None),
        });
        let table_clone = Rc::clone(&table);
        table.exec_console.event_notifier.add_notification_cb(
            events::EV_AUTO_UPDATE | events::EV_STASHES_CHANGE,
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
                let stash = get_row_item_for_event!(view, event, String, 0);
                table_clone.set_hovered_stash(stash);
                table_clone.popup_menu.popup_at_event(event);
                return Inhibit(true);
            } else if event.get_button() == 2 {
                table_clone.view.get_selection().unselect_all();
                return Inhibit(true);
            }
            Inhibit(false)
        });

        let table_clone = Rc::clone(&table);
        table
            .popup_menu
            .append_item(
                "show",
                "Show",
                Some(&action_icons::stash_show_image(16)),
                "Show the diff for the selected/indicated stash",
                repos::SAV_IN_REPO + SAV_SELN_UNIQUE_OR_HOVER_OK,
            )
            .connect_activate(move |_| {
                if let Some(stash) = table_clone.get_chosen_stash() {
                    if let Some(text) = get_stash_diff_text(&stash) {
                        let lines = Lines::from_string(&text);
                        let diff_plus_parser = DiffPlusParser::new();
                        match diff_plus_parser.parse_lines(&lines) {
                            Ok(ref diff_pluses) => {
                                let diff_notebook = DiffPlusNotebook::new(1);
                                diff_notebook.repopulate(&diff_pluses);
                                let subtitle = format!("diff: {}", stash);
                                let title = config::window_title(Some(&subtitle));
                                let dialog = table_clone.new_dialog_with_buttons(
                                    Some(&title),
                                    gtk::DialogFlags::DESTROY_WITH_PARENT,
                                    &[("Close", gtk::ResponseType::Close)],
                                );
                                dialog.enable_auto_destroy();
                                dialog.get_content_area().pack_start(
                                    &diff_notebook.pwo(),
                                    true,
                                    true,
                                    0,
                                );
                                dialog.get_content_area().pack_start(
                                    &diff_notebook.tws_count_display().pwo(),
                                    false,
                                    false,
                                    0,
                                );
                                dialog.set_size_from_recollections(
                                    "stash:show:diff:dialog",
                                    (600, 300),
                                );
                                dialog.show()
                            }
                            Err(err) => {
                                let msg = format!("{}: Malformed diff text", stash);
                                table_clone.report_error(&msg, &err);
                            }
                        }
                    }
                }
            });

        table.popup_menu.append_separator();
        let table_clone = Rc::clone(&table);
        table
            .popup_menu
            .append_item(
                "pop",
                "Pop",
                Some(&action_icons::stash_pop_image(16)),
                "Pop and apply the selected/indicated stash",
                repos::SAV_IN_REPO + SAV_SELN_UNIQUE_OR_HOVER_OK,
            )
            .connect_activate(move |_| {
                if let Some(stash) = table_clone.get_chosen_stash() {
                    let subtitle = format!("Pop Stash: {}", stash);
                    let title = config::window_title(Some(&subtitle));
                    let dialog = table_clone.new_dialog_with_buttons(
                        Some(&title),
                        gtk::DialogFlags::DESTROY_WITH_PARENT,
                        CANCEL_OK_BUTTONS,
                    );
                    let index_ch_btn = gtk::CheckButton::new_with_label("--index");
                    let ca = dialog.get_content_area();
                    ca.pack_start(&index_ch_btn, false, false, 0);
                    ca.show_all();
                    dialog.set_size_from_recollections("stash:pop:dialog", (400, 50));
                    let result = dialog.run();
                    dialog.hide();
                    if gtk::ResponseType::from(result) == gtk::ResponseType::Ok {
                        let cmd = if index_ch_btn.get_active() {
                            format!("git stash pop --index {}", shlex::quote(&stash))
                        } else {
                            format!("git stash pop {}", shlex::quote(&stash))
                        };
                        let cursor = table_clone.show_busy();
                        let result = table_clone
                            .exec_console
                            .exec_cmd(&cmd, events::EV_STASHES_CHANGE | events::EV_FILES_CHANGE);
                        table_clone.unshow_busy(cursor);
                        table_clone.report_any_command_problems(&cmd, &result);
                    }
                    dialog.destroy();
                }
            });

        let table_clone = Rc::clone(&table);
        table
            .popup_menu
            .append_item(
                "apply",
                "Apply",
                Some(&action_icons::stash_apply_image(16)),
                "Apply the selected/indicated stash",
                repos::SAV_IN_REPO + SAV_SELN_UNIQUE_OR_HOVER_OK,
            )
            .connect_activate(move |_| {
                if let Some(stash) = table_clone.get_chosen_stash() {
                    let subtitle = format!("Apply Stash: {}", stash);
                    let title = config::window_title(Some(&subtitle));
                    let dialog = table_clone.new_dialog_with_buttons(
                        Some(&title),
                        gtk::DialogFlags::DESTROY_WITH_PARENT,
                        CANCEL_OK_BUTTONS,
                    );
                    let index_ch_btn = gtk::CheckButton::new_with_label("--index");
                    let ca = dialog.get_content_area();
                    ca.pack_start(&index_ch_btn, false, false, 0);
                    ca.show_all();
                    dialog.set_size_from_recollections("stash:apply:dialog", (400, 50));
                    let result = dialog.run();
                    dialog.hide();
                    if gtk::ResponseType::from(result) == gtk::ResponseType::Ok {
                        let cmd = if index_ch_btn.get_active() {
                            format!("git stash apply --index {}", shlex::quote(&stash))
                        } else {
                            format!("git stash apply {}", shlex::quote(&stash))
                        };
                        let cursor = table_clone.show_busy();
                        let result = table_clone
                            .exec_console
                            .exec_cmd(&cmd, events::EV_STASHES_CHANGE | events::EV_FILES_CHANGE);
                        table_clone.unshow_busy(cursor);
                        table_clone.report_any_command_problems(&cmd, &result);
                    }
                    dialog.destroy();
                }
            });

        let table_clone = Rc::clone(&table);
        table
            .popup_menu
            .append_item(
                "branch",
                "Branch",
                Some(&action_icons::stash_branch_image(16)),
                "Branch the selected/indicated stash",
                repos::SAV_IN_REPO + SAV_SELN_UNIQUE_OR_HOVER_OK,
            )
            .connect_activate(move |_| {
                if let Some(stash) = table_clone.get_chosen_stash() {
                    let (response, name) = table_clone.ask_string_cancel_or_ok("Branch Name:");
                    if response == gtk::ResponseType::Ok {
                        if let Some(branch) = name {
                            let cmd = format!(
                                "git stash branch {} {}",
                                shlex::quote(&branch),
                                shlex::quote(&stash)
                            );
                            let cursor = table_clone.show_busy();
                            let result = table_clone.exec_console.exec_cmd(
                                &cmd,
                                events::EV_STASHES_CHANGE
                                    | events::EV_FILES_CHANGE
                                    | events::EV_BRANCHES_CHANGE,
                            );
                            table_clone.unshow_busy(cursor);
                            table_clone.report_any_command_problems(&cmd, &result);
                        }
                    }
                }
            });

        table.popup_menu.append_separator();
        let table_clone = Rc::clone(&table);
        table
            .popup_menu
            .append_item(
                "drop",
                "Drop",
                Some(&action_icons::stash_drop_image(16)),
                "Drop/delete the selected/indicated stash",
                repos::SAV_IN_REPO + SAV_SELN_UNIQUE_OR_HOVER_OK,
            )
            .connect_activate(move |_| {
                if let Some(stash) = table_clone.get_chosen_stash() {
                    let cmd = format!("git stash drop {}", shlex::quote(&stash));
                    let msg = format!("Confirm: {}", cmd);
                    if table_clone.ask_confirm_action(&msg, None) {
                        let cursor = table_clone.show_busy();
                        let result = table_clone
                            .exec_console
                            .exec_cmd(&cmd, events::EV_STASHES_CHANGE);
                        table_clone.unshow_busy(cursor);
                        table_clone.report_any_command_problems(&cmd, &result);
                    }
                }
            });

        table
    }

    fn set_hovered_stash(&self, stash: Option<String>) {
        let condns = self
            .view
            .get_selection()
            .get_masked_conditions_with_hover_ok(stash.is_some());
        self.popup_menu.update_condns(condns);
        *self.hovered_stash.borrow_mut() = stash;
    }

    fn get_chosen_stash(&self) -> Option<String> {
        let selection = self.view.get_selection();
        if let Some((store, iter)) = selection.get_selected() {
            store.get_value(&iter, 0).get::<String>()
        } else {
            self.hovered_stash.borrow().clone()
        }
    }
}

fn get_stash_diff_text(stash_name: &str) -> Option<String> {
    let output = Command::new("git")
        .arg("stash")
        .arg("show")
        .arg("-p")
        .arg(stash_name)
        .output()
        .expect("\"git stash show -p <name>\" blew up!!!");
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        None
    }
}
