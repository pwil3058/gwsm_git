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

use std::error::Error;
use std::fmt;
use std::io;
use std::process;
use std::rc::Rc;

use gtk;
use gtk::prelude::*;

use shlex;

use pw_gix::gtkx::dialog::RememberDialogSize;
use pw_gix::gtkx::entry::LabelledTextEntry;
use pw_gix::wrapper::*;

use crate::action_icons;
use crate::events;
use crate::exec::ExecConsole;
use crate::message;
use crate::repos;

#[derive(Debug)]
pub enum TagError {
    NoTagName,
    NoKeyId,
    NoAnnotationMessage,
}

impl fmt::Display for TagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TagError is here!")
    }
}

impl Error for TagError {
    fn description(&self) -> &str {
        match self {
            TagError::NoTagName => "Tag name is required",
            TagError::NoKeyId => "User key-id is required",
            TagError::NoAnnotationMessage => "Annotation message is required",
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub struct NewTagWidget {
    v_box: gtk::Box,
    tag_name_entry: Rc<LabelledTextEntry>,
    force_cbth: gtk::CheckButton,
    annotate_cbtn: gtk::CheckButton,
    sign_cbtn: gtk::CheckButton,
    key_cbtn: gtk::CheckButton,
    key_id_entry: Rc<LabelledTextEntry>,
    message: Rc<message::MessageWidget>,
    exec_console: Rc<ExecConsole>,
}

impl_widget_wrapper!(v_box: gtk::Box, NewTagWidget);

impl NewTagWidget {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<NewTagWidget> {
        let ntw = Rc::new(NewTagWidget {
            v_box: gtk::Box::new(gtk::Orientation::Vertical, 0),
            tag_name_entry: LabelledTextEntry::new("Tag: "),
            force_cbth: gtk::CheckButton::new_with_label("--force"),
            annotate_cbtn: gtk::CheckButton::new_with_label("--annotate"),
            sign_cbtn: gtk::CheckButton::new_with_label("--sign"),
            key_cbtn: gtk::CheckButton::new_with_label("--local-user"),
            key_id_entry: LabelledTextEntry::new("Key Id: "),
            message: message::MessageWidget::new("Message"),
            exec_console: Rc::clone(exec_console),
        });
        ntw.key_id_entry.pwo().set_sensitive(false);
        ntw.message.pwo().set_sensitive(false);

        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        h_box.pack_start(&ntw.tag_name_entry.pwo(), true, true, 0);
        h_box.pack_start(&ntw.force_cbth, false, false, 0);
        ntw.v_box.pack_start(&h_box, false, false, 0);
        ntw.tag_name_entry.entry().set_activates_default(true);

        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 20);
        h_box.pack_start(&ntw.annotate_cbtn, false, false, 0);
        h_box.pack_start(&ntw.sign_cbtn, false, false, 0);
        h_box.pack_start(&ntw.key_cbtn, false, false, 0);
        h_box.pack_start(&ntw.key_id_entry.pwo(), true, true, 0);
        ntw.v_box.pack_start(&h_box, false, false, 0);

        ntw.v_box.pack_start(&ntw.message.pwo(), true, true, 0);

        let ntw_clone = Rc::clone(&ntw);
        ntw.annotate_cbtn.connect_property_active_notify(move |cb| {
            if cb.get_active() {
                ntw_clone.sign_cbtn.set_active(false);
                ntw_clone.key_cbtn.set_active(false);
                ntw_clone.message.pwo().set_sensitive(true);
            } else if !ntw_clone.sign_cbtn.get_active() && !ntw_clone.key_cbtn.get_active() {
                ntw_clone.message.pwo().set_sensitive(false);
            }
        });

        let ntw_clone = Rc::clone(&ntw);
        ntw.sign_cbtn.connect_property_active_notify(move |cb| {
            if cb.get_active() {
                ntw_clone.annotate_cbtn.set_active(false);
                ntw_clone.key_cbtn.set_active(false);
                ntw_clone.message.pwo().set_sensitive(true);
            } else if !ntw_clone.annotate_cbtn.get_active() && !ntw_clone.key_cbtn.get_active() {
                ntw_clone.message.pwo().set_sensitive(false);
            }
        });

        let ntw_clone = Rc::clone(&ntw);
        ntw.key_cbtn.connect_property_active_notify(move |cb| {
            if cb.get_active() {
                ntw_clone.annotate_cbtn.set_active(false);
                ntw_clone.sign_cbtn.set_active(false);
                ntw_clone.key_id_entry.pwo().set_sensitive(true);
                ntw_clone.message.pwo().set_sensitive(true);
            } else {
                ntw_clone.key_id_entry.pwo().set_sensitive(false);
                if !ntw_clone.annotate_cbtn.get_active() && !ntw_clone.sign_cbtn.get_active() {
                    ntw_clone.message.pwo().set_sensitive(false);
                }
            }
        });

        ntw.v_box.show_all();

        ntw
    }

    pub fn apply(
        &self,
        target: Option<&str>,
    ) -> Result<(String, io::Result<process::Output>), TagError> {
        if let Some(tag_name) = self.tag_name_entry.entry().get_text() {
            if tag_name.len() > 0 {
                let mut cmd = "git tag ".to_string();
                if self.force_cbth.get_active() {
                    cmd.push_str("--force ")
                }
                let mut annotate = false;
                if self.annotate_cbtn.get_active() {
                    annotate = true;
                    cmd.push_str("--annotate ");
                } else if self.sign_cbtn.get_active() {
                    annotate = true;
                    cmd.push_str("--sign ");
                } else if self.key_cbtn.get_active() {
                    annotate = true;
                    if let Some(key_id) = self.key_id_entry.entry().get_text() {
                        if key_id.len() > 0 {
                            cmd.push_str("--local-user ");
                            cmd.push_str(&shlex::quote(&key_id));
                            cmd.push(' ');
                        } else {
                            return Err(TagError::NoKeyId);
                        }
                    } else {
                        return Err(TagError::NoKeyId);
                    }
                }
                if annotate {
                    if let Some(msg) = self.message.get_message() {
                        cmd.push_str("-m ");
                        cmd.push_str(&shlex::quote(&msg));
                        cmd.push(' ');
                    } else {
                        return Err(TagError::NoAnnotationMessage);
                    }
                }
                cmd.push_str(&shlex::quote(&tag_name));
                if let Some(target) = target {
                    cmd.push(' ');
                    cmd.push_str(&shlex::quote(target));
                };
                println!("Command: {}", cmd);
                let result = self.exec_console.exec_cmd(&cmd, events::EV_TAGS_CHANGE);
                return Ok((cmd, result));
            } else {
                return Err(TagError::NoTagName);
            }
        } else {
            return Err(TagError::NoTagName);
        }
    }
}

pub trait CreatTag: WidgetWrapper {
    fn exec_console(&self) -> &Rc<ExecConsole>;

    fn create_tag_for(&self, target: Option<&str>) {
        let dialog = self.new_dialog_with_buttons(
            Some("New Tag"),
            gtk::DialogFlags::DESTROY_WITH_PARENT | gtk::DialogFlags::MODAL,
            CANCEL_OK_BUTTONS,
        );
        dialog.set_default_response(gtk::ResponseType::Ok);
        let ntw = NewTagWidget::new(self.exec_console());
        dialog
            .get_content_area()
            .pack_start(&ntw.pwo(), true, true, 0);
        dialog.get_content_area().show_all();
        dialog.set_size_from_recollections("tag:dialog", (640, 320));
        loop {
            let result = dialog.run();
            if gtk::ResponseType::from(result) == gtk::ResponseType::Ok {
                match ntw.apply(target) {
                    Ok((cmd, result)) => {
                        self.report_any_command_problems(&cmd, &result);
                        if let Ok(ref output) = result {
                            if output.status.success() {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    Err(err) => self.report_error("Insufficient data for creating tag", &err),
                }
            } else {
                break;
            }
        }
        dialog.destroy();
    }
}

pub struct TagButton {
    button: gtk::Button,
    exec_console: Rc<ExecConsole>,
}

impl_widget_wrapper!(button: gtk::Button, TagButton);

impl CreatTag for TagButton {
    fn exec_console(&self) -> &Rc<ExecConsole> {
        &self.exec_console
    }
}

impl TagButton {
    pub fn new(exec_console: &Rc<ExecConsole>) -> Rc<Self> {
        let button = gtk::Button::new();
        button.set_tooltip_text(Some("Tag the current HEAD revision"));
        button.set_image(&action_icons::tag_image(32));
        button.set_image_position(gtk::PositionType::Top);
        button.set_label("tag");
        exec_console
            .managed_buttons
            .add_widget("tag", &button, repos::SAV_IN_REPO);
        let bb = Rc::new(Self {
            button: button,
            exec_console: Rc::clone(&exec_console),
        });

        let bb_clone = Rc::clone(&bb);
        bb.button
            .connect_clicked(move |_| bb_clone.create_tag_for(None));

        bb
    }
}
