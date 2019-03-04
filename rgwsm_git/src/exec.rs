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

use std::process::Command;
use std::rc::Rc;
use std::string::FromUtf8Error;
use std::time::SystemTime;

use gtk;
use gtk::prelude::*;

use chrono::prelude::*;
use shlex;

use pw_gix::wrapper::*;

use bab::enotify::EventNotifier;

pub struct ExecConsole {
    text_view: gtk::TextView,
    pub event_notifier: Rc<EventNotifier>,
}

impl_widget_wrapper!(text_view: gtk::TextView, ExecConsole);

impl ExecConsole {
    pub fn new() -> Rc<Self> {
        let ec = Rc::new(Self {
            text_view: gtk::TextView::new(),
            event_notifier: Rc::new(EventNotifier::default()),
        });
        ec.append_bold("% ");
        ec
    }

    fn append_markup(&self, markup: &str) {
        let bfr = self.text_view.get_buffer().expect("failed to find text buffer");
        let mut model_iter = bfr.get_end_iter();
        bfr.insert_markup(&mut model_iter, markup);
        if let Some(eobuf) = bfr.create_mark("eobuf", &bfr.get_end_iter(), false) {
            self.text_view.scroll_to_mark(&eobuf, 0.001, false, 0.0, 0.0);
        };
    }

    fn append_bold(&self, text: &str) {
        let markup = format!(r###"<span foreground="black" weight="bold" font_family="monospace">{}</span>"###, text);
        self.append_markup(&markup);
    }

    fn append_cmd(&self, text: &str) {
        let markup = format!(r###"<span foreground="black" font_family="monospace">{}</span>"###, text);
        self.append_markup(&markup);
        self.append_markup("\n");
    }

    fn append_stdout(&self, text: &str) {
        let markup = format!(r###"<span foreground="black" font_family="monospace">{}</span>"###, text);
        self.append_markup(&markup);
    }

    fn append_stderr(&self, text: &str) {
        let markup = format!(r###"<span foreground="AA0000" font_family="monospace">{}</span>"###, text);
        self.append_markup(&markup);
    }

    fn append_stdin(&self, text: &str) {
        let markup = format!(r###"<span foreground="00AA00" font_family="monospace">{}</span>"###, text);
        self.append_markup(&markup);
    }

    pub fn exec_cmd(&self, cmd: &str, events: u64) {
        let dt = DateTime::<Local>::from(SystemTime::now());
        self.append_bold(&format!("{}: ", dt.format("%Y-%m-%d-%H-%M-%S")));
        self.append_cmd(cmd);
        let cmd_line = shlex::split(cmd).unwrap();
        match Command::new(&cmd_line[0]).args(&cmd_line[1..]).output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                self.append_stdout(&stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                self.append_stderr(&stderr);
                self.append_bold("% ");
                if output.status.success() {
                    if events != 0 {
                        self.event_notifier.notify_events(events)
                    }
                } else {
                    let msg = format!("\"{}\": failed.", cmd);
                    self.warn_user(&msg, Some(&stderr));
                }
            }
            Err(err) => {
                let msg = format!("\"{}\": blew up!", cmd);
                self.report_error(&msg, &err);
            }
        }
        //let error_code = if output.status.success() { 0 } else { 1 };
        //let std_err = String::from_utf8_lossy(output.stderr);
        //let std_out = String::from_utf8(output.stdout);
    }
}


#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        assert!(false);
    }
}
