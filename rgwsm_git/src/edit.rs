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

use std::cell::Cell;
use std::convert::From;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::rc::Rc;

use glob::{Pattern, PatternError};

use pw_gix::{
    glib,
    gtk::{self, prelude::*},
    //gtkx::dialog::AutoDestroy,
    sav_state::*,
    wrapper::*,
};

use crate::config;
use crate::repos;

#[derive(Debug)]
pub enum EditorTableError {
    InputOutput(io::Error),
    GlobPattern(PatternError),
    SerdeJson(serde_json::Error),
}

impl fmt::Display for EditorTableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EditorTableError is here!")
    }
}

impl Error for EditorTableError {
    fn description(&self) -> &str {
        match self {
            EditorTableError::InputOutput(_) => "I/O Error accessing editor assignment table",
            EditorTableError::GlobPattern(_) => {
                "Glob Pattern Error accessing editor assignment table"
            }
            EditorTableError::SerdeJson(_) => "Serde Json Error accessing editor assignment table",
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            EditorTableError::InputOutput(err) => Some(err),
            EditorTableError::GlobPattern(err) => Some(err),
            EditorTableError::SerdeJson(err) => Some(err),
        }
    }
}

impl From<io::Error> for EditorTableError {
    fn from(error: io::Error) -> Self {
        EditorTableError::InputOutput(error)
    }
}

impl From<PatternError> for EditorTableError {
    fn from(error: PatternError) -> Self {
        EditorTableError::GlobPattern(error)
    }
}

impl From<serde_json::Error> for EditorTableError {
    fn from(error: serde_json::Error) -> Self {
        EditorTableError::SerdeJson(error)
    }
}

#[cfg(target_family = "unix")]
const PATH_SEP: char = ':';
#[cfg(target_os = "windows")]
const PATH_SEP: char = ';';

#[cfg(target_family = "unix")]
const DEFAULT_EDITOR: &str = "gedit";
#[cfg(target_os = "windows")]
const DEFAULT_EDITOR: &str = "notepad";

pub fn default_editor() -> String {
    if let Ok(editor) = env::var("VISUAL") {
        editor
    } else {
        DEFAULT_EDITOR.to_string()
    }
}

fn editor_assignment_table_filepath() -> PathBuf {
    let mut pathbuf = config::get_config_dir_path();
    pathbuf.push("editor_assignment_table");
    pathbuf
}

fn read_editor_assignment_table() -> Result<Vec<(String, String)>, EditorTableError> {
    let mut file = File::open(editor_assignment_table_filepath())?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let v: Vec<(String, String)> = serde_json::from_str(&buffer)?;
    Ok(v)
}

fn write_editor_assignment_table(table: &[(String, String)]) -> Result<usize, EditorTableError> {
    let data = serde_json::to_string(table)?;
    let mut file = File::create(editor_assignment_table_filepath())?;
    let nbytes = file.write(data.as_bytes())?;
    Ok(nbytes)
}

pub fn init_editor_assignment_table() {
    if !editor_assignment_table_filepath().is_file() {
        write_editor_assignment_table(&[]).expect("failed to initialize editor assignment table");
    }
}

pub fn get_assigned_editor(file_path: &str) -> Result<String, EditorTableError> {
    let editor_assignment_table = read_editor_assignment_table()?;
    for (globs, editor) in editor_assignment_table.iter() {
        for glob in globs.split(PATH_SEP) {
            let pattern = Pattern::new(glob)?;
            if pattern.matches(file_path) {
                return Ok(editor.to_string());
            }
        }
    }
    Ok(default_editor())
}

const SAV_MODIFIED: u64 = repos::SAV_HAS_SUBMODULES << 1;
const SAV_NOT_MODIFIED: u64 = SAV_MODIFIED << 1;
const SAV_MODIFIED_MASK: u64 = SAV_MODIFIED | SAV_NOT_MODIFIED;

#[derive(PWO, Wrapper)]
pub struct EditorAllocationTableEditor {
    v_box: gtk::Box,
    view: gtk::TreeView,
    list_store: gtk::ListStore,
    add_button: gtk::Button,
    insert_button: gtk::Button,
    delete_button: gtk::Button,
    undo_button: gtk::Button,
    apply_button: gtk::Button,
    managed_buttons: Rc<ConditionalWidgetGroups<gtk::Button>>,
    modified: Cell<bool>,
}

impl EditorAllocationTableEditor {
    pub fn new() -> Rc<Self> {
        let list_store = gtk::ListStore::new(&[glib::Type::String; 2]);
        let view = gtk::TreeView::with_model(&list_store);
        let managed_buttons = ConditionalWidgetGroups::<gtk::Button>::new(
            WidgetStatesControlled::Sensitivity,
            Some(&view.get_selection()),
            None,
        );

        let eate = Rc::new(Self {
            v_box: gtk::Box::new(gtk::Orientation::Vertical, 0),
            view,
            list_store,
            add_button: gtk::Button::with_label("Add"),
            insert_button: gtk::Button::with_label("Insert"),
            delete_button: gtk::Button::with_label("Delete"),
            undo_button: gtk::Button::with_label("Undo"),
            apply_button: gtk::Button::with_label("Apply"),
            managed_buttons,
            modified: Cell::new(false),
        });
        eate.set_modified(false);

        eate.view.set_headers_visible(true);
        eate.view.set_reorderable(true);
        eate.view.set_grid_lines(gtk::TreeViewGridLines::Both);

        eate.view
            .get_selection()
            .set_mode(gtk::SelectionMode::Single);
        eate.view.connect_button_press_event(move |view, event| {
            if event.get_button() == 2 {
                view.get_selection().unselect_all();
                return Inhibit(true);
            }
            Inhibit(false)
        });

        let eate_clone = Rc::clone(&eate);
        eate.list_store
            .connect_row_changed(move |_, _, _| eate_clone.set_modified(true));
        let eate_clone = Rc::clone(&eate);
        eate.list_store
            .connect_row_deleted(move |_, _| eate_clone.set_modified(true));
        let eate_clone = Rc::clone(&eate);
        eate.list_store
            .connect_row_inserted(move |_, _, _| eate_clone.set_modified(true));

        let col = gtk::TreeViewColumn::new();
        col.set_title("File Pattern(s)");
        col.set_expand(true);
        col.set_resizable(false);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(true);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", 0);
        let eate_clone = Rc::clone(&eate);
        cell.connect_edited(move |_, tree_path, new_text| {
            if let Some(tree_iter) = eate_clone.list_store.get_iter(&tree_path) {
                eate_clone
                    .list_store
                    .set_value(&tree_iter, 0, &new_text.to_value());
                eate_clone.set_modified(true);
            }
        });

        eate.view.append_column(&col);

        let col = gtk::TreeViewColumn::new();
        col.set_title("Editor Command");
        col.set_expand(true);
        col.set_resizable(false);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(true);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", 1);
        let eate_clone = Rc::clone(&eate);
        cell.connect_edited(move |_, tree_path, new_text| {
            if let Some(tree_iter) = eate_clone.list_store.get_iter(&tree_path) {
                eate_clone
                    .list_store
                    .set_value(&tree_iter, 1, &new_text.to_value());
                eate_clone.set_modified(true);
            }
        });

        eate.view.append_column(&col);

        eate.managed_buttons
            .add_widget("add", &eate.add_button, SAV_SELN_NONE);
        let eate_clone = Rc::clone(&eate);
        eate.add_button.connect_clicked(move |_| {
            eate_clone.list_store.append();
        });
        eate.add_button
            .set_tooltip_text(Some("Append a new entry to the table."));

        eate.managed_buttons
            .add_widget("insert", &eate.insert_button, SAV_SELN_UNIQUE);
        let eate_clone = Rc::clone(&eate);
        eate.insert_button.connect_clicked(move |_| {
            if let Some((_, iter)) = eate_clone.view.get_selection().get_selected() {
                eate_clone.list_store.insert_before(Some(&iter));
            }
        });
        eate.insert_button.set_tooltip_text(Some(
            "Insert a new entry to the table before the selected entry.",
        ));

        eate.managed_buttons
            .add_widget("delete", &eate.delete_button, SAV_SELN_MADE);
        let eate_clone = Rc::clone(&eate);
        eate.delete_button.connect_clicked(move |_| {
            if let Some((_, iter)) = eate_clone.view.get_selection().get_selected() {
                eate_clone.list_store.remove(&iter);
            }
        });
        eate.delete_button
            .set_tooltip_text(Some("Remove the selected entry from the table."));

        eate.managed_buttons
            .add_widget("undo", &eate.undo_button, SAV_MODIFIED);
        let eate_clone = Rc::clone(&eate);
        eate.undo_button
            .connect_clicked(move |_| eate_clone.load_table());
        eate.undo_button
            .set_tooltip_text(Some("Undo all unapplied changes in the table."));

        eate.managed_buttons
            .add_widget("apply", &eate.apply_button, SAV_MODIFIED);
        let eate_clone = Rc::clone(&eate);
        eate.apply_button.connect_clicked(move |_| {
            eate_clone.write_table();
            eate_clone.load_table();
        });
        eate.apply_button
            .set_tooltip_text(Some("Apply outstanding changes in the table."));

        let adj: Option<&gtk::Adjustment> = None;
        let scrolled_window = gtk::ScrolledWindow::new(adj, adj);
        eate.v_box.pack_start(&scrolled_window, true, true, 0);
        scrolled_window.add(&eate.view);
        scrolled_window.show_all();
        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        h_box.pack_start(&eate.add_button, true, true, 0);
        h_box.pack_start(&eate.insert_button, true, true, 0);
        h_box.pack_start(&eate.delete_button, true, true, 0);
        h_box.pack_start(&eate.undo_button, true, true, 0);
        h_box.pack_start(&eate.apply_button, true, true, 0);
        h_box.show_all();
        eate.v_box.pack_start(&h_box, false, false, 0);
        eate.v_box.show_all();

        eate
    }

    fn load_table(&self) {
        self.list_store.clear();
        match read_editor_assignment_table() {
            Ok(table) => {
                for (globs, editor) in table.iter() {
                    let t_iter = self.list_store.append();
                    self.list_store.set_value(&t_iter, 0, &globs.to_value());
                    self.list_store.set_value(&t_iter, 1, &editor.to_value());
                }
            }
            Err(err) => {
                let msg = "Problem loading editor assignment table";
                self.report_error(msg, &err);
            }
        }
        self.set_modified(false);
    }

    fn set_modified(&self, val: bool) {
        self.modified.set(val);
        if val {
            let condns = MaskedCondns {
                condns: SAV_MODIFIED,
                mask: SAV_MODIFIED_MASK,
            };
            self.managed_buttons.update_condns(condns);
        } else {
            let condns = MaskedCondns {
                condns: SAV_NOT_MODIFIED,
                mask: SAV_MODIFIED_MASK,
            };
            self.managed_buttons.update_condns(condns);
        }
    }

    fn write_table(&self) {
        let mut v: Vec<(String, String)> = vec![];
        if let Some(t_iter) = self.list_store.get_iter_first() {
            loop {
                let globs_v = self.list_store.get_value(&t_iter, 0);
                let editor_v = self.list_store.get_value(&t_iter, 1);
                let globs = globs_v
                    .get::<String>()
                    .unwrap()
                    .expect("error extracting globs from list store");
                let editor = editor_v
                    .get::<String>()
                    .unwrap()
                    .expect("error extracting editor from list store");
                let globs = globs.trim().to_string();
                let editor = editor.trim().to_string();
                if !globs.is_empty() && !editor.is_empty() {
                    v.push((globs, editor));
                }
                if !self.list_store.iter_next(&t_iter) {
                    break;
                }
            }
        }
        if let Err(err) = write_editor_assignment_table(&v) {
            let msg = "Problem writing editor assignment table";
            self.report_error(msg, &err);
        } else {
            self.set_modified(false);
        }
    }
}

#[derive(PWO, Wrapper)]
pub struct EditorAlocationMenuItem {
    menu_item: gtk::MenuItem,
}

impl EditorAlocationMenuItem {
    pub fn new() -> Rc<Self> {
        let eami = Rc::new(Self {
            menu_item: gtk::MenuItem::with_label("Editor Allocation"),
        });

        let eami_clone = Rc::clone(&eami);
        eami.menu_item.connect_activate(move |_| {
            let title = format!("{}: Editor Allocation", config::APP_NAME);
            let dialog = eami_clone
                .new_dialog_builder()
                .title(&title)
                .destroy_with_parent(true)
                .build();
            for button in Self::CLOSE_BUTTONS.iter() {
                dialog.add_button(button.0, button.1);
            }
            // TODO: dialog.enable_auto_destroy();
            let table = EditorAllocationTableEditor::new();
            dialog
                .get_content_area()
                .pack_start(table.pwo(), true, true, 0);
            table.load_table();
            dialog.show();
        });

        eami
    }
}
