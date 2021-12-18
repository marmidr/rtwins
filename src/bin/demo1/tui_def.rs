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
                size: Size { width: 18, height: 1 },
                typ: prop::Label {
                    title: "",
                    fg_color: ColorFG::Black,
                    bg_color: ColorBG::White,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::LabelAbout.into(),
                coord: Coord { col: 2, row: 3 },
                size: Size { width: 0, height: 1 },
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

#[rustfmt::skip]
const PAGE_SERV_CHILDS: &[Widget] = &[
    Widget {
        id: Id::Layer1.into(),
        typ: prop::Layer {
        }.into(),
        childs: &[
            Widget {
                id: Id::LabelMultiFmt.into(),
                coord: Coord { col: 24, row: 2 },
                size: Size { width: 35, height: 4 },
                typ: prop::Label {
                    title: "",
                    fg_color: ColorFG::YellowIntense,
                    bg_color: ColorBG::BlueIntense,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::ListBox.into(),
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 20, height: 8 },
                typ: prop::ListBox {
                    fg_color: ColorFG::Green,
                    bg_color: ColorBG::White,
                    no_frame: false
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::Layer2.into(),
        typ: prop::Layer {
        }.into(),
        childs: &[
            Widget {
                id: Id::Radio1.into(),
                coord: Coord { col: 25, row: 7 },
                typ: prop::Radio {
                    text : "YES",
                    fg_color : ColorFG::GreenIntense,
                    group_id : 1,
                    radio_id : 0,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::Radio2.into(),
                coord: Coord { col: 35, row: 7 },
                typ: prop::Radio {
                    text : "NO",
                    fg_color : ColorFG::Yellow,
                    group_id : 1,
                    radio_id : 1,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::Radio3.into(),
                coord: Coord { col: 44, row: 7 },
                typ: prop::Radio {
                    text : "Don't know",
                    fg_color : ColorFG::Inherit,
                    group_id : 1,
                    radio_id : 3,
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::ChbxL1.into(),
        coord: Coord { col: 25, row: 9 },
        typ: prop::CheckBox {
            text : "Layer 1",
            fg_color : ColorFG::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::ChbxL2.into(),
        coord: Coord { col: 40, row: 9 },
        typ: prop::CheckBox {
            text : "Layer 2",
            fg_color : ColorFG::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_DIAG_CHILDS: &[Widget] = &[
    Widget {
        id: Id::PanelEdt.into(),
        coord: Coord { col: 2, row: 1 },
        size: Size { width: 32, height: 5 },
        typ: prop::Panel {
            title : "",
            fg_color : ColorFG::White,
            bg_color : ColorBG::White,
            no_frame : true,
        }.into(),
        childs: &[
            Widget {
                id: Id::Edt1.into(),
                coord: Coord { col: 1, row: 1 },
                size: Size { width: 30, height: 1 },
                typ: prop::TextEdit {
                    fg_color: ColorFG::Black,
                    bg_color: ColorBG::White,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::Edt2.into(),
                coord: Coord { col: 1, row: 3 },
                size: Size { width: 30, height: 1 },
                typ: prop::TextEdit {
                    fg_color: ColorFG::Black,
                    bg_color: ColorBG::Yellow,
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::CustomWgt1.into(),
        coord: Coord { col: 2, row: 6 },
        size: Size { width: 32, height: 4 },
        typ: prop::CustomWgt {
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::PanelChbx.into(),
        coord: Coord { col: 36, row: 1 },
        size: Size { width: 22, height: 10 },
        typ: prop::Panel {
            title : "",
            fg_color : ColorFG::White,
            bg_color : ColorBG::BlueIntense,
            no_frame : false,
        }.into(),
        childs: &[
            Widget {
                id: Id::LblChbxTitle.into(),
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 14, height: 1 },
                typ: prop::Label {
                    title: "Check list:", // ESC_BOLD "Check list:" ESC_NORMAL
                    fg_color: ColorFG::WhiteIntense,
                    bg_color: ColorBG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::ChbxA.into(),
                coord: Coord { col: 2, row: 4 },
                typ: prop::CheckBox {
                    text : "Check A ",
                    fg_color : ColorFG::YellowIntense,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::ChbxB.into(),
                coord: Coord { col: 2, row: 5 },
                typ: prop::CheckBox {
                    text : "Check B ",
                    fg_color : ColorFG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::ChbxC.into(),
                coord: Coord { col: 2, row: 6 },
                typ: prop::CheckBox {
                    text : "Check C ",
                    fg_color : ColorFG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::ChbxD.into(),
                coord: Coord { col: 2, row: 7 },
                typ: prop::CheckBox {
                    text : "Check D ",
                    fg_color : ColorFG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    }
];

#[rustfmt::skip]
const PAGE_INACT_CHILDS: &[Widget] = &[
    Widget {
        id: Id::PanelEmpty1.into(),
        coord: Coord { col: 5, row: 1 },
        size: Size { width: 20, height: 10 },
        typ: prop::Panel {
            title : "Word-wrap",
            fg_color : ColorFG::Inherit,
            bg_color : ColorBG::Inherit,
            no_frame : true,
        }.into(),
        childs: &[
            Widget {
                id: Id::LblWordwrap.into(),
                coord: Coord { col: 2, row: 1 },
                size: Size { width: 16, height: 6 },
                typ: prop::Label {
                    title: "",
                    fg_color: ColorFG::White,
                    bg_color: ColorBG::Blue,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::BtnNoaction.into(),
                coord: Coord { col: 5, row: 8 },
                typ: prop::Button {
                    text: "...",
                    fg_color: ColorFG::White,
                    bg_color: ColorBG::Inherit,
                    style: ButtonStyle::Simple
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::PanelEmpty2.into(),
        coord: Coord { col: 40, row: 1 },
        size: Size { width: 12, height: 10 },
        typ: prop::Panel {
            title : "...",
            fg_color : ColorFG::Inherit,
            bg_color : ColorBG::Inherit,
            no_frame : false,
        }.into(),
        childs: &[
            Widget {
                id: Id::LblEmpty2.into(),
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 1, height: 1 },
                typ: prop::Label {
                    title: "---",
                    fg_color: ColorFG::Inherit,
                    bg_color: ColorBG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_TXTBOX_CHILDS: &[Widget] = &[
    Widget {
        id: Id::TbxLoremipsum.into(),
        coord: Coord { col: 3, row: 1 },
        size: Size { width: 40, height: 10 },
        typ: prop::TextBox {
            fg_color: ColorFG::White,
            bg_color: ColorBG::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::Tbx1line.into(),
        coord: Coord { col: 46, row: 1 },
        size: Size { width: 12, height: 10 },
        typ: prop::TextBox {
            fg_color: ColorFG::White,
            bg_color: ColorBG::BlueIntense,
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_CMBBOX_CHILDS: &[Widget] = &[
    Widget {
        id: Id::CbxOptions.into(),
        coord: Coord { col: 10, row: 2 },
        size: Size { width: 20, height: 1 },
        typ: prop::ComboBox {
            fg_color: ColorFG::White,
            bg_color: ColorBG::BlueIntense,
            drop_down_size: 4
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::LbxUnderoptions.into(),
        coord: Coord { col: 5, row: 4 },
        size: Size { width: 30, height: 7 },
        typ: prop::ListBox {
            fg_color: ColorFG::Inherit,
            bg_color: ColorBG::Inherit,
            no_frame: false
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::BtnSayYes.into(),
        coord: Coord { col: 38, row: 2 },
        typ: prop::Button {
            text: "Say YES",
            fg_color: ColorFG::White,
            bg_color: ColorBG::Green,
            style: ButtonStyle::Simple
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::BtnSayNo.into(),
        coord: Coord { col: 38, row: 4 },
        typ: prop::Button {
            text: "Say NO",
            fg_color: ColorFG::White,
            bg_color: ColorBG::Red,
            style: ButtonStyle::Simple
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::Btn1p5.into(),
        coord: Coord { col: 38, row: 7 },
        typ: prop::Button {
            text: "   ????   ",
            fg_color: ColorFG::White,
            bg_color: ColorBG::GreenIntense,
            style: ButtonStyle::Solid1p5
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
                    childs: &PAGE_SERV_CHILDS,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageDiag.into(),
                    typ: prop::Page {
                        title: "Diagnostics",
                        fg_color: ColorFG::Yellow,
                    }.into(),
                    childs: &PAGE_DIAG_CHILDS,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageInactiv.into(),
                    typ: prop::Page {
                        title: "Inactiv 🍀",
                        fg_color: ColorFG::White,
                    }.into(),
                    childs: &PAGE_INACT_CHILDS,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageTextbox.into(),
                    typ: prop::Page {
                        title: "Text Box",
                        fg_color: ColorFG::White,
                    }.into(),
                    childs: &PAGE_TXTBOX_CHILDS,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageCombobox.into(),
                    typ: prop::Page {
                        title: "Combo Box",
                        fg_color: ColorFG::White,
                    }.into(),
                    childs: &PAGE_CMBBOX_CHILDS,
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
                    " ",
                    rtwins::csi!("7m"),
                    "F2",
                    rtwins::csi!("27m"),
                    " Wnd En • ",
                    "F4 Mouse On • ",
                    "F5 Refresh • ",
                    "F6 Clr Logs • ",
                    "F9/F10 Page • ",
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
