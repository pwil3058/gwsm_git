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
use std::marker::PhantomData;
use std::rc::Rc;

use gtk;
use gtk::prelude::*;

use pw_gix::file_tree::FileTreeIfce;
use pw_gix::fs_db::{FsDbIfce, FsObjectIfce};
use pw_gix::gtkx::menu::ManagedMenu;
use pw_gix::sav_state::*;
use pw_gix::wrapper::*;

use crate::events;
use crate::exec;
use crate::fs_db::{GitIndexDb, ScmFsoData};

pub struct GenIndexFsTree<FSDB, FSOI>
where
    FSDB: FsDbIfce<FSOI> + 'static,
    FSOI: FsObjectIfce + 'static,
{
    v_box: gtk::Box,
    view: gtk::TreeView,
    store: gtk::TreeStore,
    fs_db: FSDB,
    hide_clean: gtk::CheckButton,
    exec_console: Rc<exec::ExecConsole>,
    popup_menu: ManagedMenu,
    hovered_fso_path: RefCell<Option<String>>,
    phantom: PhantomData<FSOI>,
}

pub type GitIndexFsTree = GenIndexFsTree<GitIndexDb<ScmFsoData>, ScmFsoData>;

impl_widget_wrapper!(v_box: gtk::Box, GenIndexFsTree<FSDB, FSOI>
    where
        FSDB: FsDbIfce<FSOI> + 'static,
        FSOI: FsObjectIfce + 'static,
);

impl<FSDB, FSOI> FileTreeIfce<FSDB, FSOI> for GenIndexFsTree<FSDB, FSOI>
where
    FSDB: FsDbIfce<FSOI> + 'static,
    FSOI: FsObjectIfce + 'static,
{
    fn view(&self) -> &gtk::TreeView {
        &self.view
    }

    fn store(&self) -> &gtk::TreeStore {
        &self.store
    }

    fn fs_db(&self) -> &FSDB {
        &self.fs_db
    }

    fn auto_expand(&self) -> bool {
        true
    }

    fn show_hidden(&self) -> bool {
        true
    }

    fn hide_clean(&self) -> bool {
        self.hide_clean.get_active()
    }
}

impl<FSDB, FSOI> GenIndexFsTree<FSDB, FSOI>
where
    FSDB: FsDbIfce<FSOI> + 'static,
    FSOI: FsObjectIfce + 'static,
{
    pub fn new(exec_console: &Rc<exec::ExecConsole>) -> Rc<Self> {
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let view = gtk::TreeView::new();
        let store = gtk::TreeStore::new(&FSOI::tree_store_spec());
        view.set_model(&store);
        let adj: Option<&gtk::Adjustment> = None;
        let scrolled_window = gtk::ScrolledWindow::new(adj, adj);
        scrolled_window.add(&view);
        v_box.pack_start(&scrolled_window, true, true, 0);
        let hide_clean = gtk::CheckButton::new_with_label("Hide Clean");
        if FSDB::honours_hide_clean() {
            let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            if FSDB::honours_hide_clean() {
                h_box.pack_start(&hide_clean, false, false, 0);
            }
            v_box.pack_start(&h_box, false, false, 0);
        }
        view.set_headers_visible(false);
        view.get_selection().set_mode(gtk::SelectionMode::Multiple);
        for col in FSOI::tree_view_columns() {
            view.append_column(&col);
        }

        let popup_menu = ManagedMenu::new(
            WidgetStatesControlled::Sensitivity,
            Some(&view.get_selection()),
            Some(&exec_console.changed_condns_notifier),
            &vec![],
        );

        let ift = Rc::new(Self {
            v_box: v_box,
            view: view,
            store: store,
            fs_db: FSDB::new(),
            hide_clean: hide_clean,
            exec_console: Rc::clone(&exec_console),
            popup_menu: popup_menu,
            hovered_fso_path: RefCell::new(None),
            phantom: PhantomData,
        });
        let ift_clone = Rc::clone(&ift);
        ift.view
            .connect_row_expanded(move |_, dir_iter, _| ift_clone.expand_row(dir_iter));
        let ift_clone = Rc::clone(&ift);
        ift.view.connect_row_collapsed(move |_, dir_iter, _| {
            ift_clone.insert_place_holder_if_needed(dir_iter)
        });
        let ift_clone = Rc::clone(&ift);
        ift.hide_clean.connect_toggled(move |_| {
            ift_clone.update_dir(".", None);
        });
        let ift_clone = Rc::clone(&ift);
        ift.exec_console.event_notifier.add_notification_cb(
            events::EV_AUTO_UPDATE | events::EV_CHECKOUT | events::EV_FILES_CHANGE,
            Box::new(move |_| {
                ift_clone.update(false);
            }),
        );
        let ift_clone = Rc::clone(&ift);
        ift.exec_console.event_notifier.add_notification_cb(
            events::EV_CHANGE_DIR,
            Box::new(move |_| ift_clone.repopulate()),
        );

        ift.repopulate();
        ift.view.show_all();
        scrolled_window.show_all();
        ift.v_box.show_all();

        ift
    }

    fn set_hovered_fso_path(&self, path: Option<String>) {
        let condns = self
            .view
            .get_selection()
            .get_masked_conditions_with_hover_ok(path.is_some());
        self.popup_menu.update_condns(condns);
        *self.hovered_fso_path.borrow_mut() = path;
    }
}
