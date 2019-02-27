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
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::iter::Iterator;
use std::rc::Rc;
use std::slice::Iter;

use gtk::prelude::*;
use gtk::{StaticType, ToValue, TreeIter};
use pango_sys::{PANGO_STYLE_ITALIC, PANGO_STYLE_NORMAL, PANGO_STYLE_OBLIQUE};

use crypto_hash::{Algorithm, Hasher};
use git2;
use regex::Regex;

use pw_gix::fs_db::{FsDbIfce, FsObjectIfce, TreeRowOps};

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
    static ref SCM_FS_DB_ROW_SPEC: [gtk::Type; 9] =
        [
            gtk::Type::String,          // 0 Name
            gtk::Type::String,          // 1 Path
            gtk::Type::String,          // 2 Status
            gtk::Type::String,          // 3 AssociatedFile
            gtk::Type::String,          // 4 Relation
            gtk::Type::String,          // 5 Icon
            gtk::Type::String,          // 6 Foreground
            gtk::Type::I32,             // 7 Style
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

const NAME: i32 = 0;
const PATH: i32 = 1;
const STATUS: i32 = 2;
const RELATED_FILE: i32 = 3;
const RELATION: i32 = 4;
const ICON: i32 = 5;
const FOREGROUND: i32 = 6;
const STYLE: i32 = 7;
const IS_DIR: i32 = 8;

#[derive(Debug, PartialEq, Clone)]
pub struct RelatedFileData {
    file_path: String,
    relation: String,
}

#[derive(Debug)]
pub struct ScmFsoData {
    name: String,
    path: String,
    status: String,
    related_file_data: Option<RelatedFileData>,
    is_dir: bool,
}

impl ScmFsoData {
    fn get_rfd_from_row<S: TreeRowOps>(store: &S, iter: &TreeIter) -> Option<RelatedFileData> {
        let relation = store.get_value(iter, RELATION).get::<String>().unwrap();
        if relation.len() == 0 {
            None
        } else {
            let file_path = store.get_value(iter, RELATED_FILE).get::<String>().unwrap();
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
    fn new(dir_entry: &UsableDirEntry) -> Self {
        ScmFsoData {
            name: dir_entry.file_name(),
            path: dir_entry.path().to_string_lossy().into_owned(),
            status: NO_STATUS.to_string(),
            related_file_data: None,
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
            let (style, foreground) = get_deco(&self.status.as_str());
            store.set_value(iter, STYLE as u32, &style.to_value());
            store.set_value(iter, FOREGROUND as u32, &foreground.to_value());
            changed = true;
        }
        if self.related_file_data != ScmFsoData::get_rfd_from_row(store, iter) {
            self.set_rfd_in_row(store, iter);
            changed = true;
        }
        if self.is_dir != store.get_value(iter, IS_DIR).get::<bool>().unwrap() {
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
        let mut v: Vec<&'static str> = MODIFIED_LIST.iter().filter(|x| !CLEAN_SET.contains(*x)).map(|x| *x).collect();
        v.push(NOT_TRACKED);
        v
    };
}

fn is_ignored_path(path: &str) -> bool {
    match git2::Repository::open(".") {
        Ok(repo) => match repo.is_path_ignored(path) {
            Ok(is_ignored) => is_ignored,
            Err(_) => false,
        },
        Err(_) => false,
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

struct Snapshot<'a> {
    num_dir_components: usize,
    file_status_data: FileStatusData,
    relevant_keys: Vec<String>,
    status: &'a str,
    clean_status: &'a str,
}

impl<'a> Snapshot<'a> {
    fn iter(&self) -> SnapshotIterator {
        SnapshotIterator {
            num_dir_components: self.num_dir_components,
            file_status_data: Rc::clone(&self.file_status_data),
            relevant_keys_iter: self.relevant_keys.iter(),
        }
    }

    fn narrowed_for_dir_path(&'a self, dir_path: &str) -> Self {
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
            relevant_keys: relevant_keys,
            status: status,
            clean_status: clean_status,
        }
    }
}

struct SnapshotIterator<'a> {
    num_dir_components: usize,
    file_status_data: FileStatusData,
    relevant_keys_iter: Iter<'a, String>,
}

impl<'a> Iterator for SnapshotIterator<'a> {
    type Item = (String, String, String, bool, Option<RelatedFileData>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(file_path) = self.relevant_keys_iter.next() {
                let (status, related_file_data) = self.file_status_data.get(&*file_path).unwrap();
                let mut components = file_path.path_components();
                // git doesn't include "./" but have to for read_dir() so add for compatibility
                components.insert(0, StrPathComponent::CurDir);
                let is_dir =
                    components.len() > self.num_dir_components + 1 || file_path.path_is_dir();
                let name = components[self.num_dir_components + 1].to_string();
                let path = components[..self.num_dir_components + 1].to_string_path();
                return Some((
                    name,
                    path,
                    status.to_string(),
                    is_dir,
                    related_file_data.clone(),
                ));
            } else {
                return None;
            }
        }
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
            path.as_str().to_string(),
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
    let status_set: HashSet<&str> = file_status_data.values().map(|(a, _)| a.as_str()).collect();
    let status = first_status_in_set(&ORDERED_DIR_STATUS_LIST, &status_set, None);
    let clean_status = first_status_in_set(&ORDERED_DIR_CLEAN_STATUS_LIST, &status_set, None);
    Snapshot {
        num_dir_components: 1,
        file_status_data: file_status_data,
        relevant_keys: relevant_keys,
        status,
        clean_status,
    }
}

#[derive(Debug)]
struct GitFsDbDir<FSOI>
where
    FSOI: FsObjectIfce,
{
    path: String,
    show_hidden: bool,
    hide_clean: bool,
    dirs_data: Rc<Vec<FSOI>>,
    files_data: Rc<Vec<FSOI>>,
    hash_digest: Option<Vec<u8>>,
    sub_dirs: HashMap<String, GitFsDbDir<FSOI>>,
}

impl<FSOI> GitFsDbDir<FSOI>
where
    FSOI: FsObjectIfce,
{
    fn new(dir_path: &str, show_hidden: bool, hide_clean: bool) -> Self {
        Self {
            path: dir_path.to_string(),
            show_hidden: show_hidden,
            hide_clean: hide_clean,
            dirs_data: Rc::new(vec![]),
            files_data: Rc::new(vec![]),
            hash_digest: None,
            sub_dirs: HashMap::new(),
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
            let mut dirs = vec![];
            let mut files = vec![];
            for dir_entry in dir_entries {
                let path = dir_entry.path().to_string_lossy().into_owned();
                hasher.write_all(&path.into_bytes()).unwrap();
                if !self.show_hidden && dir_entry.file_name().starts_with(".") {
                    continue;
                }
                if dir_entry.is_dir() {
                    let path = dir_entry.path().to_string_lossy().into_owned();
                    dirs.push(FSOI::new(&dir_entry));
                    self.sub_dirs.insert(
                        dir_entry.file_name(),
                        GitFsDbDir::<FSOI>::new(&path, self.show_hidden, self.hide_clean),
                    );
                } else {
                    files.push(FSOI::new(&dir_entry));
                }
            }
            dirs.sort_unstable_by(|a, b| a.name().partial_cmp(b.name()).unwrap());
            files.sort_unstable_by(|a, b| a.name().partial_cmp(b.name()).unwrap());
            self.dirs_data = Rc::new(dirs);
            self.files_data = Rc::new(files);
        }
        self.hash_digest = Some(hasher.finish());
    }

    fn find_dir(&mut self, components: &[StrPathComponent]) -> Option<&mut GitFsDbDir<FSOI>> {
        if self.hash_digest.is_none() {
            self.populate();
        }
        if components.len() == 0 {
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

    fn dirs_and_files<'a>(&'a mut self) -> (Rc<Vec<FSOI>>, Rc<Vec<FSOI>>) {
        (Rc::clone(&self.dirs_data), Rc::clone(&self.files_data))
    }
}

pub struct GitFsDb<FSOI>
where
    FSOI: FsObjectIfce,
{
    base_dir: RefCell<GitFsDbDir<FSOI>>,
    curr_dir: RefCell<String>, // so we can tell if there's a change of current directory
}

impl<FSOI> FsDbIfce<FSOI> for GitFsDb<FSOI>
where
    FSOI: FsObjectIfce,
{
    fn honours_hide_clean() -> bool {
        false
    }

    fn honours_show_hidden() -> bool {
        true
    }

    fn new() -> Self {
        let curr_dir = str_path_current_dir_or_panic();
        let base_dir = GitFsDbDir::<FSOI>::new("./", false, false); // paths are relative
        Self {
            base_dir: RefCell::new(base_dir),
            curr_dir: RefCell::new(curr_dir),
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

    fn is_current(&self) -> bool {
        self.curr_dir_unchanged() && self.base_dir.borrow_mut().is_current()
    }

    fn reset(&self) {
        *self.curr_dir.borrow_mut() = str_path_current_dir_or_panic();
        *self.base_dir.borrow_mut() = GitFsDbDir::new("./", false, false);
    }
}

impl<FSOI> GitFsDb<FSOI>
where
    FSOI: FsObjectIfce,
{
    fn curr_dir_unchanged(&self) -> bool {
        *self.curr_dir.borrow() == str_path_current_dir_or_panic()
    }

    fn check_visibility(&self, show_hidden: bool, hide_clean: bool) {
        let mut base_dir = self.base_dir.borrow_mut();
        if base_dir.show_hidden != show_hidden || base_dir.hide_clean != hide_clean {
            *base_dir = GitFsDbDir::new("./", show_hidden, hide_clean);
        }
    }
}
