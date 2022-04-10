//! # RTWins Widget tests

extern crate rtwins;
use rtwins::colors::*;
use rtwins::wgt;
use rtwins::*;

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
                    children: &PAGE_VER_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageServ.into(),
                    prop: prop::Page {
                        title: "Service ∑",
                        fg_color: ColorFG::White,
                    }.into(),
                    children: &[
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


const WND_TEST_ARRAY: [Widget; rtwins::wgt::transform::tree_wgt_count(&WINDOW_TEST)] =
    rtwins::wgt::transform::tree_to_array(&WINDOW_TEST);

struct WndTestState {
    widgets: &'static [Widget],
    pub wnd_visible: bool,
    pub wnd_enabled: bool,
    pub lbl_about_visible: bool,
    pub lbl_about_enabled: bool,
}

impl WndTestState {
    fn new(widgets: &'static [Widget]) -> Self {
        WndTestState{widgets,
            wnd_visible: false,
            wnd_enabled: false,
            lbl_about_visible: false,
            lbl_about_enabled: false
        }
    }
}

impl WindowState for WndTestState {
    /** common state queries **/

    fn is_enabled(&mut self, wgt: &Widget) -> bool {
        if wgt.id == Id::WndTest.into() { self.wnd_enabled }
        else if wgt.id == Id::LabelAbout.into() { self.lbl_about_enabled }
        else { true }
    }

    fn is_focused(&mut self, _wgt: &Widget) -> bool {
        true
    }

    fn is_visible(&mut self, wgt: &Widget) -> bool {
        if wgt.id == Id::WndTest.into() { self.wnd_visible }
        else if wgt.id == Id::LabelAbout.into() { self.lbl_about_visible }
        else { true }
    }

    fn get_widgets(&self) -> &'static [Widget] {
        self.widgets
    }

    fn get_window_coord(&mut self) -> Coord {
        if let Some(ref w) = self.widgets.first() {
            w.coord
        }
        else {
            Coord::cdeflt()
        }
    }
}

// -----------------------------------------------------------------------------------------------

#[test]
fn rect_within() {
    let r = Rect::new(2, 2, 10, 5);
    assert!(r.is_rect_within(&Rect::new(2, 2, 10, 5)));
    assert!(r.is_rect_within(&Rect::new(3, 3, 8, 4)));
    assert!(!r.is_rect_within(&Rect::new(1, 2, 10, 5)));
    assert!(!r.is_rect_within(&Rect::new(2, 1, 10, 5)));
    assert!(!r.is_rect_within(&Rect::new(2, 2, 11, 5)));
    assert!(!r.is_rect_within(&Rect::new(2, 2, 10, 6)));
}

#[test]
fn point_within() {
    let r = Rect::new(2, 2, 10, 5);
    assert!(r.is_point_within(2, 2));
    assert!(r.is_point_within(2+10-1, 2));
    assert!(r.is_point_within(2+10-1, 2+5-1));
    assert!(!r.is_point_within(2+10, 2));
    assert!(!r.is_point_within(2+10-1, 2+5));
    assert!(!r.is_point_within(1, 2));
    assert!(!r.is_point_within(2, 1));
}

#[test]
fn get_parent() {
    // window parent is the window itself
    {
        let wgt = &WND_TEST_ARRAY[0];
        let par = wgt::get_parent(wgt);
        assert!(par.id == wgt.id);
    }

    // panel parent is the window
    {
        let wnd = &WND_TEST_ARRAY[0];
        let wgt = wgt::find_by_id(Id::PgControl.into(), &WND_TEST_ARRAY);
        assert!(wgt.is_some());
        let par = wgt::get_parent(wgt.unwrap());
        assert!(par.id == wnd.id);
    }

    // try to find invalid widget
    {
        let wgt = wgt::find_by_id(Id::NotExistingWgt.into(), &WND_TEST_ARRAY);
        assert!(wgt.is_none());
    }
}

#[test]
fn widget_iter() {
    // iterate over panel
    {
        let pnl_vers = wgt::find_by_id(Id::PanelVersions.into(), &WND_TEST_ARRAY);
        assert!(pnl_vers.is_some());
        let pnl_vers = pnl_vers.unwrap();

        assert_eq!(2, pnl_vers.iter_children().count());

        for wgt in pnl_vers.iter_children() {
            assert_eq!(pnl_vers.link.own_idx, wgt.link.parent_idx);
        }
    }

    // iterate over ... checkbox
    {
        let chbx = wgt::find_by_id(Id::ChbxEnbl.into(), &WND_TEST_ARRAY);
        assert!(chbx.is_some());
        assert_eq!(0, wgt::ChildrenIter::new(chbx.unwrap()).count());
    }
}

#[test]
fn screen_coord() {
    let lbl = wgt::find_by_id(Id::LabelAbout.into(), &WND_TEST_ARRAY);
    assert!(lbl.is_some());
    let coord = wgt::get_screen_coord(lbl.unwrap());
    assert_eq!(33, coord.col);
    assert_eq!(7, coord.row);
}

#[test]
fn is_visible() {
    let mut ws = WndTestState::new(&WND_TEST_ARRAY);
    let lbl = wgt::find_by_id(Id::LabelAbout.into(), &WND_TEST_ARRAY);
    assert!(lbl.is_some());

    assert!(!wgt::is_visible(&mut ws, lbl.unwrap()));
    ws.lbl_about_visible = true;
    assert!(!wgt::is_visible(&mut ws, lbl.unwrap()));
    ws.wnd_visible = true;
    assert!(wgt::is_visible(&mut ws, lbl.unwrap()));
}

#[test]
fn is_enabled() {
    let mut ws = WndTestState::new(&WND_TEST_ARRAY);
    let lbl = wgt::find_by_id(Id::LabelAbout.into(), &WND_TEST_ARRAY);
    assert!(lbl.is_some());

    assert!(!wgt::is_enabled(&mut ws, lbl.unwrap()));
    ws.lbl_about_enabled = true;
    assert!(!wgt::is_enabled(&mut ws, lbl.unwrap()));
    ws.wnd_enabled = true;
    assert!(wgt::is_enabled(&mut ws, lbl.unwrap()));
}

#[test]
fn widget_at() {
    let mut ws = WndTestState::new(&WND_TEST_ARRAY);
    ws.wnd_visible = true;
    let mut wgt_r = Rect::cdeflt();

    // point beyound main window
    assert!(wgt::find_at(&mut ws, 1, 1, &mut wgt_r).is_none());

    // origin of main window
    {
        let wnd_coord = ws.get_window_coord();
        let opt_w = wgt::find_at(&mut ws, wnd_coord.col, wnd_coord.row, &mut wgt_r);
        assert!(opt_w.is_some());
        if let Some(wgt) = opt_w {
            assert!(wgt.id == Id::WndTest.into());
            assert!(wgt::is_parent(wgt));
        }
    }

    // origin of button - this one is always visible
    {
        let btn = wgt::find_by_id(Id::BtnYes.into(), &WND_TEST_ARRAY);
        assert!(btn.is_some());
        let btn_coord = wgt::get_screen_coord(btn.unwrap());
        assert_eq!(60, btn_coord.col);
        assert_eq!(10, btn_coord.row);

        let opt_b = wgt::find_at(&mut ws, btn_coord.col+2, btn_coord.row, &mut wgt_r);
        assert!(opt_b.is_some());
        if let Some(wgt) = opt_b {
            assert!(wgt.id == Id::BtnYes.into());
            assert!(!wgt::is_parent(wgt));
        }
    }
}

#[test]
fn page_idx() {
    // correct Page 0
    {
        let page = wgt::find_by_id(Id::PageVer.into(), &WND_TEST_ARRAY);
        assert!(page.is_some());
        let idx = wgt::page_page_idx(&page.unwrap());
        assert!(idx.is_some());
        assert_eq!(0, idx.unwrap());
    }

    // correct Page 1
    {
        let page = wgt::find_by_id(Id::PageServ.into(), &WND_TEST_ARRAY);
        assert!(page.is_some());
        let idx = wgt::page_page_idx(&page.unwrap());
        assert!(idx.is_some());
        assert_eq!(1, idx.unwrap());
    }


    // wrong widget type
    {
        let btn = wgt::find_by_id(Id::BtnYes.into(), &WND_TEST_ARRAY);
        assert!(btn.is_some());
        let idx = wgt::page_page_idx(&btn.unwrap());
        assert!(idx.is_none());
    }
}

#[test]
fn page_wid() {
    // correct Page 1
    {
        let pgctrl = wgt::find_by_id(Id::PgControl.into(), &WND_TEST_ARRAY);
        assert!(pgctrl.is_some());
        let pgctrl = pgctrl.unwrap();
        let wid = wgt::pagectrl_page_wid(&pgctrl, 1);
        assert_eq!(Id::PageServ.into(), wid);
    }

    // incorrect Page 7
    {
        let pgctrl = wgt::find_by_id(Id::PgControl.into(), &WND_TEST_ARRAY);
        assert!(pgctrl.is_some());
        let pgctrl = pgctrl.unwrap();
        let wid = wgt::pagectrl_page_wid(&pgctrl,7);
        assert_eq!(WIDGET_ID_NONE, wid);
    }

    // wrong widget type
    {
        let btn = wgt::find_by_id(Id::BtnYes.into(), &WND_TEST_ARRAY);
        assert!(btn.is_some());
        let btn = btn.unwrap();
        let wid = wgt::pagectrl_page_wid(&btn, 0);
        assert_eq!(WIDGET_ID_NONE, wid);
    }
}
