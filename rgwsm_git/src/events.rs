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

use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub const EV_CHANGE_DIR: u64 = 2 ^ 0;

pub struct EventNotifier {
    callbacks: RefCell<Vec<(u64, u64, Box<Fn(u64)>)>>,
    next_token: Cell<u64>,
}

impl EventNotifier {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            callbacks: RefCell::new(Vec::new()),
            next_token: Cell::new(0),
        })
    }
    pub fn add_notification_cb(&self, events: u64, callback: Box<Fn(u64)>) -> u64 {
        let token = self.next_token.get();
        self.next_token.set(token + 1);

        self.callbacks.borrow_mut().push((token, events, callback));

        token
    }

    pub fn del_notification_cb(&self, token: u64) {
        let position = self.callbacks.borrow().iter().position(|x| x.0 == token);
        if let Some(position) = position {
            self.callbacks.borrow_mut().remove(position);
        }
    }

    pub fn notify_events(&self, events: u64) {
        for (_, registered_events, callback) in self.callbacks.borrow().iter() {
            if registered_events & events != 0 {
                callback(events)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {}
}
