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

// XPM
static STASH_SHOW_XPM: &[&str] = &[
    "64 64 16 1",
    "0	c #000000",
    "1	c #893B2D",
    "2	c #DBDB00",
    "3	c #FF1F16",
    "4	c #27452E",
    "5	c #547257",
    "6	c #2D7F40",
    "7	c #171717",
    "8	c #FF5C43",
    "9	c #77B37F",
    "A	c #438F53",
    "B	c #FF7B59",
    "C	c #4E3027",
    "D	c #FF3C2B",
    "E	c #5C9B67",
    "   c None",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "  777777777777777777   777777777777777777   777777777777777777  ",
    " 7711111111111111117  7711111111111111117  7711111111111111117  ",
    " 7CBBBBBBB888DDDD337  7CBBBBBBB888DDDD337  7CBBBBBBB888DDDD337  ",
    " 7CBBBBB8888D8DD3337  7CBBBBB8888D8DD3337  7CBBBBB8888D8DD3337  ",
    " 7CBBBBB8888DDD3D337  7CBBBBB8888DDD3D337  7CBBBBB8888DDD3D337  ",
    " 7CBBB8888DDDDD33337  7CBBB8888DDDDD33337  7CBBB8888DDDDD33337  ",
    " 7CBBB88888DDD333337  7CBBB88888DDD333337  7CBBB88888DDD333337  ",
    " 7777777777777777077  7777777777777777077  7777777777777777077  ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "       77777777             77777777             77777777       ",
    "       759E9E57             759E9E57             759E9E57       ",
    "       759999E7             759999E7             759999E7       ",
    "       759999E7             759999E7             759999E7       ",
    "       759999E7             759999E7             759999E7       ",
    "       759999E7             759999E7             759999E7       ",
    "  7444445999954444477  7444445999954444477  7444445999954444477 ",
    "  799999999E9EEEEAA47  799999999E9EEEEAA47  799999999E9EEEEAA47 ",
    "  799999999EEEEAAAA47  799999999EEEEAAAA47  799999999EEEEAAAA47 ",
    "  4999999EEEEEAAAA647  4999999EEEEEAAAA647  4999999EEEEEAAAA647 ",
    "  799999EEEEAAAAA6677  799999EEEEAAAAA6677  799999EEEEAAAAA6677 ",
    "  79999EEEEEAAA666647  79999EEEEEAAA666647  79999EEEEEAAA666647 ",
    "  7777776AAAA6777777   7777776AAAA6777777   7777776AAAA6777777  ",
    "       74EAAA67             74EAAA67             74EAAA67       ",
    "       76AA6667             76AA6667             76AA6667       ",
    "       74A66647             74A66647             74A66647       ",
    "       74A66667             74A66667             74A66667       ",
    "       74444447             74444447             74444447       ",
    "       77777777             77777777             77777777       ",
    "                                                                ",
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

pub fn stash_show_pixbuf() -> gdk_pixbuf::Pixbuf {
    gdk_pixbuf::Pixbuf::new_from_xpm_data(STASH_SHOW_XPM)
}

pub fn stash_show_image(size: i32) -> gtk::Image {
    if let Some(pixbuf) =
        stash_show_pixbuf().scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear)
    {
        gtk::Image::new_from_pixbuf(Some(&pixbuf))
    } else {
        panic!("File: {:?} Line: {:?}", file!(), line!())
    }
}

// XPM
static STASH_POP_XPM: &[&str] = &[
    "64 64 3 1",
    "0	c #000000",
    "1	c #DBDB00",
    " 	c None",
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
    "                               00                               ",
    "                              0000                              ",
    "                             000000                             ",
    "                            00000000                            ",
    "                           0000000000                           ",
    "                          000000000000                          ",
    "                         00000000000000                         ",
    "                        0000000000000000                        ",
    "                       000000000000000000                       ",
    "                      00000000000000000000                      ",
    "                     0000000000000000000000                     ",
    "                    000000000000000000000000                    ",
    "                   00000000000000000000000000                   ",
    "                  0000000000000000000000000000                  ",
    "                 000000000000000000000000000000                 ",
    "                00000000000000000000000000000000                ",
    "                           0000000000                           ",
    "                           0000000000                           ",
    "                           0000000000                           ",
    "                           0000000000                           ",
    "                           0000000000                           ",
    "                           0000000000                           ",
    "                           0000000000                           ",
    "                           0000000000                           ",
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
    "0000000000000000000000000000000000000000000000000000000000000000",
    "0111111111111111111111111111111111111111111111111111111111111110",
    "0111111111111111111111111111111111111111111111111111111111111110",
    "0111111111111111111111111111111111111111111111111111111111111110",
    "0111111111111111111111111111111111111111111111111111111111111110",
    "0111111111111111111111111111111111111111111111111111111111111110",
    "0111111111111111111111111111111111111111111111111111111111111110",
    "0111111111111111111111111111111111111111111111111111111111111110",
    "0000000000000000000000000000000000000000000000000000000000000000",
];

pub fn stash_pop_pixbuf() -> gdk_pixbuf::Pixbuf {
    gdk_pixbuf::Pixbuf::new_from_xpm_data(STASH_POP_XPM)
}

pub fn stash_pop_image(size: i32) -> gtk::Image {
    if let Some(pixbuf) =
        stash_pop_pixbuf().scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear)
    {
        gtk::Image::new_from_pixbuf(Some(&pixbuf))
    } else {
        panic!("File: {:?} Line: {:?}", file!(), line!())
    }
}

// XPM
static STASH_APPLY_XPM: &[&str] = &[
    "64 64 16 1",
    "0	c #000000",
    "1	c #7A7C11",
    "2	c #7A7D7B",
    "3	c #2D3334",
    "4	c #BDBFBA",
    "5	c #C7C700",
    "6	c #9EA09D",
    "7	c #D6D8D3",
    "8	c #50544A",
    "9	c #B1B101",
    "A	c #EAEBE9",
    "B	c #B0B3AE",
    "C	c #DADA00",
    "D	c #C9CBC6",
    "E	c #363C3D",
    "   c None",
    "                          38   3EE33                            ",
    "                        3EEE   34A4EE                           ",
    "                       384A83  E77AE                            ",
    "                       36A77EE32747E                            ",
    "                       3E74D7AAAAD4E3 333                       ",
    "                        E67A744447A783E6E3                      ",
    "                    E  38AA4BBBBBB47767ABE                      ",
    "                   3EEEE7AB6BBBB444477BDAE3                     ",
    "                   3BADBA4BBB477D444D7448E8                     ",
    "                   E77477BBBDB284ADDD77E3                       ",
    "                   EB74ADBBD4333E4A4DD7E                        ",
    "                   3332A444783  E2ADD7AE                        ",
    "                      EA44BA23  32A7D7723EE                     ",
    "                      EA44474E3337A7D774D63                     ",
    "                     3E7744DA7227A7777464DE                     ",
    "                   8E8B4ADDD7AAAA7777A6BD6E 3EE33               ",
    "                   EE7467A4DD777777774EEEE3 E4A4EE              ",
    "                    E67D27A7D7D777777E4A8E  E77AE               ",
    "                    3E2EEEDA7777777D22A7DEEE2D47E               ",
    "                     E33 EE6D77774B6BED4D7AAAA74E3 333          ",
    "                          E66B8EEE6BD227A744447A783E6EE         ",
    "                          E444E  EE46E6ABBBBBBB47767ABE         ",
    "                         EE242E EEEEEDABBBBBB4B447767AE3        ",
    "                          3EEEE 3B7B4ABBBB4D77444DA448E8        ",
    "                           8    EA7B7ABB4DB28DA4D47DE3          ",
    "                       EEEEE    EBDDA4BB44E3334ADD77E           ",
    "                      3E4AA83  333E2A444783  E2ADDDAE           ",
    "                      EEA4D23 EE2E3EA4B472E  32ADD772EE3        ",
    "                       ED66BEEE476EEAD447DE33E7A7777BD6E        ",
    "                   3E  EB47A7D666D2ED74DDAD227A7777D64DE        ",
    "                  3EEEE2A466647666E64AD4DDAAAAA77776B763        ",
    "                  E4ADBA666BBB4D6EB46DADDD7777D7774EEEE3        ",
    "                  EA46D46628274D4E67427AD7D7777777E3  3         ",
    "                  8AD676B23EE67DD3E2EEEDA7777777D23             ",
    "                 8EE2BA648E 38AD722EE 336D77A74B6BE             ",
    "000000000000000000000E7B46303B77D266E00366B83336BD23000000000000",
    "0CCCCCCCCCCCCCCCCCCCC84D4768BA77B226859EDB4E999E46E155C5CCCCCCC0",
    "0CCCCCCCCCCCCCCCCCCC98B74D7AA7772662195E24219991E1955555CCCCCCC0",
    "0CCCCCCCCCCCCCCCCCC98466D7D777728EEE959918E99999995555CCCCCCCCC0",
    "0CCCCCCCCCCCCCCCCC5127666477DB2E9999995595999999995555C5CCCCCCC0",
    "0CCCCCCCCCCCCCCC559986DBE832226E99999955555555555555CCCCCCCCCCC0",
    "0CCCCCCCCCCCCCCC55999E2E111266639199999555555555C5CCCCCCCCCCCCC0",
    "0CCCCCCCCCCCCCCC559999111113B62319999555C55C5C5CCCCCCCCCCCCCCCC0",
    "0000000000000000000000000003330000000000000000000000000000000000",
    "                                                                ",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "0CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC0",
    "0CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC0",
    "0CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC0",
    "0CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC0",
    "0CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC0",
    "0CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC0",
    "0CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC0",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "                                                                ",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "0CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC0",
    "0CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC0",
    "0CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC0",
    "0CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC0",
    "0CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC0",
    "0CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC0",
    "0CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC0",
    "0000000000000000000000000000000000000000000000000000000000000000",
];

pub fn stash_apply_pixbuf() -> gdk_pixbuf::Pixbuf {
    gdk_pixbuf::Pixbuf::new_from_xpm_data(STASH_APPLY_XPM)
}

pub fn stash_apply_image(size: i32) -> gtk::Image {
    if let Some(pixbuf) =
        stash_apply_pixbuf().scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear)
    {
        gtk::Image::new_from_pixbuf(Some(&pixbuf))
    } else {
        panic!("File: {:?} Line: {:?}", file!(), line!())
    }
}

// XPM
static STASH_BRANCH_XPM: &[&str] = &[
    "64 64 4 1",
    "0	c #000000",
    "1	c #DBDB00",
    "2	c #DBFFB6",
    "   c None",
    "                                      0                         ",
    "                                     000                        ",
    "                                     00200                      ",
    "                                     002200                     ",
    "          0000000000000000000000000000022200                    ",
    "          00000000000000000000000000000222200                   ",
    "          002222222222222222222222222222222200                  ",
    "          0022222222222222222222222222222222200                 ",
    "          00222222222222222222222222222222222200                ",
    "          002222222222222222222222222222222222200               ",
    "          00222222222222222222222222222222222200                ",
    "          0022222222222222222222222222222222200                 ",
    "          002222222222222222222222222222222200                  ",
    "          00222222200000000000000000000222200                   ",
    "          00222222200                0022200                    ",
    "          00222222200                002200                     ",
    "          00222222200                00200                      ",
    "          00222222200                0000                       ",
    "          00222222200                000                        ",
    "          00222222200                00                         ",
    "          00222222200                0                          ",
    "          00222222200                                           ",
    "          00222222200                                           ",
    "          00222222200                          0                ",
    "          00222222200                          00               ",
    "          00222222200                          000              ",
    "          00222222200                          0000             ",
    "          00222222200                          00200            ",
    "          00222222200                          002200           ",
    "          00222222200                          0022200          ",
    "          00222222200                          00222200         ",
    "          00222222200                          002222200        ",
    "          00222222200                          0022222200       ",
    "          00222222200                          00222222200      ",
    "00000000000022222220000000000000000000000000000002222222200     ",
    "0000000000002222222000000000000000000000000000000222222222000000",
    "0100222222222222222222222222222222222222222222222222222222200110",
    "0100222222222222222222222222222222222222222222222222222222220010",
    "0100222222222222222222222222222222222222222222222222222222222000",
    "0100222222222222222222222222222222222222222222222222222222222200",
    "0100222222222222222222222222222222222222222222222222222222222000",
    "0100222222222222222222222222222222222222222222222222222222220010",
    "0100222222222222222222222222222222222222222222222222222222200110",
    "0000222222222222222222222222222222222222222222222222222222000000",
    "00000000000000000000000000000000000000000000000002222222200     ",
    "0000000000000000000000000000000000000000000000000222222200000000",
    "0111111111111111111111111111111111111111111111100222222001111110",
    "0111111111111111111111111111111111111111111111100222220011111110",
    "0111111111111111111111111111111111111111111111100222200111111110",
    "0111111111111111111111111111111111111111111111100222001111111110",
    "0111111111111111111111111111111111111111111111100220011111111110",
    "0111111111111111111111111111111111111111111111100200111111111110",
    "0111111111111111111111111111111111111111111111100001111111111110",
    "0000000000000000000000000000000000000000000000000000000000000000",
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
];

pub fn stash_branch_pixbuf() -> gdk_pixbuf::Pixbuf {
    gdk_pixbuf::Pixbuf::new_from_xpm_data(STASH_BRANCH_XPM)
}

pub fn stash_branch_image(size: i32) -> gtk::Image {
    if let Some(pixbuf) =
        stash_branch_pixbuf().scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear)
    {
        gtk::Image::new_from_pixbuf(Some(&pixbuf))
    } else {
        panic!("File: {:?} Line: {:?}", file!(), line!())
    }
}

// XPM
static STASH_DROP_XPM: &[&str] = &[
    "64 64 16 1",
    "0	c #000000",
    "1	c #845202",
    "2	c #9B9C99",
    "3	c #A3A300",
    "4	c #555753",
    "5	c #F5F4F2",
    "6	c #676965",
    "7	c #BD1410",
    "8	c #DADA00",
    "9	c #E34443",
    "A	c #E01B1B",
    "B	c #E88C8B",
    "C	c #EB6C6A",
    "D	c #EEA4A2",
    "E	c #E0302F",
    "   c None",
    "                                                                ",
    "                         AEAAEAAEAEAA                           ",
    "                      AAEE9CCCCCC9C9EAAA                        ",
    "                    EEE9CCCCCDDDBBCCCC9EAA                      ",
    "                   AA9CCCCCC999999CCCC9CEAA                     ",
    "                 AAECC99EAAAAAAAAAAAAE99C9EAA                   ",
    "                AAEC9EAEAECB55555DD9AAAEE99EAA                  ",
    "               AA9CEEAAAB555555555555BEAAA99EAA                 ",
    "               AECAAAAC5555555555555555CAAAE9EA                 ",
    "              AACEAAAB555555555555555555DEAAE9AA                ",
    "             AACEAAED55555555555555555555DAEA99A7               ",
    "             A99AAAB5555555555555555555555DAAA9EA               ",
    "            AA9EAAC555555555555555555555555CEAE9AA              ",
    "            A99AAE55555262555555555526255555AAA9E7              ",
    "            A9EAAB55555666255555555266655555BAAA9A              ",
    "           AE9AAA5555552444255555524442555555AAA9A7             ",
    "           AE9AA955555552444255552644255555559EAEEA             ",
    "           A9EAAD5555555524462552444255555555DAAAE7             ",
    "           A9AAAD5555555552444224442555555555DAAEEA             ",
    "           A9AAA555555555552444444255555555555AAAE7             ",
    "           79AAA555555555555244442555555555555AAAEA             ",
    "           A9AAA555555555555244442555555555555AAA97             ",
    "           A9AAAD55555555552444444255555555555AAAA7             ",
    "           7EAAAD5555555552444224442555555555DAAAE7             ",
    "           AEAAAB5555555524442552444255555555DAAAEA             ",
    "           7EEAA955555552444255552444255555559AAAA7             ",
    "           7AEA7A5555552644255555524442555555A7AA77             ",
    "            7EAAAC55555644255555555244655555CAAAE7              ",
    "            7EAA7A555552625555555555262555557A7AA7              ",
    "            77EAA795555555555555555555555559AAAA77              ",
    "             7AAAAAB5555555555555555555555B7A7A77               ",
    "             77EA7A7B55555555555555555555DAAAAA77               ",
    "              77EAAA7B555555555555555555B7A7AE77                ",
    "               77EAEE7CD55555555555555DCAEAEE77                 ",
    "               A7A9EEEEEB55555555555DBEEEEEE77A                 ",
    "0000000000000000077EEEEEEE9BDD555DBCEE9EEEE770000000000000000000",
    "088888888888888838179999999999EEE99E999E9E7138888888888888888880",
    "0888888888888883833177E9E999999999999E9E771383838888888888888880",
    "088888888888883833331177999999999999EE77113338388888888888888880",
    "088888888888888888333111777EE9E99EE77711133383888888888888888880",
    "0888888888888883838333311117777777111113333838888888888888888880",
    "0888888888888888888888333333313131333338388888888888888888888880",
    "0888888888888888888888888838383838838888888888888888888888888880",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "                                                                ",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "0888888888888888888888888888888888888888888888888888888888888880",
    "0888888888888888888888888888888888888888888888888888888888888880",
    "0888888888888888888888888888888888888888888888888888888888888880",
    "0888888888888888888888888888888888888888888888888888888888888880",
    "0888888888888888888888888888888888888888888888888888888888888880",
    "0888888888888888888888888888888888888888888888888888888888888880",
    "0888888888888888888888888888888888888888888888888888888888888880",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "                                                                ",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "0888888888888888888888888888888888888888888888888888888888888880",
    "0888888888888888888888888888888888888888888888888888888888888880",
    "0888888888888888888888888888888888888888888888888888888888888880",
    "0888888888888888888888888888888888888888888888888888888888888880",
    "0888888888888888888888888888888888888888888888888888888888888880",
    "0888888888888888888888888888888888888888888888888888888888888880",
    "0888888888888888888888888888888888888888888888888888888888888880",
    "0000000000000000000000000000000000000000000000000000000000000000",
];

pub fn stash_drop_pixbuf() -> gdk_pixbuf::Pixbuf {
    gdk_pixbuf::Pixbuf::new_from_xpm_data(STASH_DROP_XPM)
}

pub fn stash_drop_image(size: i32) -> gtk::Image {
    if let Some(pixbuf) =
        stash_drop_pixbuf().scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear)
    {
        gtk::Image::new_from_pixbuf(Some(&pixbuf))
    } else {
        panic!("File: {:?} Line: {:?}", file!(), line!())
    }
}

// XPM
static UPDATE_XPM: &[&str] = &[
    "64 64 16 1",
    " 	c None",
    "1	c #717963",
    "2	c #F8E908",
    "3	c #F7EE60",
    "4	c #615B06",
    "5	c #9A9EA1",
    "6	c #EFECB3",
    "7	c #C0B93C",
    "8	c #37393E",
    "9	c #BABDB1",
    "A	c #EDE77F",
    "B	c #565858",
    "C	c #828483",
    "D	c #F4F4ED",
    "E	c #C7CAD4",
    "F	c #BBB877",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                        999999999999595        45555554         ",
    "                        999999999959955        5555C5CC         ",
    "                     999E9AFAFA9AFAAAEE555     59FAFA95DD       ",
    "                  99E9996A33333333333A99A955C C59A333A544       ",
    "                99999A333A33333333333A333F555B5F3333A3744       ",
    "                9993A33333333333333333333A3A55C5A33333744       ",
    "              59E9A3333333333333333333333333F955A33333744       ",
    "             99E93333333333333333333333333333A55333333744       ",
    "            19E6A33333333333333333333333333333A9A3333A744       ",
    "           99E9333333333333333333333333333333333A33333744       ",
    "          599633333333333333333333333333333333333A3333744       ",
    "          E99A3333333333333AAAAAAAAA333333333333A3333A744       ",
    "         59933333333333333AEE95555EEA33333333333333333744       ",
    "         E93333333333333E99555595555596333333333333333744       ",
    "       79EA33333333333399555CC   1CCC55F3333333333333A744       ",
    "       999A33A3AA3A33AF95C        CCCCC5F3333333333333744       ",
    "       95A33666666A33955         55C5C5CCA333333333333744       ",
    "       99A33AA3AA663955        1C5F3AA3AAA3A333333333A744       ",
    "      C9EA3333A3366255         CCFA3333333333333333333744       ",
    "      599A3333333D67F5         CCA33333333333333333333744       ",
    "      59333323233DE55          CCF3333333333333333333A744       ",
    "      5933222222DDE5C          CCA33333333333333333333744       ",
    "      553A322222DD9CC          C15A333333333A33A333333744       ",
    "      5933222222DE9CC          115C7777777777777FFFFFF744       ",
    "      55A2222222DE51B             444444444444441B1B1BB44       ",
    "      5532222222DE9B              44444444444444BBBBBBB44       ",
    "      5FA2222222D951                                            ",
    "      5532222222DE511                                           ",
    "      C5A2222222DE9CC                                           ",
    "      5532222222D32CC                                           ",
    "      C5233222223A91C                      8BBB18               ",
    "      CC26A2222226D5C                    8B1755955C8            ",
    "      155D62222226DEC1                   B1C26DDD958B           ",
    "       CCEA22222223D51B                 BB5D632226E11           ",
    "       CC3322222223DE51B               BB9ED322226D5C           ",
    "       CC77A22222223DE51B             B1CDDA222223691           ",
    "       11C96222222226DECC1B8B8   888BB1CDDA222222275B8          ",
    "        B1563222222226DDECBBBB8 8BBBBCEDD3222222227C1           ",
    "         1C5222222222333DDE95C1C1C59ED6332222222227BB           ",
    "         B11722222222222666DE99599ED6632222222222278B           ",
    "          BCCF2222222222223DDDDDDDDD22222222222221BBB           ",
    "           115F322222222222222222222222222222222C1B             ",
    "            B15F2222222222222222222222222222222F1B8             ",
    "             B1C7222222222222222222222222222227CB8              ",
    "              BB173A22222222222222222222223A21B8B               ",
    "               BB1C923322222222222222222A3391BB8                ",
    "                88BC79326A332222222336A26F11B8                  ",
    "                   8BCF7ED62233332336E975B88                    ",
    "                     88B15F2236DA22295BBB8                      ",
    "                       88BBBBBBBBBBBB88                         ",
    "                          888888888888                          ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
    "                                                                ",
];

pub fn update_pixbuf() -> gdk_pixbuf::Pixbuf {
    gdk_pixbuf::Pixbuf::new_from_xpm_data(UPDATE_XPM)
}

pub fn update_image(size: i32) -> gtk::Image {
    if let Some(pixbuf) = update_pixbuf().scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear)
    {
        gtk::Image::new_from_pixbuf(Some(&pixbuf))
    } else {
        panic!("File: {:?} Line: {:?}", file!(), line!())
    }
}
