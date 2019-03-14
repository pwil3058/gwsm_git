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

// XPM
static SUPERPROJECT_XPM: &[&str] = &[
    "64 64 15 1",
    " 	c None",
    "1	c #8D8D8D",
    "2	c #00DB00",
    "3	c #E9E9E9",
    "4	c #2D2D2D",
    "5	c #7C7C7C",
    "6	c #C9C9C9",
    "7	c #616161",
    "8	c #111111",
    "9	c #FEFEFE",
    "A	c #3F3F3F",
    "B	c #D7D7D7",
    "C	c #B1B1B1",
    "D	c #F3F3F3",
    "E	c #9F9F9F",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "            6666666666666666666666666666666666666666            ",
    "           688888888888888888888888888888888888888886           ",
    "           682222222222222222222222222222222222222286           ",
    "           682222222222222222222222222222222222222286           ",
    "           682222222222222222222222222222222222222286           ",
    "           682222222222222222222222222222222222222286           ",
    "           682222222222222222222222222222222222222286           ",
    "           682222222222222222222222222222222222222286           ",
    "           B4444444444444444444884444444444444444444B           ",
    "            33333333333333333D6776D33333333333333333            ",
    "                              6446                              ",
    "                             B4558B                             ",
    "                             585558                             ",
    "                           D18555581D                           ",
    "                           3855555583                           ",
    "                          358555555883                          ",
    "                          185555555588                          ",
    "                         BA5555555555AB                         ",
    "                         75555555555554                         ",
    "                        6455555555555546                        ",
    "                       345555555555555543                       ",
    "                       C85555555555555588                       ",
    "                      C4555555555555555548                      ",
    "                      75555555555555555558                      ",
    "                     1855555555555555555588                     ",
    "                     7555555555555555555558                     ",
    "                    585555555555555555555588                    ",
    "                   68555555555555555555555586                   ",
    "                  B78555555555555555555555558B                  ",
    "                  E855555555555555555555555558                  ",
    "                 CA55555555555555555555555555A8                 ",
    "                 785555555555555555555555555558                 ",
    "                64555555555555555555555555555546                ",
    "               3A55555555555555555555555555555583               ",
    "              D4555555555555555555555555555555558D              ",
    "              C4555555555555555555555555555555558C              ",
    "              455555555555555555555555555555555558              ",
    "             55555555555555555555555555555555555588             ",
    "            DA555555555555555555555555555555555555AD            ",
    "           385555555555555555555555555555555555555583           ",
    "           BA84444444444855555555555555844444444448AB           ",
    "           D666BBBBBBBBC4555555555555554CBBBBBBBB666D           ",
    "                       B4555555555555554B                       ",
    "                       B4555555555555554B                       ",
    "                       B4555555555555554B                       ",
    "                       C8555555555555558C                       ",
    "                      DE8555555555555558ED                      ",
    "                       E8555555555555558E                       ",
    "                      DE8555555555555558ED                      ",
    "                      DE8555555555555558ED                      ",
    "                       E8555555555555558E                       ",
    "                       E8555555555555558E                       ",
    "                       E8555555555555558E                       ",
    "                       E8555555555555558E                       ",
    "                       E8555555555555558E                       ",
    "                       E8555555555555558E                       ",
    "                       CAAAAAAAAAAAAAAAAC                       ",
    "                        DDDDDDDDDDDDDDDD                        ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
];

pub fn superproject_pixbuf() -> gdk_pixbuf::Pixbuf {
    gdk_pixbuf::Pixbuf::new_from_xpm_data(SUPERPROJECT_XPM)
}

pub fn superproject_image(size: i32) -> gtk::Image {
    if let Some(pixbuf) =
        superproject_pixbuf().scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear)
    {
        gtk::Image::new_from_pixbuf(Some(&pixbuf))
    } else {
        panic!("File: {:?} Line: {:?}", file!(), line!())
    }
}
