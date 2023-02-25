//! Simple widget-based interface definition as const

use rtwins::colors::{ColorBG, ColorFG};
use rtwins::prop;
use rtwins::*;

#[allow(dead_code)]
#[rustfmt::skip]
#[repr(u16)]
#[derive(Clone, Copy)]
// TODO: instead of enum, use some macro defining const ID_BTN_TOASTER: WId = 1;
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
                    LblEdt1Title,
                    Edt1,
                    LblEdt2Title,
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
                TbxWide,
                TbxNarrow,
            PageCombobox,
                CbxOptions,
                CbxColors,
                LbxUnderoptions,
                BtnSayYes,
                BtnSayNo,
                Btn1p5,
        LabelFtr,
}

/// Easy conversion from enum to Wid
impl Id {
    #[inline]
    pub const fn into(self) -> WId {
        self as WId
    }
}

/// Helper to avoid code like this
/// `Id::LabelMultiFmt.into() == wgt.id`
// impl std::cmp::PartialEq<WId> for Id {
//     #[inline]
//     fn eq(&self, other: &WId) -> bool {
//         *self as WId == *other
//     }
// }

/// Helper to avoid using `.into()` in WId == Id comparison
impl std::cmp::PartialEq<Id> for WId {
    #[inline]
    fn eq(&self, other: &Id) -> bool {
        *self == *other as WId
    }
}

// ---------------------------------------------------------------------------------------------- //

#[rustfmt::skip]
const PNL_STATE_CHILDREN: &[Widget] = &[
    Widget {
        id: Id::LedBattery.into(),
        coord: Coord { col: 2, row: 1 },
        prop: prop::Led {
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
        prop: prop::Led {
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
        prop: prop::Led {
            text: "(PUMP)",
            fg_color: ColorFG::Red,
            bg_color_off: ColorBG::White,
            bg_color_on: ColorBG::Yellow,
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_VER_CHILDREN: &[Widget] = &[
    Widget {
        id: Id::PanelVersions.into(),
        link: Link::cdeflt(),
        coord: Coord { col: 1, row: 1 },
        size: Size { width: 21, height: 5 },
        prop: prop::Panel {
            title: "VER 🍁",
            fg_color: ColorFG::White,
            bg_color: ColorBG::Green,
            no_frame: false,
        }.into(),
        children: &[
            Widget {
                id: Id::LabelFwVersion.into(),
                coord: Coord { col: 2, row: 1 },
                prop: prop::Label {
                    title: "FwVer: 1.1",
                    fg_color: ColorFG::Blue,
                    bg_color: ColorBG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::LabelDate.into(),
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 18, height: 1 },
                prop: prop::Label {
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
                prop: prop::Label {
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
        prop: prop::Panel {
            title: "STATE: Leds",
            fg_color: ColorFG::Blue,
            bg_color: ColorBG::White,
            no_frame: false,
        }.into(),
        children: PNL_STATE_CHILDREN
    },
    Widget {
        id: Id::PanelKey.into(),
        link: Link::cdeflt(),
        coord: Coord { col: 1, row: 7 },
        size: Size { width: 26, height: 4 },
        prop: prop::Panel {
            title: "KEY-CODES",
            fg_color: ColorFG::White,
            bg_color: ColorBG::Magenta,
            no_frame: false,
        }.into(),
        children: &[
            Widget {
                id: Id::LabelKeyseq.into(),
                coord: Coord { col: 2, row: 1 },
                size: Size { width: 22, height: 1 },
                prop: prop::Label {
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
                prop: prop::Label {
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
        prop: prop::CheckBox {
            text: "Enable",
            fg_color: ColorFG::White,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::ChbxLock.into(),
        coord: Coord { col: 45, row: 5 },
        prop: prop::CheckBox {
            text: "Lock",
            fg_color: ColorFG::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::BtnYes.into(),
        coord: Coord { col: 30, row: 7 },
        prop: prop::Button {
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
        prop: prop::Button {
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
        prop: prop::Button {
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
        prop: prop::ProgressBar {
            fg_color: ColorFG::Yellow,
            style: PgBarStyle::Hash
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::Prgbar2.into(),
        coord: Coord { col: 30, row: 10 },
        size: Size { width: 12, height: 1 },
        prop: prop::ProgressBar {
            fg_color: ColorFG::White,
            style: PgBarStyle::Shade
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::Prgbar3.into(),
        coord: Coord { col: 43, row: 10 },
        size: Size { width: 12, height: 1 },
        prop: prop::ProgressBar {
            fg_color: ColorFG::Black,
            style: PgBarStyle::Rectangle
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_SERV_CHILDREN: &[Widget] = &[
    Widget {
        id: Id::Layer1.into(),
        prop: prop::Layer {
        }.into(),
        children: &[
            Widget {
                id: Id::LabelMultiFmt.into(),
                coord: Coord { col: 24, row: 2 },
                size: Size { width: 35, height: 4 },
                prop: prop::Label {
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
                prop: prop::ListBox {
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
        prop: prop::Layer {
        }.into(),
        children: &[
            Widget {
                id: Id::Radio1.into(),
                coord: Coord { col: 25, row: 7 },
                prop: prop::Radio {
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
                prop: prop::Radio {
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
                prop: prop::Radio {
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
        prop: prop::CheckBox {
            text : "Layer 1",
            fg_color : ColorFG::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::ChbxL2.into(),
        coord: Coord { col: 40, row: 9 },
        prop: prop::CheckBox {
            text : "Layer 2",
            fg_color : ColorFG::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_DIAG_CHILDREN: &[Widget] = &[
    Widget {
        id: Id::PanelEdt.into(),
        coord: Coord { col: 2, row: 1 },
        size: Size { width: 32, height: 5 },
        prop: prop::Panel {
            title : "",
            fg_color : ColorFG::White,
            bg_color : ColorBG::BlackIntense,
            no_frame : true,
        }.into(),
        children: &[
            Widget {
                id: Id::LblEdt1Title.into(),
                coord: Coord { col: 1, row: 0 },
                size: Size { width: 30, height: 1 },
                prop: prop::Label {
                    title: "Text edit:",
                    fg_color: ColorFG::WhiteIntense,
                    bg_color: ColorBG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::Edt1.into(),
                coord: Coord { col: 1, row: 1 },
                size: Size { width: 30, height: 1 },
                prop: prop::TextEdit {
                    fg_color: ColorFG::Black,
                    bg_color: ColorBG::White,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::LblEdt2Title.into(),
                coord: Coord { col: 1, row: 2 },
                size: Size { width: 30, height: 1 },
                prop: prop::Label {
                    title: "Num edit: UP/DOWN + Ctr/Shift:",
                    fg_color: ColorFG::WhiteIntense,
                    bg_color: ColorBG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::Edt2.into(),
                coord: Coord { col: 1, row: 3 },
                size: Size { width: 30, height: 1 },
                prop: prop::TextEdit {
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
        prop: prop::CustomWgt {
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::PanelChbx.into(),
        coord: Coord { col: 36, row: 1 },
        size: Size { width: 22, height: 10 },
        prop: prop::Panel {
            title : "",
            fg_color : ColorFG::White,
            bg_color : ColorBG::BlueIntense,
            no_frame : false,
        }.into(),
        children: &[
            Widget {
                id: Id::LblChbxTitle.into(),
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 14, height: 1 },
                prop: prop::Label {
                    title: "Check list:", // concat!(bold!(), "Check list:", normal!()),
                    fg_color: ColorFG::WhiteIntense,
                    bg_color: ColorBG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::ChbxA.into(),
                coord: Coord { col: 2, row: 4 },
                prop: prop::CheckBox {
                    text : "Check A ",
                    fg_color : ColorFG::YellowIntense,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::ChbxB.into(),
                coord: Coord { col: 2, row: 5 },
                prop: prop::CheckBox {
                    text : "Check B ",
                    fg_color : ColorFG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::ChbxC.into(),
                coord: Coord { col: 2, row: 6 },
                prop: prop::CheckBox {
                    text : "Check C ",
                    fg_color : ColorFG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::ChbxD.into(),
                coord: Coord { col: 2, row: 7 },
                prop: prop::CheckBox {
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
const PAGE_INACT_CHILDREN: &[Widget] = &[
    Widget {
        id: Id::PanelEmpty1.into(),
        coord: Coord { col: 5, row: 1 },
        size: Size { width: 20, height: 10 },
        prop: prop::Panel {
            title : "Word-wrap",
            fg_color : ColorFG::Inherit,
            bg_color : ColorBG::Inherit,
            no_frame : false,
        }.into(),
        children: &[
            Widget {
                id: Id::LblWordwrap.into(),
                coord: Coord { col: 2, row: 1 },
                size: Size { width: 16, height: 6 },
                prop: prop::Label {
                    title: "",
                    fg_color: ColorFG::White,
                    bg_color: ColorBG::BlueIntense,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::BtnNoaction.into(),
                coord: Coord { col: 5, row: 8 },
                prop: prop::Button {
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
        prop: prop::Panel {
            title : "...",
            fg_color : ColorFG::Inherit,
            bg_color : ColorBG::Inherit,
            no_frame : false,
        }.into(),
        children: &[
            Widget {
                id: Id::LblEmpty2.into(),
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 1, height: 1 },
                prop: prop::Label {
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
const PAGE_TXTBOX_CHILDREN: &[Widget] = &[
    Widget {
        id: Id::TbxWide.into(),
        coord: Coord { col: 3, row: 1 },
        size: Size { width: 40, height: 10 },
        prop: prop::TextBox {
            fg_color: ColorFG::White,
            bg_color: ColorBG::BlueIntense,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::TbxNarrow.into(),
        coord: Coord { col: 46, row: 1 },
        size: Size { width: 12, height: 10 },
        prop: prop::TextBox {
            fg_color: ColorFG::White,
            bg_color: ColorBG::BlueIntense,
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_CMBBOX_CHILDREN: &[Widget] = &[
    Widget {
        id: Id::CbxOptions.into(),
        coord: Coord { col: 10, row: 2 },
        size: Size { width: 20, height: 1 },
        prop: prop::ComboBox {
            fg_color: ColorFG::Blue,
            bg_color: ColorBG::White,
            drop_down_size: 4
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::CbxColors.into(),
        coord: Coord { col: 8, row: 4 },
        size: Size { width: 24, height: 1 },
        prop: prop::ComboBox {
            fg_color: ColorFG::GreenIntense,
            bg_color: ColorBG::Black,
            drop_down_size: 4
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::LbxUnderoptions.into(),
        coord: Coord { col: 5, row: 6 },
        size: Size { width: 30, height: 5 },
        prop: prop::ListBox {
            fg_color: ColorFG::Inherit,
            bg_color: ColorBG::Inherit,
            no_frame: false
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::BtnSayYes.into(),
        coord: Coord { col: 38, row: 2 },
        prop: prop::Button {
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
        prop: prop::Button {
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
        prop: prop::Button {
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
    coord: Coord { col: 5, row: 2 },
    size: Size { width: 80, height: 15 },
    prop: prop::Window {
        title: "",
        fg_color: ColorFG::White,
        bg_color: ColorBG::Blue,
        is_popup: false,
    }.into(),
    children: &[
        Widget {
            id: Id::BtnToaster.into(),
            coord: Coord { col: 1, row: 1 },
            size: Size { width: 14, height: 1 },
            prop: prop::Button {
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
            prop: prop::PageCtrl {
                tab_width: 14,
                vert_offs: 2
            }.into(),
            children: &[
                Widget {
                    id: Id::PageVer.into(),
                    prop: prop::Page {
                        title: "Version",
                        fg_color: ColorFG::Yellow,
                    }.into(),
                    children: PAGE_VER_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageServ.into(),
                    prop: prop::Page {
                        title: "Service ∑",
                        fg_color: ColorFG::White,
                    }.into(),
                    children: PAGE_SERV_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageDiag.into(),
                    prop: prop::Page {
                        title: "Diagnostics",
                        fg_color: ColorFG::Yellow,
                    }.into(),
                    children: PAGE_DIAG_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageInactiv.into(),
                    prop: prop::Page {
                        title: "Inactiv 🍀",
                        fg_color: ColorFG::White,
                    }.into(),
                    children: PAGE_INACT_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageTextbox.into(),
                    prop: prop::Page {
                        title: "Text Box",
                        fg_color: ColorFG::White,
                    }.into(),
                    children: PAGE_TXTBOX_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageCombobox.into(),
                    prop: prop::Page {
                        title: "Combo Box",
                        fg_color: ColorFG::White,
                    }.into(),
                    children: PAGE_CMBBOX_CHILDREN,
                    ..Widget::cdeflt()
                },
            ],
            ..Widget::cdeflt()
        },
        Widget {
            id: Id::LabelFtr.into(),
            coord: Coord { col: 1, row: 13 },
            prop: prop::Label {
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
pub const WND_MAIN_ARRAY: [Widget; rtwins::wgt::transform::tree_wgt_count(&WINDOW_MAIN)] =
    rtwins::wgt::transform::tree_to_array(&WINDOW_MAIN);
