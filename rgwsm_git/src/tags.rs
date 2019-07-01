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
use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::process::{self, Command};
use std::rc::Rc;

use crypto_hash::{Algorithm, Hasher};
use regex::Regex;

use gtk;
use gtk::prelude::*;

use shlex;


use pw_gix::gtkx::dialog::RememberDialogSize;
use pw_gix::gtkx::entry::LabelledTextEntry;
use pw_gix::gtkx::list_store::{
    BufferedUpdate, MapManagedUpdate, RequiredMapAction, Row, RowBuffer, RowBufferCore,
};
use pw_gix::gtkx::menu::ManagedMenu;
use pw_gix::sav_state::*;
use pw_gix::wrapper::*;

use crate::action_icons;
use crate::events;
use crate::exec::ExecConsole;
use crate::message;
use crate::repos;

#[derive(Debug)]
pub enum TagError {
    NoTagName,
    NoKeyId,
    NoAnnotationMessage,
}

impl fmt::Display for TagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TagError is here!")
    }
}

impl Error for TagError {
    fn description(&self) -> &str {
        match self {
            TagError::NoTagName => "Tag name is required",
            TagError::NoKeyId => "User key-id is required",
            TagError::NoAnnotationMessage => "Annotation message is required",
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub struct NewTagWidget {
    v_box: gtk::Box,
    tag_name_entry: Rc<LabelledTextEntry>,
    force_cbth: gtk::CheckButton,
    annotate_cbtn: gtk::CheckButton,
    sign_cbtn: gtk::CheckButton,
    key_cbtn: gtk::CheckButton,
    key_id_entry: Rc<LabelledTextEntry>,
    message: Rc<message::MessageWidget>,
    exec_console: Rc<ExecConsole>,
}

impl_widget_wrapper!(v_box: gtk::Box, NewTagWidget);

impl NewTagWidget {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<NewTagWidget> {
        let ntw = Rc::new(NewTagWidget {
            v_box: gtk::Box::new(gtk::Orientation::Vertical, 0),
            tag_name_entry: LabelledTextEntry::new("Tag: "),
            force_cbth: gtk::CheckButton::new_with_label("--force"),
            annotate_cbtn: gtk::CheckButton::new_with_label("--annotate"),
            sign_cbtn: gtk::CheckButton::new_with_label("--sign"),
            key_cbtn: gtk::CheckButton::new_with_label("--local-user"),
            key_id_entry: LabelledTextEntry::new("Key Id: "),
            message: message::MessageWidget::new("Message"),
            exec_console: Rc::clone(exec_console),
        });
        ntw.key_id_entry.pwo().set_sensitive(false);
        ntw.message.pwo().set_sensitive(false);

        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        h_box.pack_start(&ntw.tag_name_entry.pwo(), true, true, 0);
        h_box.pack_start(&ntw.force_cbth, false, false, 0);
        ntw.v_box.pack_start(&h_box, false, false, 0);
        ntw.tag_name_entry.entry().set_activates_default(true);

        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 20);
        h_box.pack_start(&ntw.annotate_cbtn, false, false, 0);
        h_box.pack_start(&ntw.sign_cbtn, false, false, 0);
        h_box.pack_start(&ntw.key_cbtn, false, false, 0);
        h_box.pack_start(&ntw.key_id_entry.pwo(), true, true, 0);
        ntw.v_box.pack_start(&h_box, false, false, 0);

        ntw.v_box.pack_start(&ntw.message.pwo(), true, true, 0);

        let ntw_clone = Rc::clone(&ntw);
        ntw.annotate_cbtn.connect_property_active_notify(move |cb| {
            if cb.get_active() {
                ntw_clone.sign_cbtn.set_active(false);
                ntw_clone.key_cbtn.set_active(false);
                ntw_clone.message.pwo().set_sensitive(true);
            } else if !ntw_clone.sign_cbtn.get_active() && !ntw_clone.key_cbtn.get_active() {
                ntw_clone.message.pwo().set_sensitive(false);
            }
        });

        let ntw_clone = Rc::clone(&ntw);
        ntw.sign_cbtn.connect_property_active_notify(move |cb| {
            if cb.get_active() {
                ntw_clone.annotate_cbtn.set_active(false);
                ntw_clone.key_cbtn.set_active(false);
                ntw_clone.message.pwo().set_sensitive(true);
            } else if !ntw_clone.annotate_cbtn.get_active() && !ntw_clone.key_cbtn.get_active() {
                ntw_clone.message.pwo().set_sensitive(false);
            }
        });

        let ntw_clone = Rc::clone(&ntw);
        ntw.key_cbtn.connect_property_active_notify(move |cb| {
            if cb.get_active() {
                ntw_clone.annotate_cbtn.set_active(false);
                ntw_clone.sign_cbtn.set_active(false);
                ntw_clone.key_id_entry.pwo().set_sensitive(true);
                ntw_clone.message.pwo().set_sensitive(true);
            } else {
                ntw_clone.key_id_entry.pwo().set_sensitive(false);
                if !ntw_clone.annotate_cbtn.get_active() && !ntw_clone.sign_cbtn.get_active() {
                    ntw_clone.message.pwo().set_sensitive(false);
                }
            }
        });

        ntw.v_box.show_all();

        ntw
    }

    pub fn apply(
        &self,
        target: Option<&str>,
    ) -> Result<(String, io::Result<process::Output>), TagError> {
        if let Some(tag_name) = self.tag_name_entry.entry().get_text() {
            if tag_name.len() > 0 {
                let mut cmd = "git tag ".to_string();
                if self.force_cbth.get_active() {
                    cmd.push_str("--force ")
                }
                let mut annotate = false;
                if self.annotate_cbtn.get_active() {
                    annotate = true;
                    cmd.push_str("--annotate ");
                } else if self.sign_cbtn.get_active() {
                    annotate = true;
                    cmd.push_str("--sign ");
                } else if self.key_cbtn.get_active() {
                    annotate = true;
                    if let Some(key_id) = self.key_id_entry.entry().get_text() {
                        if key_id.len() > 0 {
                            cmd.push_str("--local-user ");
                            cmd.push_str(&shlex::quote(&key_id));
                            cmd.push(' ');
                        } else {
                            return Err(TagError::NoKeyId);
                        }
                    } else {
                        return Err(TagError::NoKeyId);
                    }
                }
                if annotate {
                    if let Some(msg) = self.message.get_message() {
                        cmd.push_str("-m ");
                        cmd.push_str(&shlex::quote(&msg));
                        cmd.push(' ');
                    } else {
                        return Err(TagError::NoAnnotationMessage);
                    }
                }
                cmd.push_str(&shlex::quote(&tag_name));
                if let Some(target) = target {
                    cmd.push(' ');
                    cmd.push_str(&shlex::quote(target));
                };
                let old_cursor = self.show_busy();
                let result = self.exec_console.exec_cmd(&cmd, events::EV_TAGS_CHANGE);
                self.unshow_busy(old_cursor);
                return Ok((cmd, result));
            } else {
                return Err(TagError::NoTagName);
            }
        } else {
            return Err(TagError::NoTagName);
        }
    }
}

pub trait CreatTag: WidgetWrapper {
    fn exec_console(&self) -> &Rc<ExecConsole>;

    fn create_tag_for(&self, target: Option<&str>) {
        let dialog = self.new_dialog_with_buttons(
            Some("New Tag"),
            gtk::DialogFlags::DESTROY_WITH_PARENT | gtk::DialogFlags::MODAL,
            CANCEL_OK_BUTTONS,
        );
        dialog.set_default_response(gtk::ResponseType::Ok);
        let ntw = NewTagWidget::new(self.exec_console());
        dialog
            .get_content_area()
            .pack_start(&ntw.pwo(), true, true, 0);
        dialog.get_content_area().show_all();
        dialog.set_size_from_recollections("tag:dialog", (640, 320));
        loop {
            let result = dialog.run();
            if gtk::ResponseType::from(result) == gtk::ResponseType::Ok {
                match ntw.apply(target) {
                    Ok((cmd, result)) => {
                        self.report_any_command_problems(&cmd, &result);
                        if let Ok(ref output) = result {
                            if output.status.success() {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    Err(err) => self.report_error("Insufficient data for creating tag", &err),
                }
            } else {
                break;
            }
        }
        dialog.destroy();
    }
}

pub struct TagButton {
    button: gtk::Button,
    exec_console: Rc<ExecConsole>,
}

impl_widget_wrapper!(button: gtk::Button, TagButton);

impl CreatTag for TagButton {
    fn exec_console(&self) -> &Rc<ExecConsole> {
        &self.exec_console
    }
}

impl TagButton {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<Self> {
        let button = gtk::Button::new();
        button.set_tooltip_text(Some("Tag the current HEAD revision"));
        button.set_image(&action_icons::tag_image(32));
        button.set_image_position(gtk::PositionType::Top);
        button.set_label("tag");
        exec_console
            .managed_buttons
            .add_widget("tag", &button, repos::SAV_IN_REPO);
        let bb = Rc::new(Self {
            button: button,
            exec_console: Rc::clone(&exec_console),
        });

        let bb_clone = Rc::clone(&bb);
        bb.button
            .connect_clicked(move |_| bb_clone.create_tag_for(None));

        bb
    }
}

// Tag table

fn get_raw_data() -> (String, Vec<u8>) {
    let mut hasher = Hasher::new(Algorithm::SHA256);
    let text: String;
    let output = Command::new("git")
        .arg("tag")
        .arg("--format=%(refname:short) %(objectname:short) %(subject)")
        .output()
        .expect("getting tags text failed");
    if output.status.success() {
        hasher
            .write_all(&output.stdout)
            .expect("hasher blew up!!!");
        text = String::from_utf8_lossy(&output.stdout).to_string();
    } else {
        text = "".to_string();
    }
    (text, hasher.finish())
}

lazy_static! {
    static ref TAGS_RE: Regex =
        Regex::new(r"(\S+)\s+([a-fA-F0-9]{7}[a-fA-F0-9]*)?\s*([^\s].*)").unwrap();
}

fn extract_tag_row(line: &str) -> Row {
    let captures = TAGS_RE.captures(&line).unwrap();
    let name = captures.get(1).unwrap().as_str();
    let rev = captures.get(2).unwrap().as_str();
    let synopsis = captures.get(3).unwrap().as_str();
    let mut v = vec![];
    v.push(name.to_value());
    v.push(format!("<b>{}</b>", name).to_value());
    v.push(rev.to_value());
    v.push(synopsis.to_value());
    v
}

struct TagsRowBuffer {
    row_buffer_core: Rc<RefCell<RowBufferCore<String>>>,
}

impl TagsRowBuffer {
    fn new() -> Self {
        let core = RowBufferCore::<String>::default();
        let buffer = Self {
            row_buffer_core: Rc::new(RefCell::new(core)),
        };
        buffer.init();
        buffer
    }
}

impl RowBuffer<String> for TagsRowBuffer {
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
            rows.push(extract_tag_row(&line))
        }
        core.rows = Rc::new(rows);
        core.set_is_finalised_true();
    }
}

struct TagsNameListStore {
    list_store: gtk::ListStore,
    tags_row_buffer: Rc<RefCell<TagsRowBuffer>>,
}

impl BufferedUpdate<String, gtk::ListStore> for TagsNameListStore {
    fn get_list_store(&self) -> gtk::ListStore {
        self.list_store.clone()
    }

    fn get_row_buffer(&self) -> Rc<RefCell<dyn RowBuffer<String>>> {
        self.tags_row_buffer.clone()
    }
}

impl TagsNameListStore {
    pub fn new() -> TagsNameListStore {
        Self {
            list_store: gtk::ListStore::new(&[gtk::Type::String; 4]),
            tags_row_buffer: Rc::new(RefCell::new(TagsRowBuffer::new())),
        }
    }
}

pub struct TagsNameTable {
    scrolled_window: gtk::ScrolledWindow,
    view: gtk::TreeView,
    list_store: RefCell<TagsNameListStore>,
    required_map_action: Cell<RequiredMapAction>,
    exec_console: Rc<ExecConsole>,
    popup_menu: ManagedMenu,
    hovered_tag: RefCell<Option<String>>,
}

impl_widget_wrapper!(scrolled_window: gtk::ScrolledWindow, TagsNameTable);

impl CreatTag for TagsNameTable {
    fn exec_console(&self) -> &Rc<ExecConsole> {
        &self.exec_console
    }
}

impl MapManagedUpdate<TagsNameListStore, String, gtk::ListStore>
    for TagsNameTable
{
    fn buffered_update(&self) -> Ref<'_, TagsNameListStore> {
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

impl TagsNameTable {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<TagsNameTable> {
        let list_store = RefCell::new(TagsNameListStore::new());

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

        view.append_column(&col);

        let col = gtk::TreeViewColumn::new();
        col.set_title("Rev");
        col.set_expand(false);
        col.set_resizable(false);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", 2);

        view.append_column(&col);

        let col = gtk::TreeViewColumn::new();
        col.set_title("Annotation/Synopsis");
        col.set_expand(false);
        col.set_resizable(false);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", 3);

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

        let table = Rc::new(TagsNameTable {
            scrolled_window,
            view,
            list_store,
            required_map_action,
            exec_console: Rc::clone(exec_console),
            popup_menu: popup_menu,
            hovered_tag: RefCell::new(None),
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
                "Switch to the selected/indicated tag",
                repos::SAV_IN_REPO + SAV_SELN_UNIQUE_OR_HOVER_OK,
            )
            .connect_activate(move |_| {
                let selection = table_clone.view.get_selection();
                let tag = if let Some((store, iter)) = selection.get_selected() {
                    store.get_value(&iter, 0).get::<String>()
                } else {
                    table_clone.hovered_tag.borrow().clone()
                };
                if let Some(tag) = tag {
                    let cmd = format!("git checkout {}", shlex::quote(&tag));
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
                "tag",
                "Tag",
                None,
                "Set a new tag the selected/indicated object",
                repos::SAV_IN_REPO + SAV_SELN_UNIQUE_OR_HOVER_OK,
            )
            .connect_activate(move |_| {
                let selection = table_clone.view.get_selection();
                let tag = if let Some((store, iter)) = selection.get_selected() {
                    store.get_value(&iter, 0).get::<String>()
                } else {
                    table_clone.hovered_tag.borrow().clone()
                };
                if let Some(tag) = tag {
                    table_clone.create_tag_for(Some(&tag))
                }
            });

        let table_clone = Rc::clone(&table);
        table
            .popup_menu
            .append_item(
                "delete",
                "Delete",
                None,
                "Delete the selected/indicated tag",
                repos::SAV_IN_REPO + SAV_SELN_UNIQUE_OR_HOVER_OK,
            )
            .connect_activate(move |_| {
                let selection = table_clone.view.get_selection();
                let tag = if let Some((store, iter)) = selection.get_selected() {
                    store.get_value(&iter, 0).get::<String>()
                } else {
                    table_clone.hovered_tag.borrow().clone()
                };
                if let Some(tag) = tag {
                    let cmd = format!("git tag -d {}", shlex::quote(&tag));
                    let result = table_clone
                        .exec_console
                        .exec_cmd(&cmd, events::EV_TAGS_CHANGE);
                    table_clone.report_any_command_problems(&cmd, &result);
                }
            });

        let table_clone = table.clone();
        table.view.connect_button_press_event(move |view, event| {
            if event.get_button() == 3 {
                let tag = get_row_item_for_event!(view, event, String, 0);
                table_clone.set_hovered_tag(tag);
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

    fn set_hovered_tag(&self, tag: Option<String>) {
        let condns = self
            .view
            .get_selection()
            .get_masked_conditions_with_hover_ok(tag.is_some());
        self.popup_menu.update_condns(condns);
        *self.hovered_tag.borrow_mut() = tag;
    }
}
