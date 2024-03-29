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

// TODO: make a porcelain version 2 implementation of this functionality

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::iter::Iterator;
use std::path::MAIN_SEPARATOR;
use std::process::Command;
use std::rc::Rc;
use std::slice::Iter;

use crypto_hash::{Algorithm, Hasher};
use regex::Regex;

use pw_gix::{
    fs_db::{FsDbIfce, FsObjectIfce, TreeRowOps},
    glib::{self, StaticType, ToValue},
    gtk::{self, prelude::*, TreeIter},
    pango_sys::{PANGO_STYLE_ITALIC, PANGO_STYLE_NORMAL, PANGO_STYLE_OBLIQUE},
};

use pw_pathux::str_path::*;
use pw_pathux::UsableDirEntry;

const NO_STATUS: &str = "";
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
    static ref SCM_FS_DB_ROW_SPEC: [glib::Type; 9] =
        [
            glib::Type::String,          // 0 Name
            glib::Type::String,          // 1 Path
            glib::Type::String,          // 2 Status
            glib::Type::String,          // 3 AssociatedFile
            glib::Type::String,          // 4 Relation
            glib::Type::String,          // 5 Icon
            glib::Type::String,          // 6 Foreground
            glib::Type::I32,             // 7 Style
            bool::static_type(),        // 8 is a directory?
        ];

    static ref _DECO_MAP: HashMap<&'static str, (i32, &'static str)> = {
        let mut m = HashMap::new();
        m.insert(NO_STATUS, (PANGO_STYLE_NORMAL, "black"));
        m.insert(UNMODIFIED, (PANGO_STYLE_NORMAL, "black"));
        m.insert(WD_ONLY_MODIFIED, (PANGO_STYLE_NORMAL, "blue"));
        m.insert(WD_ONLY_DELETED, (PANGO_STYLE_NORMAL, "red"));
        m.insert(MODIFIED, (PANGO_STYLE_NORMAL, "blue"));
        m.insert(MODIFIED_MODIFIED, (PANGO_STYLE_NORMAL, "blue"));
        m.insert(MODIFIED_DELETED, (PANGO_STYLE_NORMAL, "red"));
        m.insert(ADDED, (PANGO_STYLE_NORMAL, "darkgreen"));
        m.insert(ADDED_MODIFIED, (PANGO_STYLE_NORMAL, "blue"));
        m.insert(ADDED_DELETED, (PANGO_STYLE_NORMAL, "red"));
        m.insert(DELETED, (PANGO_STYLE_NORMAL, "red"));
        m.insert(DELETED_MODIFIED, (PANGO_STYLE_NORMAL, "blue"));
        m.insert(RENAMED, (PANGO_STYLE_ITALIC, "pink"));
        m.insert(RENAMED_MODIFIED, (PANGO_STYLE_ITALIC, "blue"));
        m.insert(RENAMED_DELETED, (PANGO_STYLE_ITALIC, "red"));
        m.insert(COPIED, (PANGO_STYLE_ITALIC, "green"));
        m.insert(COPIED_MODIFIED, (PANGO_STYLE_ITALIC, "blue"));
        m.insert(COPIED_DELETED, (PANGO_STYLE_ITALIC, "red"));
        m.insert(UNMERGED, (PANGO_STYLE_NORMAL, "magenta"));
        m.insert(UNMERGED_ADDED, (PANGO_STYLE_NORMAL, "magenta"));
        m.insert(UNMERGED_ADDED_US, (PANGO_STYLE_NORMAL, "magenta"));
        m.insert(UNMERGED_ADDED_THEM, (PANGO_STYLE_NORMAL, "magenta"));
        m.insert(UNMERGED_DELETED, (PANGO_STYLE_NORMAL, "magenta"));
        m.insert(UNMERGED_DELETED_US, (PANGO_STYLE_NORMAL, "magenta"));
        m.insert(UNMERGED_DELETED_THEM, (PANGO_STYLE_NORMAL, "magenta"));
        m.insert(NOT_TRACKED, (PANGO_STYLE_ITALIC, "cyan"));
        m.insert(IGNORED, (PANGO_STYLE_ITALIC, "grey"));
        m
    };
}

fn get_deco(status: &str) -> &(i32, &'static str) {
    _DECO_MAP
        .get(status)
        .unwrap_or(&(PANGO_STYLE_NORMAL, "black"))
}

pub const NAME: i32 = 0;
pub const PATH: i32 = 1;
pub const STATUS: i32 = 2;
pub const RELATED_FILE: i32 = 3;
pub const RELATION: i32 = 4;
pub const ICON: i32 = 5;
pub const FOREGROUND: i32 = 6;
pub const STYLE: i32 = 7;
pub const IS_DIR: i32 = 8;

#[derive(Debug, PartialEq, Clone)]
pub struct RelatedFileData {
    file_path: String,
    relation: String,
}

#[derive(Debug, Clone)]
pub struct ScmFsoData {
    name: String,
    path: String,
    status: String,
    clean_status: String,
    related_file_data: Option<RelatedFileData>,
    is_dir: bool,
}

pub trait ScmFsoDataIfce {
    fn set_status(&mut self, status: &str);
    fn set_clean_status(&mut self, clean_status: &str);
    fn set_related_file_data(&mut self, related_file_data: &Option<RelatedFileData>);
    fn is_visible(&self, show_hidden: bool, hide_clean: bool) -> bool;
}

impl ScmFsoDataIfce for ScmFsoData {
    fn set_status(&mut self, status: &str) {
        self.status = status.to_string();
    }

    fn set_clean_status(&mut self, clean_status: &str) {
        self.clean_status = clean_status.to_string();
    }

    fn set_related_file_data(&mut self, related_file_data: &Option<RelatedFileData>) {
        self.related_file_data = related_file_data.clone();
    }

    fn is_visible(&self, show_hidden: bool, hide_clean: bool) -> bool {
        if self.is_dir {
            if show_hidden {
                if hide_clean {
                    !self.is_clean_dir()
                } else {
                    true
                }
            } else if hide_clean {
                !self.is_clean_dir() && !self.is_hidden_dir()
            } else {
                !self.is_hidden_dir()
            }
        } else if show_hidden {
            if hide_clean {
                !self.is_clean_file()
            } else {
                true
            }
        } else if hide_clean {
            !self.is_clean_file() && !self.is_hidden_file()
        } else {
            !self.is_hidden_file()
        }
    }
}

impl ScmFsoData {
    fn is_hidden_dir(&self) -> bool {
        if self.name.starts_with('.') {
            !SIGNIFICANT_SET.contains(&self.status.as_str())
                && !SIGNIFICANT_SET.contains(&self.clean_status.as_str())
        } else {
            self.status == IGNORED
        }
    }

    fn is_clean_dir(&self) -> bool {
        CLEAN_SET.contains(&self.status.as_str())
            && !SIGNIFICANT_SET.contains(&self.clean_status.as_str())
    }

    fn is_hidden_file(&self) -> bool {
        if self.name.starts_with('.') {
            !SIGNIFICANT_SET.contains(&self.status.as_str())
        } else {
            self.status == IGNORED
        }
    }

    fn is_clean_file(&self) -> bool {
        CLEAN_SET.contains(&self.status.as_str())
    }

    fn get_rfd_from_row<S: TreeRowOps>(store: &S, iter: &TreeIter) -> Option<RelatedFileData> {
        let relation = store
            .get_value(iter, RELATION)
            .get::<String>()
            .unwrap()
            .unwrap();
        if relation.is_empty() {
            None
        } else {
            let file_path = store
                .get_value(iter, RELATED_FILE)
                .get::<String>()
                .unwrap()
                .unwrap();
            Some(RelatedFileData {
                file_path,
                relation,
            })
        }
    }

    fn set_rfd_in_row<S: TreeRowOps>(&self, store: &S, iter: &TreeIter) {
        if let Some(ref rfd) = self.related_file_data {
            store.set_value(iter, RELATED_FILE as u32, &rfd.file_path.to_value());
            store.set_value(iter, RELATION as u32, &rfd.relation.to_value());
        } else {
            store.set_value(iter, RELATED_FILE as u32, &"".to_value());
            store.set_value(iter, RELATION as u32, &"".to_value());
        }
    }

    fn set_icon_in_row<S: TreeRowOps>(&self, store: &S, iter: &TreeIter) {
        if self.is_dir {
            store.set_value(iter, ICON as u32, &"gtk-directory".to_value());
        } else {
            store.set_value(iter, ICON as u32, &"gtk-file".to_value());
        }
    }
}

impl FsObjectIfce for ScmFsoData {
    fn new(name: &str, path: &str, is_dir: bool) -> Self {
        ScmFsoData {
            name: name.to_string(),
            path: path.to_string(),
            status: NO_STATUS.to_string(),
            clean_status: NO_STATUS.to_string(),
            related_file_data: None,
            is_dir,
        }
    }

    fn from_dir_entry(dir_entry: &UsableDirEntry) -> Self {
        ScmFsoData {
            name: dir_entry.file_name(),
            path: dir_entry.path().to_string_lossy().into_owned(),
            status: NO_STATUS.to_string(),
            clean_status: NO_STATUS.to_string(),
            related_file_data: None,
            is_dir: dir_entry.is_dir(),
        }
    }

    fn tree_store_spec() -> Vec<glib::Type> {
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
        col.add_attribute(&cell, "text", STATUS);
        col.add_attribute(&cell, "foreground", FOREGROUND);
        col.add_attribute(&cell, "style", STYLE);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", NAME);
        col.add_attribute(&cell, "foreground", FOREGROUND);
        col.add_attribute(&cell, "style", STYLE);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", RELATION);
        col.add_attribute(&cell, "foreground", FOREGROUND);
        col.add_attribute(&cell, "style", STYLE);

        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", RELATED_FILE);
        col.add_attribute(&cell, "foreground", FOREGROUND);
        col.add_attribute(&cell, "style", STYLE);

        vec![col]
    }

    fn row_is_a_dir<S: TreeRowOps>(store: &S, iter: &TreeIter) -> bool {
        store
            .get_value(iter, IS_DIR)
            .get::<bool>()
            .unwrap()
            .unwrap()
    }

    fn row_is_place_holder<S: TreeRowOps>(store: &S, iter: &TreeIter) -> bool {
        store
            .get_value(iter, NAME)
            .get::<String>()
            .unwrap()
            .unwrap()
            .as_str()
            == "(empty)"
    }

    fn get_name_from_row<S: TreeRowOps>(store: &S, iter: &TreeIter) -> String {
        store
            .get_value(iter, NAME)
            .get::<String>()
            .unwrap()
            .unwrap()
    }

    fn get_path_from_row<S: TreeRowOps>(store: &S, iter: &TreeIter) -> String {
        store
            .get_value(iter, PATH)
            .get::<String>()
            .unwrap()
            .unwrap()
    }

    fn update_row_if_required<S: TreeRowOps>(&self, store: &S, iter: &TreeIter) -> bool {
        assert_eq!(
            self.name,
            store
                .get_value(iter, NAME)
                .get::<String>()
                .unwrap()
                .unwrap()
        );
        let mut changed = false;
        if self.path
            != store
                .get_value(iter, PATH)
                .get::<String>()
                .unwrap()
                .unwrap()
        {
            store.set_value(iter, PATH as u32, &self.path.to_value());
            changed = true;
        }
        if self.status
            != store
                .get_value(iter, STATUS)
                .get::<String>()
                .unwrap()
                .unwrap()
        {
            store.set_value(iter, STATUS as u32, &self.status.to_value());
            let (style, foreground) = get_deco(self.status.as_str());
            store.set_value(iter, STYLE as u32, &style.to_value());
            store.set_value(iter, FOREGROUND as u32, &foreground.to_value());
            changed = true;
        }
        if self.related_file_data != ScmFsoData::get_rfd_from_row(store, iter) {
            self.set_rfd_in_row(store, iter);
            changed = true;
        }
        if self.is_dir
            != store
                .get_value(iter, IS_DIR)
                .get::<bool>()
                .unwrap()
                .unwrap()
        {
            store.set_value(iter, IS_DIR as u32, &self.is_dir.to_value());
            self.set_icon_in_row(store, iter);
            changed = true;
        }
        changed
    }

    fn set_row_values<S: TreeRowOps>(&self, store: &S, iter: &TreeIter) {
        store.set_value(iter, NAME as u32, &self.name.to_value());
        store.set_value(iter, PATH as u32, &self.path.to_value());
        store.set_value(iter, STATUS as u32, &self.status.to_value());
        self.set_rfd_in_row(store, iter);
        self.set_icon_in_row(store, iter);
        let (style, foreground) = get_deco(self.status.as_str());
        store.set_value(iter, STYLE as u32, &style.to_value());
        store.set_value(iter, FOREGROUND as u32, &foreground.to_value());
        store.set_value(iter, IS_DIR as u32, &self.is_dir.to_value());
    }

    fn set_place_holder_values<S: TreeRowOps>(store: &S, iter: &TreeIter) {
        store.set_value(iter, NAME as u32, &"(empty)".to_value());
        store.set_value(iter, PATH as u32, &"".to_value());
        store.set_value(iter, IS_DIR as u32, &false.to_value());
        store.set_value(iter, FOREGROUND as u32, &"purple".to_value());
        store.set_value(iter, STYLE as u32, &PANGO_STYLE_OBLIQUE.to_value());
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

lazy_static! {
    static ref MODIFIED_LIST: [&'static str; 23] = [
        // TODO: review order of modified set re directory decoration
        // order is preference order for directory decoration based on contents' states
        WD_ONLY_MODIFIED, WD_ONLY_DELETED,
        MODIFIED_MODIFIED, MODIFIED_DELETED,
        ADDED_MODIFIED, ADDED_DELETED,
        DELETED_MODIFIED,
        RENAMED_MODIFIED, RENAMED_DELETED,
        COPIED_MODIFIED, COPIED_DELETED,
        UNMERGED,
        UNMERGED_ADDED, UNMERGED_ADDED_US, UNMERGED_ADDED_THEM,
        UNMERGED_DELETED, UNMERGED_DELETED_US, UNMERGED_DELETED_THEM,
        MODIFIED, ADDED, DELETED, RENAMED, COPIED,
    ];

    static ref MODIFIED_SET: HashSet<&'static str> = {
        let mut s: HashSet<&'static str> = HashSet::new();
        for status in MODIFIED_LIST.iter() {
            s.insert(status);
        }
        s
    };

    static ref CLEAN_SET: HashSet<&'static str> = {
        let mut s: HashSet<&'static str> = HashSet::new();
        for status in [UNMODIFIED, MODIFIED, ADDED, DELETED, RENAMED, COPIED, IGNORED, NO_STATUS].iter() {
            s.insert(status);
        }
        s
    };

    static ref SIGNIFICANT_SET: HashSet<&'static str> = {
        let mut s: HashSet<&'static str> = MODIFIED_SET.clone();
        s.insert(NOT_TRACKED);
        s
    };

    static ref ORDERED_DIR_STATUS_LIST: Vec<&'static str> = {
        let mut v = MODIFIED_LIST.to_vec();
        v.push(NOT_TRACKED);
        v
    };

    static ref ORDERED_DIR_CLEAN_STATUS_LIST: Vec<&'static str> = {
        let mut v: Vec<&'static str> = MODIFIED_LIST.iter().filter(|x| !CLEAN_SET.contains(*x)).copied().collect();
        v.push(NOT_TRACKED);
        v
    };
}

fn is_ignored_path(path: &str) -> bool {
    // TODO: examine caching ignore::gitignore::Gitignore instances
    use ignore::gitignore::Gitignore;
    if ".gitignore".path_is_file() {
        let (git_ignore, _) = Gitignore::new(".gitignore");
        git_ignore.matched(path, true).is_ignore()
    } else {
        let (git_ignore, _) = Gitignore::global();
        git_ignore.matched(path, true).is_ignore()
    }
}

fn first_status_in_set(
    status_list: &[&'static str],
    status_set: &HashSet<&str>,
    path: Option<&str>,
) -> &'static str {
    for status in status_list.iter() {
        if status_set.contains(status) {
            return status;
        }
    }
    let ignored = if let Some(path) = path {
        is_ignored_path(path)
    } else {
        is_ignored_path(".")
    };
    if ignored {
        IGNORED
    } else {
        NO_STATUS
    }
}

type FileStatusData = Rc<HashMap<String, (String, Option<RelatedFileData>)>>;

#[derive(Debug, Clone)]
struct Snapshot {
    num_dir_components: usize,
    file_status_data: FileStatusData,
    relevant_keys: Vec<String>,
    status: String,
    clean_status: String,
}

impl Snapshot {
    fn iter(&self) -> SnapshotIterator<'_> {
        SnapshotIterator {
            num_dir_components: self.num_dir_components,
            file_status_data: Rc::clone(&self.file_status_data),
            relevant_keys_iter: self.relevant_keys.iter(),
            already_seen: HashSet::new(),
        }
    }

    fn narrowed_for_dir_path(&self, dir_path: &str) -> Self {
        let relevant_keys: Vec<String> = self
            .file_status_data
            .keys()
            .filter(|k| k.path_starts_with(dir_path))
            .map(|s| s.to_string())
            .collect();
        let mut status_set = HashSet::new();
        for key in relevant_keys.iter() {
            let (status, _) = self.file_status_data.get(key).unwrap();
            status_set.insert(status.as_str());
        }
        let status = first_status_in_set(&ORDERED_DIR_STATUS_LIST, &status_set, Some(dir_path));
        let clean_status =
            first_status_in_set(&ORDERED_DIR_CLEAN_STATUS_LIST, &status_set, Some(dir_path));
        Self {
            num_dir_components: dir_path.path_components().len(),
            file_status_data: Rc::clone(&self.file_status_data),
            relevant_keys,
            status: status.to_string(),
            clean_status: clean_status.to_string(),
        }
    }
}

struct SnapshotIterator<'a> {
    num_dir_components: usize,
    file_status_data: FileStatusData,
    relevant_keys_iter: Iter<'a, String>,
    already_seen: HashSet<String>,
}

impl<'a> Iterator for SnapshotIterator<'a> {
    type Item = (String, String, bool, String, Option<RelatedFileData>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(file_path) = self.relevant_keys_iter.next() {
                let components = file_path.path_components();
                if components.len() == self.num_dir_components {
                    // This is me and I'm a submodule
                    continue;
                }
                let (status, related_file_data) = self.file_status_data.get(file_path).unwrap();
                let path = components[..self.num_dir_components + 1].to_string_path();
                let is_dir = components.len() > self.num_dir_components + 1 || path.path_is_dir();
                let name = components[self.num_dir_components].to_string();
                if self.already_seen.contains(&name) {
                    continue;
                };
                self.already_seen.insert(name.clone());
                return Some((
                    name,
                    path,
                    is_dir,
                    status.to_string(),
                    related_file_data.clone(),
                ));
            } else {
                return None;
            }
        }
    }
}

fn get_snapshot_text() -> (String, Vec<u8>) {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .arg("--ignored")
        .arg("--untracked=all")
        .arg("--ignore-submodules=none")
        .output()
        .expect("get_snapshot_text() failed");
    if output.status.success() {
        let mut hasher = Hasher::new(Algorithm::SHA256);
        hasher.write_all(&output.stdout).expect("hasher blew up!!!");
        (
            String::from_utf8_lossy(&output.stdout).to_string(),
            hasher.finish(),
        )
    } else {
        ("".to_string(), vec![])
    }
}

lazy_static! {
    static ref GIT_FILE_DATA_RE: Regex =
        Regex::new(r###"(("([^"]+)")|(\S+))( -> (("([^"]+)")|(\S+)))?"###).unwrap();
}

macro_rules! parse_line {
    ( $line:ident ) => {{
        let captures = GIT_FILE_DATA_RE.captures(&$line[3..]).unwrap();
        let path = if let Some(path) = captures.get(3) {
            path
        } else {
            captures.get(4).unwrap()
        };
        let related_file_data = if captures.get(5).is_some() {
            let file_path = if let Some(path) = captures.get(8) {
                path
            } else {
                captures.get(9).unwrap()
            };
            Some(RelatedFileData {
                file_path: file_path.as_str().to_string(),
                relation: "->".to_string(),
            })
        } else {
            None
        };
        (
            // git doesn't include "./" but we have to for read_dir() so add for compatibility
            format!(".{}{}", MAIN_SEPARATOR, path.as_str()),
            $line[..2].to_string(),
            related_file_data,
        )
    }};
}

fn extract_snapshot_from_text(text: &str) -> Snapshot {
    let mut rfd_data: Vec<(String, RelatedFileData)> = vec![];
    let mut file_status_data: HashMap<String, (String, Option<RelatedFileData>)> = HashMap::new();
    for line in text.lines() {
        let (file_path, status, related_file_data) = parse_line!(line);
        if let Some(ref rfd) = related_file_data {
            rfd_data.push((file_path.to_string(), rfd.clone()));
        }
        file_status_data.insert(
            file_path.to_string(),
            (status.to_string(), related_file_data),
        );
    }
    // TODO: add "goes to" related file data
    //for file_path, related_file_path in related_file_path_data:
    //    data = fsd.get(related_file_path, None)
    //    if data is not None:
    //        # don't overwrite git's opinion on related file data if it had one
    //        if data[1] is not None: continue
    //        status = data[0]
    //    else:
    //        stdout = runext.run_get_cmd(["git", "status", "--porcelain", "--", related_file_path], default="")
    //        status = stdout[:2] if stdout else None
    //    fsd[related_file_path] = (status, fsdb.RFD(path=file_path, relation="<-"))
    let file_status_data = Rc::new(file_status_data);
    let relevant_keys: Vec<String> = file_status_data.keys().map(|s| s.to_string()).collect();
    let status: String;
    let clean_status: String;
    {
        let status_set: HashSet<&str> =
            file_status_data.values().map(|(a, _)| a.as_str()).collect();
        status = first_status_in_set(&ORDERED_DIR_STATUS_LIST, &status_set, None).to_string();
        clean_status =
            first_status_in_set(&ORDERED_DIR_CLEAN_STATUS_LIST, &status_set, None).to_string();
    }
    Snapshot {
        num_dir_components: 1,
        file_status_data,
        relevant_keys,
        status,
        clean_status,
    }
}

#[derive(Debug)]
struct GitFsDbDir<FSOI>
where
    FSOI: FsObjectIfce + ScmFsoDataIfce,
{
    path: String,
    show_hidden: bool,
    hide_clean: bool,
    dirs_data_unfiltered: Vec<FSOI>,
    files_data_unfiltered: Vec<FSOI>,
    dirs_data: Rc<Vec<FSOI>>,
    files_data: Rc<Vec<FSOI>>,
    hash_digest: Option<Vec<u8>>,
    sub_dirs: HashMap<String, GitFsDbDir<FSOI>>,
    snapshot: Snapshot,
}

impl<FSOI> GitFsDbDir<FSOI>
where
    FSOI: FsObjectIfce + ScmFsoDataIfce + Clone,
{
    fn new(dir_path: &str, snapshot: Snapshot, show_hidden: bool, hide_clean: bool) -> Self {
        Self {
            path: dir_path.to_string(),
            show_hidden,
            hide_clean,
            dirs_data_unfiltered: vec![],
            files_data_unfiltered: vec![],
            dirs_data: Rc::new(vec![]),
            files_data: Rc::new(vec![]),
            hash_digest: None,
            sub_dirs: HashMap::new(),
            snapshot,
        }
    }

    fn current_hash_digest(&self) -> Vec<u8> {
        let mut hasher = Hasher::new(Algorithm::SHA256);
        if let Ok(dir_entries) = UsableDirEntry::get_entries(&self.path) {
            for dir_entry in dir_entries {
                let path = dir_entry.path().to_string_lossy().into_owned();
                hasher.write_all(&path.into_bytes()).unwrap()
            }
        }
        hasher.finish()
    }

    fn is_current(&self) -> bool {
        match self.hash_digest {
            None => return true,
            Some(ref hash_digest) => {
                if *hash_digest != self.current_hash_digest() {
                    return false;
                } else {
                    for sub_dir in self.sub_dirs.values() {
                        if !sub_dir.is_current() {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn populate(&mut self) {
        let mut hasher = Hasher::new(Algorithm::SHA256);
        if let Ok(dir_entries) = UsableDirEntry::get_entries(&self.path) {
            let mut dirs_map = HashMap::new();
            let mut files_map = HashMap::new();
            for dir_entry in dir_entries {
                let path = dir_entry.path().to_string_path();
                hasher.write_all(&path.into_bytes()).unwrap();
                let name = dir_entry.file_name();
                if dir_entry.is_dir() {
                    let path = dir_entry.path().to_string_lossy().into_owned();
                    dirs_map.insert(name, FSOI::from_dir_entry(&dir_entry));
                    let snapshot = self.snapshot.narrowed_for_dir_path(&path);
                    self.sub_dirs.insert(
                        dir_entry.file_name(),
                        GitFsDbDir::<FSOI>::new(&path, snapshot, self.show_hidden, self.hide_clean),
                    );
                } else {
                    files_map.insert(name, FSOI::from_dir_entry(&dir_entry));
                }
            }
            for (name, path, is_dir, status, related_file_data) in self.snapshot.iter() {
                if is_dir {
                    let snapshot = self.snapshot.narrowed_for_dir_path(&path);
                    let status = snapshot.status.clone();
                    let clean_status = snapshot.clean_status.clone();
                    let is_known = dirs_map.contains_key(&name);
                    // TODO: redo this code using insert()'s return value
                    if is_known {
                        let dir_dat = dirs_map.get_mut(&name).expect("stupid rules");
                        dir_dat.set_status(&status);
                        dir_dat.set_clean_status(&clean_status);
                    } else {
                        let mut dir_dat = FSOI::new(&name, &path, is_dir);
                        dir_dat.set_status(&status);
                        dir_dat.set_clean_status(&clean_status);
                        dirs_map.insert(name.clone(), dir_dat);
                        self.sub_dirs.insert(
                            name,
                            GitFsDbDir::<FSOI>::new(
                                &path,
                                snapshot,
                                self.show_hidden,
                                self.hide_clean,
                            ),
                        );
                    }
                } else if let Some(file_data) = files_map.get_mut(&name) {
                    file_data.set_status(&status);
                    file_data.set_related_file_data(&related_file_data);
                } else {
                    let mut file_data = FSOI::new(&name, &path, is_dir);
                    file_data.set_status(&status);
                    file_data.set_related_file_data(&related_file_data);
                    dirs_map.insert(name, file_data);
                }
            }
            self.dirs_data_unfiltered = dirs_map.drain().map(|(_, y)| y).collect();
            self.files_data_unfiltered = files_map.drain().map(|(_, y)| y).collect();
            self.dirs_data_unfiltered
                .sort_unstable_by(|a, b| a.name().partial_cmp(b.name()).unwrap());
            self.files_data_unfiltered
                .sort_unstable_by(|a, b| a.name().partial_cmp(b.name()).unwrap());
            self.filter_data()
        }
        self.hash_digest = Some(hasher.finish());
    }

    fn filter_data(&mut self) {
        let dirs_filtered = self
            .dirs_data_unfiltered
            .iter()
            .filter(|x| x.is_visible(self.show_hidden, self.hide_clean))
            .cloned()
            .collect();
        self.dirs_data = Rc::new(dirs_filtered);
        let files_filtered = self
            .files_data_unfiltered
            .iter()
            .filter(|x| x.is_visible(self.show_hidden, self.hide_clean))
            .cloned()
            .collect();
        self.files_data = Rc::new(files_filtered);
    }

    fn set_visibility(&mut self, show_hidden: bool, hide_clean: bool) {
        self.show_hidden = show_hidden;
        self.hide_clean = hide_clean;
        for sub_dir in self.sub_dirs.values_mut() {
            sub_dir.set_visibility(show_hidden, hide_clean);
        }
    }

    fn re_filter_data(&mut self) {
        if self.hash_digest.is_some() {
            self.filter_data();
            for sub_dir in self.sub_dirs.values_mut() {
                sub_dir.filter_data()
            }
        }
    }

    fn find_dir(&mut self, components: &[StrPathComponent]) -> Option<&mut GitFsDbDir<FSOI>> {
        if self.hash_digest.is_none() {
            self.populate();
        }
        if components.is_empty() {
            Some(self)
        } else {
            assert!(components[0].is_normal());
            let name = components[0].to_string();
            match self.sub_dirs.get_mut(&name) {
                Some(subdir) => subdir.find_dir(&components[1..]),
                None => None,
            }
        }
    }

    fn dirs_and_files(&mut self) -> (Rc<Vec<FSOI>>, Rc<Vec<FSOI>>) {
        (Rc::clone(&self.dirs_data), Rc::clone(&self.files_data))
    }
}

pub struct GitFsDb<FSOI>
where
    FSOI: FsObjectIfce + ScmFsoDataIfce + Clone,
{
    base_dir: RefCell<GitFsDbDir<FSOI>>,
    curr_dir: RefCell<String>, // so we can tell if there's a change of current directory
    snapshot_digest: RefCell<Vec<u8>>,
}

impl<FSOI> FsDbIfce<FSOI> for GitFsDb<FSOI>
where
    FSOI: FsObjectIfce + ScmFsoDataIfce + Clone,
{
    fn honours_hide_clean() -> bool {
        true
    }

    fn honours_show_hidden() -> bool {
        true
    }

    fn new() -> Self {
        let curr_dir = str_path_current_dir_or_panic();
        let (text, snapshot_digest) = get_snapshot_text();
        let snapshot = extract_snapshot_from_text(&text);
        let base_dir = GitFsDbDir::<FSOI>::new(".", snapshot, false, false); // paths are relative
        Self {
            base_dir: RefCell::new(base_dir),
            curr_dir: RefCell::new(curr_dir),
            snapshot_digest: RefCell::new(snapshot_digest),
        }
    }

    fn dir_contents(
        &self,
        dir_path: &str,
        show_hidden: bool,
        hide_clean: bool,
    ) -> (Rc<Vec<FSOI>>, Rc<Vec<FSOI>>) {
        assert!(dir_path.path_is_relative());
        self.check_visibility(show_hidden, hide_clean);
        let components = dir_path.to_string().path_components();
        assert!(components[0].is_cur_dir());
        if let Some(ref mut dir) = self.base_dir.borrow_mut().find_dir(&components[1..]) {
            dir.dirs_and_files()
        } else {
            (Rc::new(vec![]), Rc::new(vec![]))
        }
    }

    fn update_if_necessary(&self) -> bool {
        if self.curr_dir_changed() {
            self.reset();
            true
        } else {
            let (text, digest) = get_snapshot_text();
            let mut snapshot_digest = self.snapshot_digest.borrow_mut();
            if digest != *snapshot_digest {
                *snapshot_digest = digest;
                let snapshot = extract_snapshot_from_text(&text);
                *self.base_dir.borrow_mut() = GitFsDbDir::new(".", snapshot, false, false);
                true
            } else if !self.base_dir.borrow_mut().is_current() {
                let mut base_dir = self.base_dir.borrow_mut();
                let snapshot = base_dir.snapshot.clone();
                *base_dir = GitFsDbDir::new(".", snapshot, false, false);
                true
            } else {
                false
            }
        }
    }

    fn reset(&self) {
        let (text, digest) = get_snapshot_text();
        let snapshot = extract_snapshot_from_text(&text);
        *self.snapshot_digest.borrow_mut() = digest;
        *self.curr_dir.borrow_mut() = str_path_current_dir_or_panic();
        *self.base_dir.borrow_mut() = GitFsDbDir::new(".", snapshot, false, false);
    }
}

impl<FSOI> GitFsDb<FSOI>
where
    FSOI: FsObjectIfce + ScmFsoDataIfce + Clone,
{
    fn curr_dir_changed(&self) -> bool {
        *self.curr_dir.borrow() != str_path_current_dir_or_panic()
    }

    fn check_visibility(&self, show_hidden: bool, hide_clean: bool) {
        let mut base_dir = self.base_dir.borrow_mut();
        if base_dir.show_hidden != show_hidden || base_dir.hide_clean != hide_clean {
            base_dir.set_visibility(show_hidden, hide_clean);
            base_dir.re_filter_data();
        }
    }
}

#[derive(Debug)]
struct GitIndexDbDir<FSOI>
where
    FSOI: FsObjectIfce + ScmFsoDataIfce + Clone,
{
    path: String,
    hide_clean: bool,
    dirs_data_unfiltered: Vec<FSOI>,
    files_data_unfiltered: Vec<FSOI>,
    dirs_data: Rc<Vec<FSOI>>,
    files_data: Rc<Vec<FSOI>>,
    sub_dirs: HashMap<String, GitIndexDbDir<FSOI>>,
    status_set: HashSet<String>,
}

fn first_status_in_list_in_set(
    status_list: &[&'static str],
    status_set: &HashSet<String>,
) -> &'static str {
    for status in status_list.iter() {
        if status_set.contains(&status.to_string()) {
            return status;
        }
    }
    NO_STATUS
}

impl<FSOI> GitIndexDbDir<FSOI>
where
    FSOI: FsObjectIfce + ScmFsoDataIfce + Clone,
{
    fn new(path: &str, status: &str, hide_clean: bool) -> Self {
        let mut status_set = HashSet::new();
        status_set.insert(status.to_string());
        Self {
            path: path.to_string(),
            hide_clean,
            dirs_data_unfiltered: vec![],
            files_data_unfiltered: vec![],
            dirs_data: Rc::new(vec![]),
            files_data: Rc::new(vec![]),
            sub_dirs: HashMap::new(),
            status_set,
        }
    }

    fn add_file(
        &mut self,
        path_components: &[StrPathComponent],
        status: &str,
        related_file_data: &Option<RelatedFileData>,
    ) {
        self.status_set.insert(status.to_string());
        let name = path_components[0].to_string();
        let path = self.path.path_join(&name);
        // NB: handle the case where this may be a submodule directory
        if path_components.len() > 1 || path.path_is_dir() {
            if !self.sub_dirs.contains_key(&name) {
                let dir = Self::new(&path, status, self.hide_clean);
                self.sub_dirs.insert(name.to_string(), dir);
            }
            if path_components.len() > 1 {
                self.sub_dirs.get_mut(&name).expect("wtf?").add_file(
                    &path_components[1..],
                    status,
                    related_file_data,
                );
            }
        } else {
            let mut file_data = FSOI::new(&name, &path, false);
            file_data.set_status(status);
            file_data.set_related_file_data(related_file_data);
            self.files_data_unfiltered.push(file_data);
        }
    }

    fn find_dir(&self, components: &[StrPathComponent]) -> Option<&GitIndexDbDir<FSOI>> {
        if components.is_empty() {
            Some(self)
        } else {
            assert!(components[0].is_normal());
            let name = components[0].to_string();
            match self.sub_dirs.get(&name) {
                Some(subdir) => subdir.find_dir(&components[1..]),
                None => None,
            }
        }
    }

    fn finalize(&mut self) {
        self.files_data_unfiltered
            .sort_unstable_by(|a, b| a.name().partial_cmp(b.name()).unwrap());
        for (name, sub_dir) in self.sub_dirs.iter_mut() {
            sub_dir.finalize();
            let mut dir_data = FSOI::new(name, &sub_dir.path, true);
            let status = first_status_in_list_in_set(&ORDERED_DIR_STATUS_LIST, &sub_dir.status_set);
            dir_data.set_status(status);
            let clean_status =
                first_status_in_list_in_set(&ORDERED_DIR_CLEAN_STATUS_LIST, &sub_dir.status_set);
            dir_data.set_clean_status(clean_status);
            self.dirs_data_unfiltered.push(dir_data);
        }
        self.dirs_data_unfiltered
            .sort_unstable_by(|a, b| a.name().partial_cmp(b.name()).unwrap());
        self.filter_data();
    }

    fn filter_data(&mut self) {
        let dirs_filtered = self
            .dirs_data_unfiltered
            .iter()
            .filter(|x| x.is_visible(true, self.hide_clean))
            .cloned()
            .collect();
        self.dirs_data = Rc::new(dirs_filtered);
        let files_filtered = self
            .files_data_unfiltered
            .iter()
            .filter(|x| x.is_visible(true, self.hide_clean))
            .cloned()
            .collect();
        self.files_data = Rc::new(files_filtered);
    }

    fn set_visibility(&mut self, hide_clean: bool) {
        self.hide_clean = hide_clean;
        for sub_dir in self.sub_dirs.values_mut() {
            sub_dir.set_visibility(hide_clean);
        }
    }

    fn re_filter_data(&mut self) {
        self.filter_data();
        for sub_dir in self.sub_dirs.values_mut() {
            sub_dir.filter_data()
        }
    }
}

pub struct GitIndexDb<FSOI>
where
    FSOI: FsObjectIfce + ScmFsoDataIfce + Clone,
{
    base_dir: RefCell<GitIndexDbDir<FSOI>>,
    populated_digest: RefCell<Vec<u8>>,
}

fn get_digest_text() -> (String, Vec<u8>) {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .arg("--untracked-files=no")
        .arg("--ignore-submodules=none")
        .output()
        .expect("get_digest_text() failed");
    if output.status.success() {
        let mut hasher = Hasher::new(Algorithm::SHA256);
        hasher.write_all(&output.stdout).expect("hasher blew up!!!");
        (
            String::from_utf8_lossy(&output.stdout).to_string(),
            hasher.finish(),
        )
    } else {
        ("".to_string(), vec![])
    }
}

impl<FSOI> FsDbIfce<FSOI> for GitIndexDb<FSOI>
where
    FSOI: FsObjectIfce + ScmFsoDataIfce + Clone,
{
    fn honours_hide_clean() -> bool {
        true
    }

    fn honours_show_hidden() -> bool {
        false
    }

    fn new() -> Self {
        let base_dir = GitIndexDbDir::<FSOI>::new(".", NO_STATUS, false); // paths are relative
        let gib = Self {
            base_dir: RefCell::new(base_dir),
            populated_digest: RefCell::new(vec![]),
        };
        let (text, text_digest) = get_digest_text();
        gib.populate(&text, &text_digest);

        gib
    }

    fn dir_contents(
        &self,
        dir_path: &str,
        _show_hidden: bool,
        hide_clean: bool,
    ) -> (Rc<Vec<FSOI>>, Rc<Vec<FSOI>>) {
        assert!(dir_path.path_is_relative());
        self.check_visibility(hide_clean);
        let components = dir_path.to_string().path_components();
        assert!(components[0].is_cur_dir());
        if let Some(ref mut dir) = self.base_dir.borrow_mut().find_dir(&components[1..]) {
            (Rc::clone(&dir.dirs_data), Rc::clone(&dir.files_data))
        } else {
            (Rc::new(vec![]), Rc::new(vec![]))
        }
    }

    fn update_if_necessary(&self) -> bool {
        let (text, digest) = get_digest_text();
        if digest != *self.populated_digest.borrow() {
            self.populate(&text, &digest);
            true
        } else {
            false
        }
    }

    fn reset(&self) {
        let (text, digest) = get_digest_text();
        self.populate(&text, &digest);
    }
}

impl<FSOI> GitIndexDb<FSOI>
where
    FSOI: FsObjectIfce + ScmFsoDataIfce + Clone,
{
    fn check_visibility(&self, hide_clean: bool) {
        let mut base_dir = self.base_dir.borrow_mut();
        if base_dir.hide_clean != hide_clean {
            base_dir.set_visibility(hide_clean);
            base_dir.re_filter_data();
        }
    }

    fn populate(&self, text: &str, digest: &[u8]) {
        let mut base_dir = self.base_dir.borrow_mut();
        let hide_clean = base_dir.hide_clean;
        *base_dir = GitIndexDbDir::new(".", NO_STATUS, hide_clean);
        for line in text.lines() {
            if line.starts_with(' ') {
                continue; // not in the index
            }
            let (file_path, status, related_file_data) = parse_line!(line);
            let path_components = file_path.path_components();
            base_dir.add_file(&path_components[1..], &status, &related_file_data)
        }
        base_dir.finalize();
        *self.populated_digest.borrow_mut() = digest.to_vec();
    }
}
