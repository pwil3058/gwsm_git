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

// XPM
static COMMIT_XPM: &[&str] = &[
    "64 64 16 1",
    " 	c None",
    "1	c #5C5620",
    "2	c #75B37E",
    "3	c #D3692B",
    "4	c #742B11",
    "5	c #EDC84D",
    "6	c #B34C1F",
    "7	c #92B6DB",
    "8	c #1B1815",
    "9	c #FE7745",
    "A	c #A49738",
    "B	c #C12C13",
    "C	c #FDF45C",
    "D	c #3E8E4F",
    "E	c #FC9A4E",
    "F  c #000000",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                             FFFFFFFFFFF                        ",
    "                            F84699999364F                       ",
    "                          F8699999999999968F                    ",
    "                         F699999999999999996F                   ",
    "                       F8399999999999999999998F                 ",
    "                      F899999999999999999999991F                ",
    "                     F8999EE5C559999999999999958F               ",
    "                     F3995CCCCCC59999999999999E58F              ",
    "                    F69ECCCCC561633399999999999CAF              ",
    "                   F89ECCCCA8FFFFF133399999999955F              ",
    "                   F9ECCC58F      F133399999999EC1F             ",
    "                  F4ECCCAF         F63399999999EC5F             ",
    "                  F3C55AF          F83339999999ECC8F            ",
    "                 F45C5AF            F36339999999CC1F            ",
    "                 FA55AF             F63699999999CCAF            ",
    "                F85558F             F46339999999CCAF            ",
    "                F8551F              F43639999999CCAF            ",
    "                FA51F               F4666999999ECC5F            ",
    "                FAA8F               F8636999999ECC5F            ",
    "                FE1F                F8663999999ECCAF            ",
    "8888888888  888FFAFF88  8888 FFFFFFFF83B6999999ECCAFFFFFFFFFF   ",
    "8E99993BB8  8E9 F8FBB8  8E98F43664646B6B699999E999999ECCC51F    ",
    "899999BBB8  8998 F8BB8  8996 F6B66B66333399999999999ECCCC58F    ",
    "8888888888  888 8F8888  88888F8B66639999999999999999CCCC5F      ",
    "                              F8B6BB399999999999999CCCCC8F      ",
    "                               F4B66B99999999999995CCCC8F       ",
    "   8888        8888        8888 F4BBB399999999999ECCCC1F        ",
    "   8228        8228        8228 F84B6B3999999999ECCCC1F         ",
    "   8228        8228        8228  F8BBBB99999999E5CCCAF          ",
    "8888228888  8888228888  8888228888F8BBB699999995CCCAF           ",
    "8222222DD8  8222222DD8  8222222DD8 F4BBB39999955555F            ",
    "82222DDDD8  82222DDDD8  82222DDDD8  F4BBB3999E55C5F             ",
    "8888DD8888  8888DD8888  8888DD8888   F4BBB99955558F             ",
    "   8DD8        8DD8        8DD8       FBBB6955558F              ",
    "   8DD8        8DD8        8DD8        FBBBEE558F               ",
    "   8888        8888        8888        F4BBA551F                ",
    "                                        F4B5E1F                 ",
    "                        FFFFFFFFFFFFFFFFFFF3FFFFFFFFFFFFFFFFFFFF",
    "                        F7777777777777777FFFFF77777777777777777F",
    "                        F77777777777777777FFF777777777777777777F",
    "                        F777777777777777777F7777777777777777777F",
    "                        F777777777777777777F7777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        F77777777777777777777777777777777777777F",
    "                        FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
];

pub fn commit_pixbuf() -> gdk_pixbuf::Pixbuf {
    gdk_pixbuf::Pixbuf::new_from_xpm_data(COMMIT_XPM)
}

pub fn commit_image(size: i32) -> gtk::Image {
    if let Some(pixbuf) = commit_pixbuf().scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear)
    {
        gtk::Image::new_from_pixbuf(Some(&pixbuf))
    } else {
        panic!("File: {:?} Line: {:?}", file!(), line!())
    }
}

// XPM
static STASH_PUSH_XPM: &[&str] = &[
    "64 64 4 1",
    "0	c #000000",
    "1	c #B6DBB6",
    "2	c #DBDB00",
    " 	c None",
    "                       0000000000000                            ",
    "                       0000000000000                            ",
    "                       0000000000000                            ",
    "                       0000000000000                            ",
    "                       0000000000000                            ",
    "                       0000000000000                            ",
    "                       0000000000000                            ",
    "                       0000000000000                            ",
    "                       0000000000000                            ",
    "                       0000000000000                            ",
    "                       0000000000000                            ",
    "                       0000000000000                            ",
    "               000000000000000000000000000000                   ",
    "                 00000000000000000000000000                     ",
    "                   0000000000000000000000                       ",
    "                     000000000000000000                         ",
    "                       00000000000000                           ",
    "                         0000000000                             ",
    "                           000000                               ",
    "                             00                                 ",
    "                                                                ",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "0111111111111111111111111111111111111111111111111111111111111110",
    "0111111111111111111111111111111111111111111111111111111111111110",
    "0111111111111111111111111111111111111111111111111111111111111110",
    "0111111111111111111111111111111111111111111111111111111111111110",
    "0111111111111111111111111111111111111111111111111111111111111110",
    "0111111111111111111111111111111111111111111111111111111111111110",
    "0111111111111111111111111111111111111111111111111111111111111110",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "                                                                ",
    "                               000                              ",
    "                               000                              ",
    "                               000                              ",
    "                                                                ",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "                                                                ",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "                                                                ",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0222222222222222222222222222222222222222222222222222222222222220",
    "0000000000000000000000000000000000000000000000000000000000000000",
];

pub fn stash_push_pixbuf() -> gdk_pixbuf::Pixbuf {
    gdk_pixbuf::Pixbuf::new_from_xpm_data(STASH_PUSH_XPM)
}

pub fn stash_push_image(size: i32) -> gtk::Image {
    if let Some(pixbuf) =
        stash_push_pixbuf().scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear)
    {
        gtk::Image::new_from_pixbuf(Some(&pixbuf))
    } else {
        panic!("File: {:?} Line: {:?}", file!(), line!())
    }
}
