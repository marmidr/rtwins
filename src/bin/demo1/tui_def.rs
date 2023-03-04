//! Simple widget-based interface definition as const

use rtwins::colors::{ColorBg, ColorFg};
use rtwins::common::*;
use rtwins::wgt::prop;
use rtwins::wgt::*;

use super::tui_colors::*;

// ---------------------------------------------------------------------------------------------- //

pub mod id {
    use rtwins::wgt::{WId, WIDGET_ID_NONE};

    #[rustfmt::skip]
    rtwins::generate_ids!(
        WndMain
            BtnToaster
            PgControl
                PageVer
                    PanelVersions
                        LabelFwVersion
                        LabelDate
                        LabelAbout
                    PanelState
                        LedPump
                        LedLock
                        LedBattery
                    PanelKey
                        LabelKeyseq
                        LabelKeyname
                    ChbxEnbl
                    ChbxLock
                    BtnYes
                    BtnNo
                    BtnPopup
                    Prgbar1
                    Prgbar2
                    Prgbar3
                PageServ
                    Layer1
                        LabelMultiFmt
                        ListBox
                    Layer2
                        Radio1
                        Radio2
                        Radio3
                    ChbxL1
                    ChbxL2
                PageDiag
                    PanelEdt
                        LblEdt1Title
                        Edit1
                        LblEdt2Title
                        Edit2
                    CustomWgt1
                    PanelChbx
                        LblChbxTitle
                        ChbxA
                        ChbxB
                        ChbxC
                        ChbxD
                PageInactiv
                    PanelEmpty1
                        LblWordwrap
                        BtnNoaction
                    PanelEmpty2
                        LblEmpty2
                PageTextbox
                    TbxWide
                    TbxNarrow
                PageCombobox
                    CbxOptions
                    CbxColors
                    LbxUnderoptions
                    BtnSayYes
                    BtnSayNo
                    Btn1p5
            LabelFtr
    );
}

// ---------------------------------------------------------------------------------------------- //

#[rustfmt::skip]
const PNL_STATE_CHILDREN: &[Widget] = &[
    Widget {
        id: id::LedBattery,
        coord: Coord { col: 2, row: 1 },
        prop: prop::Led {
            text: "(BATT)",
            fg_color: ColorFg::Black,
            bg_color_off: ColorBg::White,
            bg_color_on: ColorBg::Magenta,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::LedLock,
        coord: Coord { col: 9, row: 1 },
        prop: prop::Led {
            text: "(LOCK)",
            fg_color: ColorFg::Black,
            bg_color_off: ColorBg::White,
            bg_color_on: ColorBg::Green,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::LedPump,
        coord: Coord { col: 16, row: 1 },
        prop: prop::Led {
            text: "(PUMP)",
            fg_color: ColorFg::Red,
            bg_color_off: ColorBg::White,
            bg_color_on: ColorBg::Yellow,
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_VER_CHILDREN: &[Widget] = &[
    Widget {
        id: id::PanelVersions,
        link: Link::cdeflt(),
        coord: Coord { col: 1, row: 1 },
        size: Size { width: 21, height: 5 },
        prop: prop::Panel {
            title: "VER 🍁",
            fg_color: ColorFg::White,
            bg_color: ColorBgTheme::PanelVer.into(),
            no_frame: false,
        }.into(),
        children: &[
            Widget {
                id: id::LabelFwVersion,
                coord: Coord { col: 2, row: 1 },
                prop: prop::Label {
                    title: "FwVer: 1.1",
                    fg_color: ColorFg::Blue,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::LabelDate,
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 18, height: 1 },
                prop: prop::Label {
                    title: "",
                    fg_color: ColorFg::Black,
                    bg_color: ColorBg::White,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::LabelAbout,
                coord: Coord { col: 2, row: 3 },
                size: Size { width: 0, height: 1 },
                prop: prop::Label {
                    title: "",
                    fg_color: ColorFg::White,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
        ]
    },
    Widget {
        id: id::PanelState,
        link: Link::cdeflt(),
        coord: Coord { col: 30, row: 1 },
        size: Size { width: 25, height: 3 },
        prop: prop::Panel {
            title: "STATE: Leds",
            fg_color: ColorFg::Blue,
            bg_color: ColorBgTheme::PanelLeds.into(),
            no_frame: false,
        }.into(),
        children: PNL_STATE_CHILDREN
    },
    Widget {
        id: id::PanelKey,
        link: Link::cdeflt(),
        coord: Coord { col: 1, row: 7 },
        size: Size { width: 26, height: 4 },
        prop: prop::Panel {
            title: "KEY-CODES",
            fg_color: ColorFg::White,
            bg_color: ColorBgTheme::PanelKeyCodes.into(),
            no_frame: false,
        }.into(),
        children: &[
            Widget {
                id: id::LabelKeyseq,
                coord: Coord { col: 2, row: 1 },
                size: Size { width: 22, height: 1 },
                prop: prop::Label {
                    title: "",
                    fg_color: ColorFg::White,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::LabelKeyname,
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 17, height: 1 },
                prop: prop::Label {
                    title: "",
                    fg_color: ColorFg::White,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
        ]
    },
    Widget {
        id: id::ChbxEnbl,
        coord: Coord { col: 30, row: 5 },
        prop: prop::CheckBox {
            text: "Enable",
            fg_color: ColorFg::White,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::ChbxLock,
        coord: Coord { col: 45, row: 5 },
        prop: prop::CheckBox {
            text: "Lock",
            fg_color: ColorFg::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::BtnYes,
        coord: Coord { col: 30, row: 7 },
        prop: prop::Button {
            text: "YES",
            fg_color: ColorFg::White,
            bg_color: ColorBgTheme::ButtonGreen.into(),
            style: ButtonStyle::Solid
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::BtnNo,
        coord: Coord { col: 38, row: 7 },
        prop: prop::Button {
            text: "NO",
            fg_color: ColorFgTheme::ButtonOrange.into(),
            bg_color: ColorBgTheme::ButtonOrange.into(),
            style: ButtonStyle::Solid
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::BtnPopup,
        coord: Coord { col: 45, row: 7 },
        prop: prop::Button {
            text: "POPUP",
            fg_color: ColorFg::White,
            bg_color: ColorBg::Inherit,
            style: ButtonStyle::Simple
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::Prgbar1,
        coord: Coord { col: 30, row: 9 },
        size: Size { width: 25, height: 1 },
        prop: prop::ProgressBar {
            fg_color: ColorFg::Yellow,
            style: PgBarStyle::Hash
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::Prgbar2,
        coord: Coord { col: 30, row: 10 },
        size: Size { width: 12, height: 1 },
        prop: prop::ProgressBar {
            fg_color: ColorFg::White,
            style: PgBarStyle::Shade
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::Prgbar3,
        coord: Coord { col: 43, row: 10 },
        size: Size { width: 12, height: 1 },
        prop: prop::ProgressBar {
            fg_color: ColorFg::Black,
            style: PgBarStyle::Rectangle
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_SERV_CHILDREN: &[Widget] = &[
    Widget {
        id: id::Layer1,
        prop: prop::Layer {
        }.into(),
        children: &[
            Widget {
                id: id::LabelMultiFmt,
                coord: Coord { col: 24, row: 2 },
                size: Size { width: 35, height: 4 },
                prop: prop::Label {
                    title: "",
                    fg_color: ColorFg::YellowIntense,
                    bg_color: ColorBg::BlueIntense,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::ListBox,
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 20, height: 8 },
                prop: prop::ListBox {
                    fg_color: ColorFg::Green,
                    bg_color: ColorBg::White,
                    no_frame: false
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    },
    Widget {
        id: id::Layer2,
        prop: prop::Layer {
        }.into(),
        children: &[
            Widget {
                id: id::Radio1,
                coord: Coord { col: 25, row: 7 },
                prop: prop::Radio {
                    text : "YES",
                    fg_color : ColorFg::GreenIntense,
                    group_id : 1,
                    radio_id : 0,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::Radio2,
                coord: Coord { col: 35, row: 7 },
                prop: prop::Radio {
                    text : "NO",
                    fg_color : ColorFg::Yellow,
                    group_id : 1,
                    radio_id : 1,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::Radio3,
                coord: Coord { col: 44, row: 7 },
                prop: prop::Radio {
                    text : "Don't know",
                    fg_color : ColorFg::Inherit,
                    group_id : 1,
                    radio_id : 3,
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    },
    Widget {
        id: id::ChbxL1,
        coord: Coord { col: 25, row: 9 },
        prop: prop::CheckBox {
            text : "Layer 1",
            fg_color : ColorFg::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::ChbxL2,
        coord: Coord { col: 40, row: 9 },
        prop: prop::CheckBox {
            text : "Layer 2",
            fg_color : ColorFg::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_DIAG_CHILDREN: &[Widget] = &[
    Widget {
        id: id::PanelEdt,
        coord: Coord { col: 2, row: 1 },
        size: Size { width: 32, height: 5 },
        prop: prop::Panel {
            title : "",
            fg_color : ColorFg::White,
            bg_color : ColorBg::BlackIntense,
            no_frame : true,
        }.into(),
        children: &[
            Widget {
                id: id::LblEdt1Title,
                coord: Coord { col: 1, row: 0 },
                size: Size { width: 30, height: 1 },
                prop: prop::Label {
                    title: "Text edit:",
                    fg_color: ColorFg::WhiteIntense,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::Edit1,
                coord: Coord { col: 1, row: 1 },
                size: Size { width: 30, height: 1 },
                prop: prop::TextEdit {
                    fg_color: ColorFg::Black,
                    bg_color: ColorBgTheme::Edit1.into(),
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::LblEdt2Title,
                coord: Coord { col: 1, row: 2 },
                size: Size { width: 30, height: 1 },
                prop: prop::Label {
                    title: "Num edit: UP/DOWN + Ctr/Shift:",
                    fg_color: ColorFg::WhiteIntense,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::Edit2,
                coord: Coord { col: 1, row: 3 },
                size: Size { width: 30, height: 1 },
                prop: prop::TextEdit {
                    fg_color: ColorFg::Black,
                    bg_color: ColorBgTheme::Edit2.into(),
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    },
    Widget {
        id: id::CustomWgt1,
        coord: Coord { col: 2, row: 6 },
        size: Size { width: 32, height: 4 },
        prop: prop::CustomWgt {
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::PanelChbx,
        coord: Coord { col: 36, row: 1 },
        size: Size { width: 22, height: 10 },
        prop: prop::Panel {
            title : "",
            fg_color : ColorFgTheme::PanelChbox.into(),
            bg_color : ColorBgTheme::PanelChbox.into(),
            no_frame : false,
        }.into(),
        children: &[
            Widget {
                id: id::LblChbxTitle,
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 14, height: 1 },
                prop: prop::Label {
                    title: "Check list:", // concat!(bold!(), "Check list:", normal!()),
                    fg_color: ColorFg::Blue,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::ChbxA,
                coord: Coord { col: 2, row: 4 },
                prop: prop::CheckBox {
                    text : "Check A ",
                    fg_color : ColorFg::Green,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::ChbxB,
                coord: Coord { col: 2, row: 5 },
                prop: prop::CheckBox {
                    text : "Check B ",
                    fg_color : ColorFgTheme::Checkbox.into(),
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::ChbxC,
                coord: Coord { col: 2, row: 6 },
                prop: prop::CheckBox {
                    text : "Check C ",
                    fg_color : ColorFgTheme::Checkbox.into(),
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::ChbxD,
                coord: Coord { col: 2, row: 7 },
                prop: prop::CheckBox {
                    text : "Check D ",
                    fg_color : ColorFgTheme::Checkbox.into(),
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
        id: id::PanelEmpty1,
        coord: Coord { col: 5, row: 1 },
        size: Size { width: 20, height: 10 },
        prop: prop::Panel {
            title : "Word-wrap",
            fg_color : ColorFg::Inherit,
            bg_color : ColorBg::Inherit,
            no_frame : false,
        }.into(),
        children: &[
            Widget {
                id: id::LblWordwrap,
                coord: Coord { col: 2, row: 1 },
                size: Size { width: 16, height: 6 },
                prop: prop::Label {
                    title: "",
                    fg_color: ColorFg::White,
                    bg_color: ColorBg::BlueIntense,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: id::BtnNoaction,
                coord: Coord { col: 5, row: 8 },
                prop: prop::Button {
                    text: "...",
                    fg_color: ColorFg::White,
                    bg_color: ColorBg::Inherit,
                    style: ButtonStyle::Simple
                }.into(),
                ..Widget::cdeflt()
            },
        ],
        ..Widget::cdeflt()
    },
    Widget {
        id: id::PanelEmpty2,
        coord: Coord { col: 40, row: 1 },
        size: Size { width: 12, height: 10 },
        prop: prop::Panel {
            title : "...",
            fg_color : ColorFg::Inherit,
            bg_color : ColorBg::Inherit,
            no_frame : false,
        }.into(),
        children: &[
            Widget {
                id: id::LblEmpty2,
                coord: Coord { col: 2, row: 2 },
                size: Size { width: 1, height: 1 },
                prop: prop::Label {
                    title: "---",
                    fg_color: ColorFg::Inherit,
                    bg_color: ColorBg::Inherit,
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
        id: id::TbxWide,
        coord: Coord { col: 3, row: 1 },
        size: Size { width: 40, height: 10 },
        prop: prop::TextBox {
            fg_color: ColorFg::White,
            bg_color: ColorBg::BlueIntense,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::TbxNarrow,
        coord: Coord { col: 46, row: 1 },
        size: Size { width: 12, height: 10 },
        prop: prop::TextBox {
            fg_color: ColorFg::White,
            bg_color: ColorBg::BlueIntense,
        }.into(),
        ..Widget::cdeflt()
    },
];

#[rustfmt::skip]
const PAGE_CMBBOX_CHILDREN: &[Widget] = &[
    Widget {
        id: id::CbxOptions,
        coord: Coord { col: 10, row: 2 },
        size: Size { width: 20, height: 1 },
        prop: prop::ComboBox {
            fg_color: ColorFg::Blue,
            bg_color: ColorBg::White,
            drop_down_size: 4
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::CbxColors,
        coord: Coord { col: 8, row: 4 },
        size: Size { width: 24, height: 1 },
        prop: prop::ComboBox {
            fg_color: ColorFg::GreenIntense,
            bg_color: ColorBg::Black,
            drop_down_size: 4
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::LbxUnderoptions,
        coord: Coord { col: 5, row: 6 },
        size: Size { width: 30, height: 5 },
        prop: prop::ListBox {
            fg_color: ColorFg::Inherit,
            bg_color: ColorBg::Inherit,
            no_frame: false
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::BtnSayYes,
        coord: Coord { col: 38, row: 2 },
        prop: prop::Button {
            text: "Say YES",
            fg_color: ColorFgTheme::ButtonGreen.into(),
            bg_color: ColorBg::Green,
            style: ButtonStyle::Simple
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::BtnSayNo,
        coord: Coord { col: 38, row: 4 },
        prop: prop::Button {
            text: "Say NO",
            fg_color: ColorFgTheme::ButtonRed.into(),
            bg_color: ColorBg::Red,
            style: ButtonStyle::Simple
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: id::Btn1p5,
        coord: Coord { col: 38, row: 7 },
        prop: prop::Button {
            text: "   ????   ",
            fg_color: ColorFg::White,
            bg_color: ColorBgTheme::ButtonGreen.into(),
            style: ButtonStyle::Solid1p5
        }.into(),
        ..Widget::cdeflt()
    },
];

// ---------------------------------------------------------------------------------------------- //

#[rustfmt::skip]
const WINDOW_MAIN: Widget = Widget {
    id: id::WndMain,
    link: Link::cdeflt(),
    coord: Coord { col: 5, row: 2 },
    size: Size { width: 80, height: 15 },
    prop: prop::Window {
        title: "",
        fg_color: ColorFgTheme::Window.into(),
        bg_color: ColorBgTheme::Window.into(),
        is_popup: false,
    }.into(),
    children: &[
        Widget {
            id: id::BtnToaster,
            coord: Coord { col: 1, row: 1 },
            size: Size { width: 14, height: 1 },
            prop: prop::Button {
                text: "",
                fg_color: ColorFg::Yellow,
                bg_color: ColorBg::Inherit,
                style: ButtonStyle::Simple
            }.into(),
            ..Widget::cdeflt()
        },
        Widget {
            id: id::PgControl,
            coord: Coord { col: 1, row: 1 },
            size: Size { width: 75, height: 12 },
            prop: prop::PageCtrl {
                tab_width: 14,
                vert_offs: 2
            }.into(),
            children: &[
                Widget {
                    id: id::PageVer,
                    prop: prop::Page {
                        title: "Version",
                        fg_color: ColorFg::Yellow,
                    }.into(),
                    children: PAGE_VER_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: id::PageServ,
                    prop: prop::Page {
                        title: "Service ∑",
                        fg_color: ColorFg::White,
                    }.into(),
                    children: PAGE_SERV_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: id::PageDiag,
                    prop: prop::Page {
                        title: "Diagnostics",
                        fg_color: ColorFg::Yellow,
                    }.into(),
                    children: PAGE_DIAG_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: id::PageInactiv,
                    prop: prop::Page {
                        title: "Inactiv 🍀",
                        fg_color: ColorFg::White,
                    }.into(),
                    children: PAGE_INACT_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: id::PageTextbox,
                    prop: prop::Page {
                        title: "Text Box",
                        fg_color: ColorFg::White,
                    }.into(),
                    children: PAGE_TXTBOX_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: id::PageCombobox,
                    prop: prop::Page {
                        title: "Combo Box",
                        fg_color: ColorFg::White,
                    }.into(),
                    children: PAGE_CMBBOX_CHILDREN,
                    ..Widget::cdeflt()
                },
            ],
            ..Widget::cdeflt()
        },
        Widget {
            id: id::LabelFtr,
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
                fg_color: ColorFg::White,
                bg_color: ColorBgTheme::LabelFtr.into(),
            }.into(),
            ..Widget::cdeflt()
        },
    ]
};

/// Example of const-evaluated and translated Widgets tree into Widgets array
pub const WND_MAIN_ARRAY: [Widget; rtwins::wgt::transform::tree_wgt_count(&WINDOW_MAIN)] =
    rtwins::wgt::transform::tree_to_array(&WINDOW_MAIN);
