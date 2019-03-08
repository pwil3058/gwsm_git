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

use gdk_pixbuf;
use gtk;

// XPM
static BRANCH_XPM: &[&str] = &[
    "64 64 3 1",
    "A	c #000000",
    "B	c #DBFFB6",
    " 	c None",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                    A                           ",
    "                                    AA                          ",
    "                                    AAA                         ",
    "                                    AAAA                        ",
    "                                    AABAA                       ",
    "                                    AABBAA                      ",
    "         AAAAAAAAAAAAAAAAAAAAAAAAAAAAABBBAA                     ",
    "         AAAAAAAAAAAAAAAAAAAAAAAAAAAAABBBBAA                    ",
    "         AABBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBAA                   ",
    "         AABBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBAA                  ",
    "         AABBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBAA                 ",
    "         AABBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBAA                ",
    "         AABBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBAA                 ",
    "         AABBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBAA                  ",
    "         AABBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBAA                   ",
    "         AABBBBBBBAAAAAAAAAAAAAAAAAAAABBBBAA                    ",
    "         AABBBBBBBAAAAAAAAAAAAAAAAAAAABBBAA                     ",
    "         AABBBBBBBAA                AABBAA                      ",
    "         AABBBBBBBAA                AABAA                       ",
    "         AABBBBBBBAA                AAAA                        ",
    "         AABBBBBBBAA                AAA                         ",
    "         AABBBBBBBAA                AA                          ",
    "         AABBBBBBBAA                A                           ",
    "         AABBBBBBBAA                                            ",
    "         AABBBBBBBAA                                            ",
    "         AABBBBBBBAA                          A                 ",
    "         AABBBBBBBAA                          AA                ",
    "         AABBBBBBBAA                          AAA               ",
    "         AABBBBBBBAA                          AAAA              ",
    "         AABBBBBBBAA                          AABAA             ",
    "         AABBBBBBBAA                          AABBAA            ",
    "         AABBBBBBBAA                          AABBBAA           ",
    "         AABBBBBBBAA                          AABBBBAA          ",
    "         AABBBBBBBAA                          AABBBBBAA         ",
    "         AABBBBBBBAA                          AABBBBBBAA        ",
    "         AABBBBBBBAA                          AABBBBBBBAA       ",
    " AAAAAAAAAABBBBBBBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABBBBBBBBAA      ",
    " AAAAAAAAAABBBBBBBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABBBBBBBBBAA     ",
    " AABBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBAA    ",
    " AABBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBAA   ",
    " AABBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBAA  ",
    " AABBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBAA ",
    " AABBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBAA  ",
    " AABBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBAA   ",
    " AABBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBAA    ",
    " AABBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBAA     ",
    " AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABBBBBBBBAA      ",
    " AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABBBBBBBAA       ",
    "                                              AABBBBBBAA        ",
    "                                              AABBBBBAA         ",
    "                                              AABBBBAA          ",
    "                                              AABBBAA           ",
    "                                              AABBAA            ",
    "                                              AABAA             ",
    "                                              AAAA              ",
    "                                              AAA               ",
    "                                              AA                ",
    "                                              A                 ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
];

pub fn branch_pixbuf() -> gdk_pixbuf::Pixbuf {
    gdk_pixbuf::Pixbuf::new_from_xpm_data(BRANCH_XPM)
}

pub fn branch_image(size: i32) -> gtk::Image {
    if let Some(pixbuf) = branch_pixbuf().scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear)
    {
        gtk::Image::new_from_pixbuf(Some(&pixbuf))
    } else {
        panic!("File: {:?} Line: {:?}", file!(), line!())
    }
}

// XPM
static DIFF_XPM: &[&str] = &[
    "64 64 4 1",
    "R c #FF0000",
    "G c #00AA00",
    "B c #000000",
    "  c None",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "BBBBBBBBBBBBBBBBBBBB  BBBBBBBBBBBBBBBBBBBB  BBBBBBBBBBBBBBBBBBBB",
    "BRRRRRRRRRRRRRRRRRRB  BRRRRRRRRRRRRRRRRRRB  BRRRRRRRRRRRRRRRRRRB",
    "BRRRRRRRRRRRRRRRRRRB  BRRRRRRRRRRRRRRRRRRB  BRRRRRRRRRRRRRRRRRRB",
    "BRRRRRRRRRRRRRRRRRRB  BRRRRRRRRRRRRRRRRRRB  BRRRRRRRRRRRRRRRRRRB",
    "BRRRRRRRRRRRRRRRRRRB  BRRRRRRRRRRRRRRRRRRB  BRRRRRRRRRRRRRRRRRRB",
    "BRRRRRRRRRRRRRRRRRRB  BRRRRRRRRRRRRRRRRRRB  BRRRRRRRRRRRRRRRRRRB",
    "BRRRRRRRRRRRRRRRRRRB  BRRRRRRRRRRRRRRRRRRB  BRRRRRRRRRRRRRRRRRRB",
    "BBBBBBBBBBBBBBBBBBBB  BBBBBBBBBBBBBBBBBBBB  BBBBBBBBBBBBBBBBBBBB",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "      BBBBBBBB              BBBBBBBB              BBBBBBBB      ",
    "      BGGGGGGB              BGGGGGGB              BGGGGGGB      ",
    "      BGGGGGGB              BGGGGGGB              BGGGGGGB      ",
    "      BGGGGGGB              BGGGGGGB              BGGGGGGB      ",
    "      BGGGGGGB              BGGGGGGB              BGGGGGGB      ",
    "      BGGGGGGB              BGGGGGGB              BGGGGGGB      ",
    "BBBBBBBGGGGGGBBBBBBB  BBBBBBBGGGGGGBBBBBBB  BBBBBBBGGGGGGBBBBBBB",
    "BGGGGGGGGGGGGGGGGGGB  BGGGGGGGGGGGGGGGGGGB  BGGGGGGGGGGGGGGGGGGB",
    "BGGGGGGGGGGGGGGGGGGB  BGGGGGGGGGGGGGGGGGGB  BGGGGGGGGGGGGGGGGGGB",
    "BGGGGGGGGGGGGGGGGGGB  BGGGGGGGGGGGGGGGGGGB  BGGGGGGGGGGGGGGGGGGB",
    "BGGGGGGGGGGGGGGGGGGB  BGGGGGGGGGGGGGGGGGGB  BGGGGGGGGGGGGGGGGGGB",
    "BGGGGGGGGGGGGGGGGGGB  BGGGGGGGGGGGGGGGGGGB  BGGGGGGGGGGGGGGGGGGB",
    "BBBBBBBGGGGGGBBBBBBB  BBBBBBBGGGGGGBBBBBBB  BBBBBBBGGGGGGBBBBBBB",
    "      BGGGGGGB              BGGGGGGB              BGGGGGGB      ",
    "      BGGGGGGB              BGGGGGGB              BGGGGGGB      ",
    "      BGGGGGGB              BGGGGGGB              BGGGGGGB      ",
    "      BGGGGGGB              BGGGGGGB              BGGGGGGB      ",
    "      BGGGGGGB              BGGGGGGB              BGGGGGGB      ",
    "      BBBBBBBB              BBBBBBBB              BBBBBBBB      ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
];

pub fn diff_pixbuf() -> gdk_pixbuf::Pixbuf {
    gdk_pixbuf::Pixbuf::new_from_xpm_data(DIFF_XPM)
}

pub fn diff_image(size: i32) -> gtk::Image {
    if let Some(pixbuf) = diff_pixbuf().scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear) {
        gtk::Image::new_from_pixbuf(Some(&pixbuf))
    } else {
        panic!("File: {:?} Line: {:?}", file!(), line!())
    }
}
