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

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{StaticType, ToValue, TreeIter};
use pango_sys::{PANGO_STYLE_ITALIC, PANGO_STYLE_NORMAL, PANGO_STYLE_OBLIQUE};

use crypto_hash::{Algorithm, Hasher};

use pw_gix::fs_db::{FsDbIfce, FsObjectIfce, TreeRowOps};

use pw_pathux::str_path::*;
use pw_pathux::UsableDirEntry;

impl_os_fs_db!(OsFsDb, OsFsDbDir);

const UNMODIFIED: &str = "  ";
const WD_ONLY_MODIFIED: &str = " M";
const WD_ONLY_DELETED: &str = " D";
const MODIFIED: &str = "M ";
const MODIFIED_MODIFIED: &str = "MM";
const MODIFIED_DELETED: &str = "MD";
const ADDED: &str = "A ";
const ADDED_MODIFIED: &str = "AM";
const ADDED_DELETED: &str = "AD";
const DELETED: &str = "D ";
const DELETED_MODIFIED: &str = "DM";
const RENAMED: &str = "R ";
const RENAMED_MODIFIED: &str = "RM";
const RENAMED_DELETED: &str = "RD";
const COPIED: &str = "C ";
const COPIED_MODIFIED: &str = "CM";
const COPIED_DELETED: &str = "CD";
const UNMERGED: &str = "UU";
const UNMERGED_ADDED: &str = "AA";
const UNMERGED_ADDED_US: &str = "AU";
const UNMERGED_ADDED_THEM: &str = "UA";
const UNMERGED_DELETED: &str = "DD";
const UNMERGED_DELETED_US: &str = "DU";
const UNMERGED_DELETED_THEM: &str = "DA";
const NOT_TRACKED: &str = "??";
const IGNORED: &str = "!!";

lazy_static! {
    static ref SCM_FS_DB_ROW_SPEC: [gtk::Type; 8] =
        [
            gtk::Type::String,          // 0 Name
            gtk::Type::String,          // 1 Path
            gtk::Type::String,          // 2 Status
            gtk::Type::String,          // 3 AssociatedFile
            gtk::Type::String,          // 4 Icon
            gtk::Type::String,          // 5 Foreground
            gtk::Type::I32,          // 6 Style
            bool::static_type(),        // 7 is a directory?
        ];

    static ref _DECO_MAP: HashMap<&'static str, (i32, &'static str)> = {
        let mut m = HashMap::new();
        m.insert(UNMODIFIED, (PANGO_STYLE_NORMAL, "black"));
        m
    };
}

fn get_deco(status: &str) -> &(i32, &'static str) {
    _DECO_MAP
        .get(status)
        .unwrap_or(&(PANGO_STYLE_NORMAL, "black"))
}

const NAME: i32 = 0;
const PATH: i32 = 1;
const STATUS: i32 = 2;
const ASSOCIATED_FILE: i32 = 3;
const ICON: i32 = 4;
const FOREGROUND: i32 = 5;
const STYLE: i32 = 6;
const IS_DIR: i32 = 7;

#[derive(Debug)]
pub struct OsFsoData {
    name: String,
    path: String,
    status: String,
    associated_file: String,
    is_dir: bool,
}

impl FsObjectIfce for OsFsoData {
    fn new(dir_entry: &UsableDirEntry) -> Self {
        OsFsoData {
            name: dir_entry.file_name(),
            path: dir_entry.path().to_string_lossy().into_owned(),
            status: UNMODIFIED.to_string(),
            associated_file: "".to_string(),
            is_dir: dir_entry.is_dir(),
        }
    }

    fn tree_store_spec() -> Vec<gtk::Type> {
        SCM_FS_DB_ROW_SPEC.to_vec()
    }

    fn tree_view_columns() -> Vec<gtk::TreeViewColumn> {
        let col = gtk::TreeViewColumn::new();

        let cell = gtk::CellRendererPixbuf::new();
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "icon-name", ICON);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", NAME);
        col.add_attribute(&cell, "foreground", FOREGROUND);
        col.add_attribute(&cell, "style", STYLE);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", ASSOCIATED_FILE);
        col.add_attribute(&cell, "foreground", FOREGROUND);
        col.add_attribute(&cell, "style", STYLE);

        vec![col]
    }

    fn row_is_a_dir<S: TreeRowOps>(store: &S, iter: &TreeIter) -> bool {
        store.get_value(iter, IS_DIR).get::<bool>().unwrap()
    }

    fn row_is_place_holder<S: TreeRowOps>(store: &S, iter: &TreeIter) -> bool {
        store
            .get_value(iter, NAME)
            .get::<String>()
            .unwrap()
            .as_str()
            == "(empty)"
    }

    fn get_name_from_row<S: TreeRowOps>(store: &S, iter: &TreeIter) -> String {
        store.get_value(iter, NAME).get::<String>().unwrap()
    }

    fn get_path_from_row<S: TreeRowOps>(store: &S, iter: &TreeIter) -> String {
        store.get_value(iter, PATH).get::<String>().unwrap()
    }

    fn update_row_if_required<S: TreeRowOps>(&self, store: &S, iter: &TreeIter) -> bool {
        assert_eq!(
            self.name,
            store.get_value(iter, NAME).get::<String>().unwrap()
        );
        let mut changed = false;
        if self.path != store.get_value(iter, PATH).get::<String>().unwrap() {
            store.set_value(iter, PATH as u32, &self.path.to_value());
            changed = true;
        }
        if self.status != store.get_value(iter, STATUS).get::<String>().unwrap() {
            store.set_value(iter, STATUS as u32, &self.status.to_value());
            changed = true;
        }
        if self.associated_file
            != store
                .get_value(iter, ASSOCIATED_FILE)
                .get::<String>()
                .unwrap()
        {
            store.set_value(
                iter,
                ASSOCIATED_FILE as u32,
                &self.associated_file.to_value(),
            );
            changed = true;
        }
        if self.is_dir != store.get_value(iter, IS_DIR).get::<bool>().unwrap() {
            store.set_value(iter, IS_DIR as u32, &self.is_dir.to_value());
            if self.is_dir {
                store.set_value(iter, ICON as u32, &"stock_directory".to_value());
            } else {
                store.set_value(iter, ICON as u32, &"stock_file".to_value());
            }
            changed = true;
        }
        changed
    }

    fn set_row_values<S: TreeRowOps>(&self, store: &S, iter: &TreeIter) {
        store.set_value(iter, NAME as u32, &self.name.to_value());
        store.set_value(iter, PATH as u32, &self.path.to_value());
        store.set_value(iter, STATUS as u32, &self.status.to_value());
        store.set_value(
            iter,
            ASSOCIATED_FILE as u32,
            &self.associated_file.to_value(),
        );
        if self.is_dir {
            store.set_value(iter, ICON as u32, &"gtk-directory".to_value());
        } else {
            store.set_value(iter, ICON as u32, &"gtk-file".to_value());
        }
        let (style, foreground) = get_deco(&self.status.as_str());
        store.set_value(iter, STYLE as u32, &style.to_value());
        store.set_value(iter, FOREGROUND as u32, &foreground.to_value());
        store.set_value(iter, IS_DIR as u32, &self.is_dir.to_value());
    }

    fn set_place_holder_values<S: TreeRowOps>(store: &S, iter: &TreeIter) {
        store.set_value(iter, NAME as u32, &"(empty)".to_value());
        store.set_value(iter, PATH as u32, &"".to_value());
        store.set_value(iter, IS_DIR as u32, &false.to_value());
        store.set_value(iter, FOREGROUND as u32, &"purple".to_value());
        store.set_value(iter, STYLE as u32, &PANGO_STYLE_ITALIC.to_value());
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn path(&self) -> &str {
        &self.path
    }

    fn is_dir(&self) -> bool {
        self.is_dir
    }
}
