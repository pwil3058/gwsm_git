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

use std::cell::RefCell;
use std::marker::PhantomData;
use std::process::Command;
use std::rc::Rc;

use shlex;

use pw_gix::{
    file_tree::*,
    fs_db::*,
    gtk::{self, prelude::*},
    gtkx::menu_ng::{ManagedMenu, ManagedMenuBuilder},
    sav_state::*,
    wrapper::*,
};

use pw_pathux::str_path::*;

//use crate::action_icons;
use crate::edit;
use crate::events;
use crate::exec;
use crate::fs_db::{self, GitFsDb, ScmFsoData};
use crate::repos;
use crate::submodules;

#[derive(PWO, Wrapper)]
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
    exec_console: Rc<exec::ExecConsole>,
    popup_menu: ManagedMenu,
    hovered_fso_path: RefCell<Option<String>>,
    phantom: PhantomData<FSOI>,
}

pub type GitWsFsTree = GenWsFsTree<GitFsDb<ScmFsoData>, ScmFsoData>;

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
    pub fn new(exec_console: &Rc<exec::ExecConsole>, auto_expand: bool) -> Rc<Self> {
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        v_box.pack_start(&gtk::Label::new(Some("Working Directory")), false, false, 0);
        let view = gtk::TreeView::new();
        let store = gtk::TreeStore::new(&FSOI::tree_store_spec());
        view.set_model(Some(&store));
        let adj: Option<&gtk::Adjustment> = None;
        let scrolled_window = gtk::ScrolledWindow::new(adj, adj);
        scrolled_window.add(&view);
        v_box.pack_start(&scrolled_window, true, true, 0);
        let show_hidden = gtk::CheckButton::with_label("Show Hidden");
        let hide_clean = gtk::CheckButton::with_label("Hide Clean");
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
        view.get_selection().set_mode(gtk::SelectionMode::Multiple);
        for col in FSOI::tree_view_columns() {
            view.append_column(&col);
        }

        let popup_menu = ManagedMenuBuilder::new()
            .widget_states_controlled(WidgetStatesControlled::Sensitivity)
            .selection(&view.get_selection())
            .change_notifier(&exec_console.changed_condns_notifier)
            .build();

        let owft = Rc::new(Self {
            v_box: v_box,
            view: view,
            store: store,
            fs_db: FSDB::new(),
            auto_expand: auto_expand,
            show_hidden: show_hidden,
            hide_clean: hide_clean,
            exec_console: Rc::clone(&exec_console),
            popup_menu: popup_menu,
            hovered_fso_path: RefCell::new(None),
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
        owft.exec_console.event_notifier.add_notification_cb(
            events::EV_AUTO_UPDATE
                | events::EV_CHECKOUT
                | events::EV_FILES_CHANGE
                | events::EV_COMMIT
                | events::EV_PULL,
            Box::new(move |_| {
                owft_clone.update();
            }),
        );

        let owft_clone = Rc::clone(&owft);
        owft.exec_console.event_notifier.add_notification_cb(
            events::EV_CHANGE_DIR,
            Box::new(move |_| owft_clone.repopulate()),
        );

        let owft_clone = Rc::clone(&owft);
        owft.popup_menu
            .append_item(
                "add",
                &(
                    "Add",
                    None,
                    Some("Add to the selected/indicated file(s) to the index"),
                )
                    .into(),
                repos::SAV_IN_REPO + SAV_SELN_MADE_OR_HOVER_OK,
            )
            .connect_activate(move |_| {
                if let Some(fso_paths) = owft_clone.get_chosen_file_paths_string() {
                    let cmd = format!("git add {}", fso_paths);
                    let result = owft_clone
                        .exec_console
                        .exec_cmd(&cmd, events::EV_FILES_CHANGE);
                    owft_clone.report_any_command_problems(&cmd, &result);
                }
            });

        let owft_clone = owft.clone();
        owft.view.connect_button_press_event(move |view, event| {
            if event.get_button() == 3 {
                let fso_path = if let Some(fso_path) =
                    get_row_item_for_event!(view, event, String, fs_db::PATH)
                {
                    Some(shlex::quote(&fso_path).to_string())
                } else {
                    None
                };
                owft_clone.set_hovered_fso_path(fso_path);
                owft_clone.popup_menu.popup_at_event(event);
                return Inhibit(true);
            } else if event.get_button() == 2 {
                owft_clone.view.get_selection().unselect_all();
                return Inhibit(true);
            }
            Inhibit(false)
        });

        // Handle double click
        let owft_clone = owft.clone();
        owft.view
            .connect_row_activated(move |view, tree_path, _tree_view_column| {
                if let Some(fso_path) =
                    get_row_item_for_tree_path!(view, tree_path, String, fs_db::PATH)
                {
                    if fso_path.path_is_dir() {
                        if submodules::is_git_submodule(Some(&fso_path))
                            || !owft_clone.exec_console.in_repo()
                        {
                            owft_clone.exec_console.chdir(&fso_path);
                        }
                    } else if fso_path.path_is_file() {
                        // this will cause deleted files to be ignored
                        match edit::get_assigned_editor(&fso_path) {
                            Ok(editor) => {
                                if let Err(err) = Command::new(&editor).arg(&fso_path).spawn() {
                                    let msg = format!(
                                        "{} {}: failed",
                                        shlex::quote(&editor),
                                        shlex::quote(&fso_path)
                                    );
                                    owft_clone.report_error(&msg, &err);
                                }
                            }
                            Err(err) => {
                                let msg = "Error accessing editor assignment table";
                                owft_clone.report_error(&msg, &err);
                            }
                        }
                    }
                }
            });

        owft.repopulate();
        owft.view.show_all();
        scrolled_window.show_all();
        owft.v_box.show_all();
        owft
    }

    fn set_hovered_fso_path(&self, path: Option<String>) {
        let condns = self
            .view
            .get_selection()
            .get_masked_conditions_with_hover_ok(path.is_some());
        self.popup_menu.update_condns(condns);
        *self.hovered_fso_path.borrow_mut() = path;
    }

    fn get_chosen_file_paths_string(&self) -> Option<String> {
        let selection = self.view.get_selection();
        let (tree_paths, store) = selection.get_selected_rows();
        if tree_paths.len() > 0 {
            let mut count = 0;
            let mut fso_paths = String::new();
            for tree_path in tree_paths.iter() {
                if let Some(iter) = store.get_iter(&tree_path) {
                    if let Some(fso_path) =
                        store.get_value(&iter, fs_db::PATH).get::<String>().unwrap()
                    {
                        if count > 0 {
                            fso_paths.push_str(" ");
                        }
                        count += 1;
                        fso_paths.push_str(&shlex::quote(&fso_path));
                    }
                }
            }
            if count > 0 {
                Some(fso_paths)
            } else {
                None
            }
        } else {
            self.hovered_fso_path.borrow().clone()
        }
    }
}
