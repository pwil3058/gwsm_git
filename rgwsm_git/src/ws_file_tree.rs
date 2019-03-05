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

use std::marker::PhantomData;
use std::rc::Rc;

use gtk;
use gtk::prelude::*;

use pw_gix::file_tree::*;
use pw_gix::fs_db::*;
use pw_gix::timeout;
use pw_gix::wrapper::*;

use crate::events;
use crate::exec;
use crate::fs_db::*;

pub struct GenWsFsTree<FSDB, FSOI>
where
    FSDB: FsDbIfce<FSOI> + 'static,
    FSOI: FsObjectIfce + 'static,
{
    v_box: gtk::Box,
    view: gtk::TreeView,
    store: gtk::TreeStore,
    fs_db: FSDB,
    auto_expand: bool,
    show_hidden: gtk::CheckButton,
    hide_clean: gtk::CheckButton,
    controlled_timeout_cycle: Rc<timeout::ControlledTimeoutCycle>,
    exec_console: Rc<exec::ExecConsole>,
    phantom: PhantomData<FSOI>,
}

pub type GitWsFsTree = GenWsFsTree<GitFsDb<ScmFsoData>, ScmFsoData>;

impl_widget_wrapper!(v_box: gtk::Box, GenWsFsTree<FSDB, FSOI>
    where
        FSDB: FsDbIfce<FSOI> + 'static,
        FSOI: FsObjectIfce + 'static,
);

impl<FSDB, FSOI> FileTreeIfce<FSDB, FSOI> for GenWsFsTree<FSDB, FSOI>
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
        self.auto_expand
    }

    fn show_hidden(&self) -> bool {
        self.show_hidden.get_active()
    }

    fn hide_clean(&self) -> bool {
        self.hide_clean.get_active()
    }
}

impl<FSDB, FSOI> GenWsFsTree<FSDB, FSOI>
where
    FSDB: FsDbIfce<FSOI> + 'static,
    FSOI: FsObjectIfce + 'static,
{
    pub fn new(
        exec_console: &Rc<exec::ExecConsole>,
        auto_expand: bool,
    ) -> Rc<Self> {
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let view = gtk::TreeView::new();
        let store = gtk::TreeStore::new(&FSOI::tree_store_spec());
        view.set_model(&store);
        let adj: Option<&gtk::Adjustment> = None;
        let scrolled_window = gtk::ScrolledWindow::new(adj, adj);
        scrolled_window.add(&view);
        v_box.pack_start(&scrolled_window, true, true, 0);
        let show_hidden = gtk::CheckButton::new_with_label("Show Hidden");
        let hide_clean = gtk::CheckButton::new_with_label("Hide Clean");
        if FSDB::honours_show_hidden() || FSDB::honours_hide_clean() {
            let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            if FSDB::honours_show_hidden() {
                h_box.pack_start(&show_hidden, false, false, 0);
            }
            if FSDB::honours_hide_clean() {
                h_box.pack_start(&hide_clean, false, false, 0);
            }
            v_box.pack_start(&h_box, false, false, 0);
        }
        view.set_headers_visible(false);
        for col in FSOI::tree_view_columns() {
            view.append_column(&col);
        }
        let owft = Rc::new(Self {
            v_box: v_box,
            view: view,
            store: store,
            fs_db: FSDB::new(),
            auto_expand: auto_expand,
            show_hidden: show_hidden,
            hide_clean: hide_clean,
            exec_console: Rc::clone(&exec_console),
            controlled_timeout_cycle: timeout::ControlledTimeoutCycle::new("Auto Update", true, 10),
            phantom: PhantomData,
        });
        let owft_clone = Rc::clone(&owft);
        owft.view
            .connect_row_expanded(move |_, dir_iter, _| owft_clone.expand_row(dir_iter));
        let owft_clone = Rc::clone(&owft);
        owft.view.connect_row_collapsed(move |_, dir_iter, _| {
            owft_clone.insert_place_holder_if_needed(dir_iter)
        });
        let owft_clone = Rc::clone(&owft);
        owft.show_hidden.connect_toggled(move |_| {
            owft_clone.update_dir(".", None);
        });
        let owft_clone = Rc::clone(&owft);
        owft.hide_clean.connect_toggled(move |_| {
            owft_clone.update_dir(".", None);
        });
        let owft_clone = Rc::clone(&owft);
        owft.controlled_timeout_cycle
            .register_callback(Box::new(move || {
                owft_clone.update(false);
            }));
        let owft_clone = Rc::clone(&owft);
        owft.exec_console.event_notifier.add_notification_cb(
            events::EV_CHANGE_DIR,
            Box::new(move |_| { owft_clone.repopulate() })
        );
        owft.repopulate();
        owft.view.show_all();
        scrolled_window.show_all();
        owft
    }
}
