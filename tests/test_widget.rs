//! # RTWins Widget tests

extern crate rtwins;
use rtwins::widget::*;
use rtwins::widget_impl::*;
use rtwins::colors::*;

#[rustfmt::skip]
#[repr(u16)]
enum Id {
    WndTest = WIDGET_ID_NONE + 1,
        BtnToaster,
        PgControl,
            PageVer,
                PanelVersions,
                    LabelFwVersion,
                    LabelAbout,
                BtnYes,
                BtnNo,
            PageServ,
                ChbxEnbl,
                ChbxLock,
        NotExistingWgt
}

impl Id {
    pub const fn into(self) -> WId {
        self as WId
    }
}

#[rustfmt::skip]
const PAGE_VER_CHILDS: &[Widget] = &[
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
        childs: &[
            Widget {
                id: Id::LabelFwVersion.into(),
                coord: Coord { col: 2, row: 1 },
                prop: prop::Label {
                    title: "FwVer: 1.1",
                    fg_color: ColorFG::YellowIntense,
                    bg_color: ColorBG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::LabelAbout.into(),
                coord: Coord { col: 2, row: 3 },
                size: Size { width: 0, height: 1 },
                prop: prop::Label {
                    title: "",
                    fg_color: ColorFG::Blue,
                    bg_color: ColorBG::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
        ]
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
];

#[rustfmt::skip]
const WINDOW_TEST: Widget = Widget {
    id: Id::WndTest.into(),
    link: Link::cdeflt(),
    coord: Coord { col: 15, row: 2 },
    size: Size { width: 80, height: 15 },
    prop: prop::Window {
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
            childs: &[
                Widget {
                    id: Id::PageVer.into(),
                    prop: prop::Page {
                        title: "Version",
                        fg_color: ColorFG::Yellow,
                    }.into(),
                    childs: &PAGE_VER_CHILDS,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageServ.into(),
                    prop: prop::Page {
                        title: "Service ∑",
                        fg_color: ColorFG::White,
                    }.into(),
                    childs: &[
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
                    ],
                    ..Widget::cdeflt()
                },
            ],
            ..Widget::cdeflt()
        },
    ]
};


const WND_TEST_ARRAY: [Widget; rtwins::widget_impl::wgt_count(&WINDOW_TEST)] =
    rtwins::widget_impl::wgt_transform_array(&WINDOW_TEST);

// -----------------------------------------------------------------------------------------------

#[test]
fn test_wgt_get_parent() {
    // window parent is the window itself
    {
        let wgt = &WND_TEST_ARRAY[0];
        let par = wgt_get_parent(wgt);
        assert!(par.id == wgt.id);
    }

    // panel parent is the window
    {
        let wnd = &WND_TEST_ARRAY[0];
        let wgt = wgt_find_by_id(Id::PgControl.into(), &WND_TEST_ARRAY);
        assert!(wgt.is_some());
        let par = wgt_get_parent(wgt.unwrap());
        assert!(par.id == wnd.id);
    }

    // try to find invalid widget
    {
        let wgt = wgt_find_by_id(Id::NotExistingWgt.into(), &WND_TEST_ARRAY);
        assert!(wgt.is_none());
    }
}

#[test]
fn test_wgt_iter() {
    // iterate over panel
    {
        let pnl_vers = wgt_find_by_id(Id::PanelVersions.into(), &WND_TEST_ARRAY);
        assert!(pnl_vers.is_some());
        let pnl_vers = pnl_vers.unwrap();

        assert_eq!(2, WidgetIter::new(pnl_vers).count());

        for wgt in WidgetIter::new(pnl_vers) {
            assert_eq!(pnl_vers.link.own_idx, wgt.link.parent_idx);
        }
    }

    // iterate over ... checkbox
    {
        let chbx = wgt_find_by_id(Id::ChbxEnbl.into(), &WND_TEST_ARRAY);
        assert!(chbx.is_some());
        assert_eq!(0, WidgetIter::new(chbx.unwrap()).count());
    }
}
