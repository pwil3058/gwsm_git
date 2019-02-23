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

use gtk;
use gtk::prelude::*;

use pw_gix::file_tree::*;
use pw_gix::fs_db::*;
use pw_gix::wrapper::*;

use crate::fs_db::*;

pub struct OsWsFsTree {
    v_box: gtk::Box,
    view: gtk::TreeView,
    store: gtk::TreeStore,
    fs_db: OsFsDb<OsFileData, OsFileData>,
    auto_expand: bool,
    show_hidden: bool,
    hide_clean: bool,
}

impl_widget_wrapper!(v_box: gtk::Box, OsWsFsTree);

impl FileTreeIfce<OsFsDb<OsFileData, OsFileData>, OsFileData, OsFileData> for OsWsFsTree {
    fn new(auto_expand: bool) -> Self {
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let view = gtk::TreeView::new();
        let store = gtk::TreeStore::new(&OsFileData::tree_store_spec());
        view.set_model(&store);
        let scrolled_window = gtk::ScrolledWindow::new(None, None);
        scrolled_window.add(&view);
        v_box.pack_start(&scrolled_window, true, true, 0);
        //
        view.set_headers_visible(false);
        let col = gtk::TreeViewColumn::new();
        col.set_title("Name");
        col.set_resizable(false);
        let cell = gtk::CellRendererText::new();
        cell.set_property_editable(false);
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", 0);
        view.append_column(&col);
        //
        let mut owft = OsWsFsTree {
            v_box: v_box,
            view: view,
            store: store,
            fs_db: OsFsDb::<OsFileData, OsFileData>::new(),
            auto_expand: auto_expand,
            show_hidden: false,
            hide_clean: false,
        };
        owft.repopulate();
        owft.view.show_all();
        scrolled_window.show_all();
        owft
    }

    fn view(&self) -> &gtk::TreeView {
        &self.view
    }

    fn store(&self) -> &gtk::TreeStore {
        &self.store
    }

    fn fs_db(&self) -> &OsFsDb<OsFileData, OsFileData> {
        &self.fs_db
    }

    fn auto_expand(&self) -> bool {
        self.auto_expand
    }

    fn show_hidden(&self) -> bool {
        self.show_hidden
    }

    fn hide_clean(&self) -> bool {
        self.hide_clean
    }
}
