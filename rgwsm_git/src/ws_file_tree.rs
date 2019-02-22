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

use std::rc::Rc;

use gtk;
use gtk::prelude::*;

use pw_gix::wrapper::*;

pub struct OsWsFsTree {
    v_box: gtk::Box,
}

impl_widget_wrapper!(v_box: gtk::Box, OsWsFsTree);

impl OsWsFsTree {
    pub fn new() -> Rc<Self> {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let label = gtk::Label::new("Work Space File Tree");
        vbox.pack_start(&label, true, true, 0);
        Rc::new(OsWsFsTree {
            v_box: vbox,
        })
    }
}
