//! Simple widget-based interface definition as const

use rtwins::colors::{ColorBG, ColorFG};
use rtwins::widget::{prop, Coord, Size, Widget, Link, ButtonStyle, WId, WIDGET_ID_NONE, PgBarStyle};

#[allow(dead_code)]
#[rustfmt::skip]
#[repr(u16)]
pub enum Id {
    WndMain = WIDGET_ID_NONE + 1,
        BtnToaster,
        PgControl,
            PageVer,
                PanelVersions,
                    LabelFwVersion,
                    LabelDate,
                    LabelAbout,
                PanelState,
                    LedPump,
                    LedLock,
                    LedBattery,
                PanelKey,
                    LabelKeyseq,
                    LabelKeyname,
                ChbxEnbl,
                ChbxLock,
                BtnYes,
                BtnNo,
                BtnPopup,
                Prgbar1,
                Prgbar2,
                Prgbar3,
            PageServ,
                Layer1,
                    LabelMultiFmt,
                    ListBox,
                Layer2,
                    Radio1,
                    Radio2,
                    Radio3,
                ChbxL1,
                ChbxL2,
            PageDiag,
                PanelEdt,
                    Edt1,
                    Edt2,
                CustomWgt1,
                PanelChbx,
                    LblChbxTitle,
                    ChbxA,
                    ChbxB,
                    ChbxC,
                    ChbxD,
            PageInactiv,
                PanelEmpty1,
                    LblWordwrap,
                    BtnNoaction,
                PanelEmpty2,
                    LblEmpty2,
            PageTextbox,
                TbxLoremipsum,
                Tbx1line,
            PageCombobox,
                CbxOptions,
                LbxUnderoptions,
                BtnSayYes,
                BtnSayNo,
                Btn1p5,
        LabelFtr,
}

/// Easy conversion from enum to Wid
impl Id {
    pub const fn into(self) -> WId {
        self as WId
    }
}

// ---------------------------------------------------------------------------------------------- //

#[rustfmt::skip]
const PNL_STATE_CHILDS: &[Widget] = &[
    Widget {
        id: Id::LedBattery.into(),
        coord: Coord { col: 2, row: 1 },
        typ: prop::Led {
            text: "(BATT)",
            fg_color: ColorFG::Black,
            bg_color_off: ColorBG::White,
            bg_color_on: ColorBG::Magenta,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::LedLock.into(),
        coord: Coord { col: 9, row: 1 },
        typ: prop::Led {
            text: "(LOCK)",
            fg_color: ColorFG::Black,
            bg_color_off: ColorBG::White,
            bg_color_on: ColorBG::Green,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::LedPump.into(),
        coord: Coord { col: 16, row: 1 },
        typ: prop::Led {
            text: "(PUMP)",
            fg_color: ColorFG::Red,
            bg_color_off: ColorBG::White,
            bg_color_on: ColorBG::Yellow,
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_VER_CHILDS: &[Widget] = &[
    Widget {
        id: Id::PanelVersions.into(),
        link: Link::cdeflt(),
        coord: Coord { col: 1, row: 1 },
        size: Size { width: 21, height: 5 },
        typ: prop::Panel {
            title: "VER 🍁",
            fg_color: ColorFG::White,
            bg_color: ColorBG::Green,
            no_frame: false,
        }.into(),
        childs: &[
            Widget {
                id: Id::LabelFwVersion.into(),
                coord: Coord { col: 2, row: 1 },
                typ: prop::Label {
                    title: "FwVer: 1.1",
                    fg_color: ColorFG::YellowIntense,
                    bg_color: ColorBG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::LabelDate.into(),
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 15, height: 1 },
                typ: prop::Label {
                    title: "Date•",
                    fg_color: ColorFG::Black,
                    bg_color: ColorBG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::LabelAbout.into(),
                coord: Coord { col: 2, row: 3 },
                size: Size { width: 0, height: 1 },
                typ: prop::Label {
                    title: "",
                    fg_color: ColorFG::Blue,
                    bg_color: ColorBG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
        ]
    },
    Widget {
        id: Id::PanelState.into(),
        link: Link::cdeflt(),
        coord: Coord { col: 30, row: 1 },
        size: Size { width: 25, height: 3 },
        typ: prop::Panel {
            title: "STATE: Leds",
            fg_color: ColorFG::Blue,
            bg_color: ColorBG::White,
            no_frame: false,
        }.into(),
        childs: &PNL_STATE_CHILDS
    },
    Widget {
        id: Id::PanelKey.into(),
        link: Link::cdeflt(),
        coord: Coord { col: 1, row: 7 },
        size: Size { width: 26, height: 4 },
        typ: prop::Panel {
            title: "KEY-CODES",
            fg_color: ColorFG::White,
            bg_color: ColorBG::Magenta,
            no_frame: false,
        }.into(),
        childs: &[
            Widget {
                id: Id::LabelKeyseq.into(),
                coord: Coord { col: 2, row: 1 },
                size: Size { width: 22, height: 1 },
                typ: prop::Label {
                    title: "",
                    fg_color: ColorFG::White,
                    bg_color: ColorBG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::LabelKeyname.into(),
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 17, height: 1 },
                typ: prop::Label {
                    title: "",
                    fg_color: ColorFG::White,
                    bg_color: ColorBG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
        ]
    },
    Widget {
        id: Id::ChbxEnbl.into(),
        coord: Coord { col: 30, row: 5 },
        typ: prop::CheckBox {
            text: "Enable",
            fg_color: ColorFG::White,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::ChbxLock.into(),
        coord: Coord { col: 45, row: 5 },
        typ: prop::CheckBox {
            text: "Lock",
            fg_color: ColorFG::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::BtnYes.into(),
        coord: Coord { col: 30, row: 7 },
        typ: prop::Button {
            text: "YES",
            fg_color: ColorFG::White,
            bg_color: ColorBG::Green,
            style: ButtonStyle::Solid
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::BtnNo.into(),
        coord: Coord { col: 38, row: 7 },
        typ: prop::Button {
            text: "NO",
            fg_color: ColorFG::White,
            bg_color: ColorBG::Red,
            style: ButtonStyle::Solid
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::BtnPopup.into(),
        coord: Coord { col: 45, row: 7 },
        typ: prop::Button {
            text: "POPUP",
            fg_color: ColorFG::White,
            bg_color: ColorBG::Inherit,
            style: ButtonStyle::Simple
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::Prgbar1.into(),
        coord: Coord { col: 30, row: 9 },
        size: Size { width: 25, height: 1 },
        typ: prop::ProgressBar {
            fg_color: ColorFG::Yellow,
            style: PgBarStyle::Hash
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::Prgbar2.into(),
        coord: Coord { col: 30, row: 10 },
        size: Size { width: 12, height: 1 },
        typ: prop::ProgressBar {
            fg_color: ColorFG::White,
            style: PgBarStyle::Shade
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::Prgbar3.into(),
        coord: Coord { col: 43, row: 10 },
        size: Size { width: 12, height: 1 },
        typ: prop::ProgressBar {
            fg_color: ColorFG::Black,
            style: PgBarStyle::Rectangle
        }.into(),
        ..Widget::cdeflt()
    },
];

// ---------------------------------------------------------------------------------------------- //

#[rustfmt::skip]
const WINDOW_MAIN: Widget = Widget {
    id: Id::WndMain.into(),
    link: Link::cdeflt(),
    coord: Coord { col: 15, row: 2 },
    size: Size { width: 80, height: 15 },
    typ: prop::Window {
        title: "",
        fg_color: ColorFG::White,
        bg_color: ColorBG::Blue,
        is_popup: false,
    }.into(),
    childs: &[
        Widget {
            id: Id::BtnToaster.into(),
            coord: Coord { col: 1, row: 1 },
            size: Size { width: 14, height: 1 },
            typ: prop::Button {
                text: "",
                fg_color: ColorFG::Yellow,
                bg_color: ColorBG::Inherit,
                style: ButtonStyle::Simple
            }.into(),
            ..Widget::cdeflt()
        },
        Widget {
            id: Id::PgControl.into(),
            coord: Coord { col: 1, row: 1 },
            size: Size { width: 75, height: 12 },
            typ: prop::PageCtrl {
                tab_width: 14,
                vert_offs: 2
            }.into(),
            childs: &[
                Widget {
                    id: Id::PageVer.into(),
                    typ: prop::Page {
                        title: "Version",
                        fg_color: ColorFG::Yellow,
                    }.into(),
                    childs: &PAGE_VER_CHILDS,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageServ.into(),
                    typ: prop::Page {
                        title: "Service ∑",
                        fg_color: ColorFG::White,
                    }.into(),
                    childs: &[],
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageDiag.into(),
                    typ: prop::Page {
                        title: "Diagnostics",
                        fg_color: ColorFG::Yellow,
                    }.into(),
                    childs: &[],
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageInactiv.into(),
                    typ: prop::Page {
                        title: "Inactiv 🍀",
                        fg_color: ColorFG::White,
                    }.into(),
                    childs: &[],
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageTextbox.into(),
                    typ: prop::Page {
                        title: "Text Box",
                        fg_color: ColorFG::White,
                    }.into(),
                    childs: &[],
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageCombobox.into(),
                    typ: prop::Page {
                        title: "Combo Box",
                        fg_color: ColorFG::White,
                    }.into(),
                    childs: &[],
                    ..Widget::cdeflt()
                },
            ],
            ..Widget::cdeflt()
        },
        Widget {
            id: Id::LabelFtr.into(),
            coord: Coord { col: 1, row: 13 },
            typ: prop::Label {
                title:  concat!(
                    "F2 Wnd En  ",
                    "F4 Mouse On  ",
                    "F5 Refresh  ",
                    "F6 Clr Logs  ",
                    "F9/F10 Page  ",
                    "\u{2581}\u{2582}\u{2583}\u{2584}\u{2585}\u{2586}\u{2587}\u{2588}\u{1F569}"
                ),
                fg_color: ColorFG::White,
                bg_color: ColorBG::BlueIntense,
            }.into(),
            ..Widget::cdeflt()
        },
    ]
};

/// Example of const-evaluated and translated Widgets tree into Widgets array
pub const WND_MAIN_ARRAY: [Widget; rtwins::widget_impl::wgt_count(&WINDOW_MAIN)] =
    rtwins::widget_impl::wgt_transform_array(&WINDOW_MAIN);
