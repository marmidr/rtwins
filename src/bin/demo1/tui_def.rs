//! Simple widget-based interface definition as const

use rtwins::colors::{ColorBg, ColorFg};
use rtwins::common::*;
use rtwins::esc;
use rtwins::wgt::prop;
use rtwins::wgt::*;

// RGB color values: https://en.wikipedia.org/wiki/Web_colors

#[allow(dead_code)]
#[repr(u8)]
pub enum ColorFgTheme {
    // double-state colors
    Checkbox = ColorFg::Theme00 as u8,
    CheckboxIntense,
    // single-state colors
    Window,
    ButtonGreen,
    ButtonRed,
    ButtonOrange,
    PanelChbox,
    //
    ThemeEnd,
}

#[allow(dead_code)]
#[repr(u8)]
pub enum ColorBgTheme {
    Window = ColorBg::Theme00 as u8,
    ButtonGreen,
    ButtonRed,
    ButtonOrange,
    PanelChbox,
    PanelVer,
    PanelKeyCodes,
    PanelLeds,
    Edit1,
    Edit1Intense,
    Edit2,
    Edit2Intense,
    LabelFtr,
    //
    ThemeEnd,
}

rtwins::static_assert!((ColorFgTheme::ThemeEnd as u8) <= ColorFg::ThemeEnd as u8);
rtwins::static_assert!((ColorBgTheme::ThemeEnd as u8) <= ColorBg::ThemeEnd as u8);

impl ColorFgTheme {
    pub const fn into(self) -> ColorFg {
        // SAFETY: static_assert() assure that the self value is within correct ColorFG range
        unsafe { std::mem::transmute::<ColorFgTheme, ColorFg>(self) }
    }

    pub const fn from(cl: ColorFg) -> ColorFgTheme {
        // SAFETY: static_assert() assure that the self value is within correct ColorFG range
        unsafe { std::mem::transmute::<ColorFg, ColorFgTheme>(cl) }
    }
}

impl ColorBgTheme {
    pub const fn into(self) -> ColorBg {
        // SAFETY: static_assert() assure that the self value is within correct ColorBG range
        unsafe { std::mem::transmute::<ColorBgTheme, ColorBg>(self) }
    }

    pub const fn from(cl: ColorBg) -> ColorBgTheme {
        // SAFETY: static_assert() assure that the self value is within correct ColorBG range
        unsafe { std::mem::transmute::<ColorBg, ColorBgTheme>(cl) }
    }
}

fn color_fg_theme_encode(cl: ColorFg) -> &'static str {
    let cl = ColorFgTheme::from(cl);

    match cl {
        // double-state colors
        ColorFgTheme::Checkbox => esc::FG_DARK_CYAN,
        ColorFgTheme::CheckboxIntense => esc::FG_DARK_TURQUOISE,
        // single-state colors
        ColorFgTheme::Window => rtwins::fg_color!(158),
        ColorFgTheme::ButtonGreen => esc::FG_WHITE,
        ColorFgTheme::ButtonRed => esc::FG_WHITE,
        ColorFgTheme::ButtonOrange => esc::FG_DARK_RED,
        ColorFgTheme::PanelChbox => esc::FG_MEDIUM_BLUE,
        _ => esc::FG_DEFAULT,
    }
}

fn color_bg_theme_encode(cl: ColorBg) -> &'static str {
    let cl = ColorBgTheme::from(cl);

    match cl {
        ColorBgTheme::Window => esc::BG_MIDNIGHT_BLUE,
        ColorBgTheme::ButtonGreen => esc::BG_OLIVE_DRAB,
        ColorBgTheme::ButtonRed => esc::BG_RED,
        ColorBgTheme::ButtonOrange => esc::BG_ORANGE,
        ColorBgTheme::PanelChbox => esc::BG_GAINSBORO,
        ColorBgTheme::PanelVer => rtwins::bg_color!(106),
        ColorBgTheme::PanelKeyCodes => rtwins::bg_color!(169),
        ColorBgTheme::PanelLeds => esc::BG_LIGHT_BLUE,
        ColorBgTheme::Edit1 => esc::BG_CYAN,
        ColorBgTheme::Edit1Intense => esc::BG_CYAN_INTENSE,
        ColorBgTheme::Edit2 => esc::BG_GREEN,
        ColorBgTheme::Edit2Intense => esc::BG_GREEN_INTENSE,
        ColorBgTheme::LabelFtr => esc::BG_NAVY,
        _ => esc::BG_DEFAULT,
    }
}

fn color_fg_theme_intensify(cl: ColorFg) -> ColorFg {
    let cl = match ColorFgTheme::from(cl) {
        ColorFgTheme::Checkbox => ColorFgTheme::CheckboxIntense,
        other => other,
    };

    cl.into()
}

fn color_bg_theme_intensify(cl: ColorBg) -> ColorBg {
    let cl = match ColorBgTheme::from(cl) {
        ColorBgTheme::Edit1 => ColorBgTheme::Edit1Intense,
        ColorBgTheme::Edit2 => ColorBgTheme::Edit2Intense,
        other => other,
    };

    cl.into()
}

pub fn init() {
    ColorFg::set_theme_encoder(color_fg_theme_encode);
    ColorFg::set_theme_intensifier(color_fg_theme_intensify);
    ColorBg::set_theme_encoder(color_bg_theme_encode);
    ColorBg::set_theme_intensifier(color_bg_theme_intensify);
}

// ---------------------------------------------------------------------------------------------- //

/*
macro_rules! generate_ids {
    ($($id: ident)+) => (
        $(
            const $id: WId = 0; // how to make it autoincremental?
        )+
    )
}

#[allow(dead_code)]
mod id {
    use super::WId;

    #[rustfmt::skip]
    generate_ids!(
        WND_MAIN
            BTN_TOASTER
            PG_CONTROL
                PAGE_VER
                    PANEL_VERSIONS
                        LABEL_FW_VERSION
                        LABEL_DATE
                        LABEL_ABOUT
                    PANEL_STATE
    );
}
*/

#[allow(dead_code)]
#[rustfmt::skip]
#[repr(u16)]
#[derive(Clone, Copy)]
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
                    Edit1,
                    LblEdt2Title,
                    Edit2,
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
            fg_color: ColorFg::Black,
            bg_color_off: ColorBg::White,
            bg_color_on: ColorBg::Magenta,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::LedLock.into(),
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
        id: Id::LedPump.into(),
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
        id: Id::PanelVersions.into(),
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
                id: Id::LabelFwVersion.into(),
                coord: Coord { col: 2, row: 1 },
                prop: prop::Label {
                    title: "FwVer: 1.1",
                    fg_color: ColorFg::Blue,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::LabelDate.into(),
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
                id: Id::LabelAbout.into(),
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
        id: Id::PanelState.into(),
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
        id: Id::PanelKey.into(),
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
                id: Id::LabelKeyseq.into(),
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
                id: Id::LabelKeyname.into(),
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
        id: Id::ChbxEnbl.into(),
        coord: Coord { col: 30, row: 5 },
        prop: prop::CheckBox {
            text: "Enable",
            fg_color: ColorFg::White,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::ChbxLock.into(),
        coord: Coord { col: 45, row: 5 },
        prop: prop::CheckBox {
            text: "Lock",
            fg_color: ColorFg::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::BtnYes.into(),
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
        id: Id::BtnNo.into(),
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
        id: Id::BtnPopup.into(),
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
        id: Id::Prgbar1.into(),
        coord: Coord { col: 30, row: 9 },
        size: Size { width: 25, height: 1 },
        prop: prop::ProgressBar {
            fg_color: ColorFg::Yellow,
            style: PgBarStyle::Hash
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::Prgbar2.into(),
        coord: Coord { col: 30, row: 10 },
        size: Size { width: 12, height: 1 },
        prop: prop::ProgressBar {
            fg_color: ColorFg::White,
            style: PgBarStyle::Shade
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::Prgbar3.into(),
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
                    fg_color: ColorFg::YellowIntense,
                    bg_color: ColorBg::BlueIntense,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::ListBox.into(),
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
        id: Id::Layer2.into(),
        prop: prop::Layer {
        }.into(),
        children: &[
            Widget {
                id: Id::Radio1.into(),
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
                id: Id::Radio2.into(),
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
                id: Id::Radio3.into(),
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
        id: Id::ChbxL1.into(),
        coord: Coord { col: 25, row: 9 },
        prop: prop::CheckBox {
            text : "Layer 1",
            fg_color : ColorFg::Inherit,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::ChbxL2.into(),
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
        id: Id::PanelEdt.into(),
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
                id: Id::LblEdt1Title.into(),
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
                id: Id::Edit1.into(),
                coord: Coord { col: 1, row: 1 },
                size: Size { width: 30, height: 1 },
                prop: prop::TextEdit {
                    fg_color: ColorFg::Black,
                    bg_color: ColorBgTheme::Edit1.into(),
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::LblEdt2Title.into(),
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
                id: Id::Edit2.into(),
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
            fg_color : ColorFgTheme::PanelChbox.into(),
            bg_color : ColorBgTheme::PanelChbox.into(),
            no_frame : false,
        }.into(),
        children: &[
            Widget {
                id: Id::LblChbxTitle.into(),
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
                id: Id::ChbxA.into(),
                coord: Coord { col: 2, row: 4 },
                prop: prop::CheckBox {
                    text : "Check A ",
                    fg_color : ColorFg::Green,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::ChbxB.into(),
                coord: Coord { col: 2, row: 5 },
                prop: prop::CheckBox {
                    text : "Check B ",
                    fg_color : ColorFgTheme::Checkbox.into(),
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::ChbxC.into(),
                coord: Coord { col: 2, row: 6 },
                prop: prop::CheckBox {
                    text : "Check C ",
                    fg_color : ColorFgTheme::Checkbox.into(),
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::ChbxD.into(),
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
        id: Id::PanelEmpty1.into(),
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
                id: Id::LblWordwrap.into(),
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
                id: Id::BtnNoaction.into(),
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
        id: Id::PanelEmpty2.into(),
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
                id: Id::LblEmpty2.into(),
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
        id: Id::TbxWide.into(),
        coord: Coord { col: 3, row: 1 },
        size: Size { width: 40, height: 10 },
        prop: prop::TextBox {
            fg_color: ColorFg::White,
            bg_color: ColorBg::BlueIntense,
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::TbxNarrow.into(),
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
        id: Id::CbxOptions.into(),
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
        id: Id::CbxColors.into(),
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
        id: Id::LbxUnderoptions.into(),
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
        id: Id::BtnSayYes.into(),
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
        id: Id::BtnSayNo.into(),
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
        id: Id::Btn1p5.into(),
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
    id: Id::WndMain.into(),
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
            id: Id::BtnToaster.into(),
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
                        fg_color: ColorFg::Yellow,
                    }.into(),
                    children: PAGE_VER_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageServ.into(),
                    prop: prop::Page {
                        title: "Service ∑",
                        fg_color: ColorFg::White,
                    }.into(),
                    children: PAGE_SERV_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageDiag.into(),
                    prop: prop::Page {
                        title: "Diagnostics",
                        fg_color: ColorFg::Yellow,
                    }.into(),
                    children: PAGE_DIAG_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageInactiv.into(),
                    prop: prop::Page {
                        title: "Inactiv 🍀",
                        fg_color: ColorFg::White,
                    }.into(),
                    children: PAGE_INACT_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageTextbox.into(),
                    prop: prop::Page {
                        title: "Text Box",
                        fg_color: ColorFg::White,
                    }.into(),
                    children: PAGE_TXTBOX_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageCombobox.into(),
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
