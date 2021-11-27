//! Simple widget-based interface definition as const

use rtwins::colors::{ColorBG, ColorFG};
use rtwins::{prop, Coord, Size, Widget, Link, ButtonStyle};

#[allow(dead_code)]
pub enum Id {
    WndMain = rtwins::WIDGET_ID_NONE as isize + 1,
    Lbl1,
    Lbl2,
    PnlGreen,
    BtnOk,
    BtnCancel,
    PnlYellow,
}

/// Easy conversion from enum to Wid
impl Id {
    pub const fn into(self) -> rtwins::WId {
        self as rtwins::WId
    }
}

#[rustfmt::skip]
pub const WINDOW: Widget = Widget {
    id: Id::WndMain.into(),
    link: Link::cdeflt(),
    coord: Coord { col: 1, row: 2 },
    size: Size {
        width: 25,
        height: 12,
    },
    typ: prop::Window {
        title: "** DEMO **",
        fg_color: ColorFG::White,
        bg_color: ColorBG::Blue,
        is_popup: false,
    }.into(),
    childs: &[
        Widget {
            id: Id::PnlGreen.into(),
            link: Link::cdeflt(),
            coord: Coord::cdeflt(),
            size: Size::cdeflt(),
            typ: prop::Panel {
                title: "Panel green",
                fg_color: ColorFG::White,
                bg_color: ColorBG::Green,
                no_frame: false,
            }.into(),
            childs: &[
                Widget {
                    id: Id::Lbl1.into(),
                    link: Link::cdeflt(),
                    coord: Coord::cdeflt(),
                    size: Size::cdeflt(),
                    typ: prop::Label {
                        title: "Label-1",
                        fg_color: ColorFG::White,
                        bg_color: ColorBG::Blue,
                    }.into(),
                    childs: &[],
                },
            ],
        },
        Widget {
            id: Id::Lbl2.into(),
            link: Link::cdeflt(),
            coord: Coord::cdeflt(),
            size: Size::cdeflt(),
            typ: prop::Label {
                title: "Label-2",
                fg_color: ColorFG::Cyan,
                bg_color: ColorBG::Black,
            }.into(),
            childs: &[],
        },
        Widget {
            id: Id::PnlYellow.into(),
            link: Link::cdeflt(),
            coord: Coord::cdeflt(),
            size: Size::cdeflt(),
            typ: prop::Panel {
                title: "Panel yellow",
                fg_color: ColorFG::Yellow,
                bg_color: ColorBG::Green,
                no_frame: false,
            }.into(),
            childs: &[
                Widget {
                    id: Id::BtnCancel.into(),
                    link: Link::cdeflt(),
                    coord: Coord::cdeflt(),
                    size: Size::cdeflt(),
                    typ: prop::Button {
                        text: "Cancel",
                        fg_color: ColorFG::White,
                        bg_color: ColorBG::Blue,
                        style: ButtonStyle::Solid
                    }.into(),
                    childs: &[],
                },
            ],
        },
    ],
};

/// Example of const-evaluated and translated Widgets tree into Widgets array
pub const DEMO_WND: [rtwins::Widget; rtwins::wgt_count(&WINDOW)] = rtwins::wgt_transform_array(&WINDOW);
