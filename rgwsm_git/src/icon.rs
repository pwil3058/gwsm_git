// Copyright 2017 Peter Williams <pwil3058@gmail.com>
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

use pw_gix::{gdk_pixbuf, gtk};

/* XPM */
static RGWSMGIT_XPM: &[&str] = &[
    "64 64 3 1",
    "0	c #FF0000",
    "1	c #000000",
    "2	c #FFFF00",
    "2222222222222222000000000000000022222222222222221111111111111111",
    "2222222222222222000000000000000022222222222222221111111111111111",
    "2222222222222222000000000000000022222222222222221111111111111111",
    "2222222222222222000000000000000022222222222222221111111111111111",
    "2222222222222222000000000000000022222222222222221111111111111111",
    "2222222222222222000000000000000022222222222222221111111111111111",
    "2222222222222222000000000000000022222222222222221111111111111111",
    "2222222222222222000000000000000022222222222222221111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "2222222222222222000000000000000000000000000000001111111111111111",
    "2222222222222222000000000000000000000000000000001111111111111111",
    "2222222222222222000000000000000000000000000000001111111111111111",
    "2222222222222222000000000000000000000000000000001111111111111111",
    "2222222222222222000000000000000000000000000000001111111111111111",
    "2222222222222222000000000000000000000000000000001111111111111111",
    "2222222222222222000000000000000000000000000000001111111111111111",
    "2222222222222222000000000000000000000000000000001111111111111111",
    "2222222222222222222222222222222222222222222222221111111111111111",
    "2222222222222222222222222222222222222222222222221111111111111111",
    "2222222222222222222222222222222222222222222222221111111111111111",
    "2222222222222222222222222222222222222222222222221111111111111111",
    "2222222222222222222222222222222222222222222222221111111111111111",
    "2222222222222222222222222222222222222222222222221111111111111111",
    "2222222222222222222222222222222222222222222222221111111111111111",
    "2222222222222222222222222222222222222222222222221111111111111111",
    "0000000000000000000000000000000011111111111111111111111111111111",
    "0000000000000000000000000000000011111111111111111111111111111111",
    "0000000000000000000000000000000011111111111111111111111111111111",
    "0000000000000000000000000000000011111111111111111111111111111111",
    "0000000000000000000000000000000011111111111111111111111111111111",
    "0000000000000000000000000000000011111111111111111111111111111111",
    "0000000000000000000000000000000011111111111111111111111111111111",
    "0000000000000000000000000000000011111111111111111111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "0000000000000000000000000000000022222222222222221111111111111111",
    "2222222222222222222222222222222211111111111111111111111111111111",
    "2222222222222222222222222222222211111111111111111111111111111111",
    "2222222222222222222222222222222211111111111111111111111111111111",
    "2222222222222222222222222222222211111111111111111111111111111111",
    "2222222222222222222222222222222211111111111111111111111111111111",
    "2222222222222222222222222222222211111111111111111111111111111111",
    "2222222222222222222222222222222211111111111111111111111111111111",
    "2222222222222222222222222222222211111111111111111111111111111111",
    "0000000000000000111111111111111111111111111111111111111111111111",
    "0000000000000000111111111111111111111111111111111111111111111111",
    "0000000000000000111111111111111111111111111111111111111111111111",
    "0000000000000000111111111111111111111111111111111111111111111111",
    "0000000000000000111111111111111111111111111111111111111111111111",
    "0000000000000000111111111111111111111111111111111111111111111111",
    "0000000000000000111111111111111111111111111111111111111111111111",
    "0000000000000000111111111111111111111111111111111111111111111111",
];

pub fn rgwsmgit_pixbuf() -> gdk_pixbuf::Pixbuf {
    gdk_pixbuf::Pixbuf::from_xpm_data(RGWSMGIT_XPM)
}

pub fn _rgwsmgit_image(size: i32) -> gtk::Image {
    if let Some(pixbuf) =
        rgwsmgit_pixbuf().scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear)
    {
        gtk::Image::from_pixbuf(Some(&pixbuf))
    } else {
        panic!("File: {:?} Line: {:?}", file!(), line!())
    }
}
