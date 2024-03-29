//! # RTWins Widget tests

extern crate rtwins;
use rtwins::colors::*;
use rtwins::common::*;
use rtwins::wgt::{self, *};

#[rustfmt::skip]
#[repr(u16)]
#[derive(Clone, Copy)]
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

// copied from tui_state.rs
impl Id {
    #[inline]
    pub const fn into(self) -> WId {
        self as WId
    }
}

// copied from tui_state.rs
impl std::cmp::PartialEq<Id> for WId {
    #[inline]
    fn eq(&self, other: &Id) -> bool {
        *self == *other as WId
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
            fg_color: ColorFg::White,
            bg_color: ColorBg::Green,
            no_frame: false,
        }.into(),
        children: &[
            Widget {
                id: Id::LabelFwVersion.into(),
                coord: Coord { col: 2, row: 1 },
                prop: prop::Label {
                    title: "FwVer: 1.1",
                    fg_color: ColorFg::YellowIntense,
                    bg_color: ColorBg::Inherit,
                }.into(),
                ..Widget::cdeflt()
            },
            Widget {
                id: Id::LabelAbout.into(),
                coord: Coord { col: 2, row: 3 },
                size: Size { width: 0, height: 1 },
                prop: prop::Label {
                    title: "",
                    fg_color: ColorFg::Blue,
                    bg_color: ColorBg::Inherit,
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
            fg_color: ColorFg::White,
            bg_color: ColorBg::Green,
            style: ButtonStyle::Solid
        }.into(),
        ..Widget::cdeflt()
    },
    Widget {
        id: Id::BtnNo.into(),
        coord: Coord { col: 38, row: 7 },
        prop: prop::Button {
            text: "NO",
            fg_color: ColorFg::White,
            bg_color: ColorBg::Red,
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
        fg_color: ColorFg::White,
        bg_color: ColorBg::Blue,
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
                    children: &PAGE_VER_CHILDREN,
                    ..Widget::cdeflt()
                },
                Widget {
                    id: Id::PageServ.into(),
                    prop: prop::Page {
                        title: "Service ∑",
                        fg_color: ColorFg::White,
                    }.into(),
                    children: &[
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
                    ],
                    ..Widget::cdeflt()
                },
            ],
            ..Widget::cdeflt()
        },
    ]
};

const WND_TEST_WGTS: [Widget; rtwins::wgt::transform::tree_wgt_count(&WINDOW_TEST)] =
    rtwins::wgt::transform::tree_to_array(&WINDOW_TEST);

struct WndTestState {
    widgets: &'static [Widget],
    pub wnd_visible: bool,
    pub wnd_enabled: bool,
    pub lbl_about_visible: bool,
    pub lbl_about_enabled: bool,
    pub pgctrl_page_idx: i16,
}

impl WndTestState {
    fn new(widgets: &'static [Widget]) -> Self {
        WndTestState {
            widgets,
            wnd_visible: false,
            wnd_enabled: false,
            lbl_about_visible: false,
            lbl_about_enabled: false,
            pgctrl_page_idx: 0,
        }
    }
}

impl WindowState for WndTestState {
    /** common state queries **/

    fn is_enabled(&self, wgt: &Widget) -> bool {
        if wgt.id == Id::WndTest {
            self.wnd_enabled
        }
        else if wgt.id == Id::LabelAbout {
            self.lbl_about_enabled
        }
        else {
            true
        }
    }

    fn is_focused(&self, _wgt: &Widget) -> bool {
        true
    }

    fn is_visible(&self, wgt: &Widget) -> bool {
        if wgt.id == Id::WndTest {
            self.wnd_visible
        }
        else if wgt.id == Id::LabelAbout {
            self.lbl_about_visible
        }
        else {
            true
        }
    }

    fn get_widgets(&self) -> &'static [Widget] {
        self.widgets
    }

    fn get_window_coord(&mut self) -> Coord {
        if let Some(ref w) = self.widgets.first() {
            // return window coord
            w.coord
        }
        else {
            Coord::cdeflt()
        }
    }

    fn on_page_control_page_change(&mut self, _wgt: &Widget, new_page_idx: i16) {
        self.pgctrl_page_idx = new_page_idx;
    }

    fn get_page_ctrl_page_index(&mut self, _wgt: &Widget) -> i16 {
        self.pgctrl_page_idx
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
    assert!(r.is_point_within(2 + 10 - 1, 2));
    assert!(r.is_point_within(2 + 10 - 1, 2 + 5 - 1));
    assert!(!r.is_point_within(2 + 10, 2));
    assert!(!r.is_point_within(2 + 10 - 1, 2 + 5));
    assert!(!r.is_point_within(1, 2));
    assert!(!r.is_point_within(2, 1));
}

#[test]
fn get_parent() {
    // window parent is the window itself
    {
        let wgt = &WND_TEST_WGTS[0];
        let par = wgt::get_parent(wgt);
        assert!(par.id == wgt.id);
    }

    // panel parent is the window
    {
        let wnd = &WND_TEST_WGTS[0];
        let wgt = wgt::find_by_id(&WND_TEST_WGTS, Id::PgControl.into());
        assert!(wgt.is_some());
        let par = wgt::get_parent(wgt.unwrap());
        assert!(par.id == wnd.id);
    }

    // try to find invalid widget
    {
        let wgt = wgt::find_by_id(&WND_TEST_WGTS, Id::NotExistingWgt.into());
        assert!(wgt.is_none());
    }
}

#[test]
fn widget_iter() {
    // iterate over panel
    {
        let pnl_vers = wgt::find_by_id(&WND_TEST_WGTS, Id::PanelVersions.into());
        assert!(pnl_vers.is_some());
        let pnl_vers = pnl_vers.unwrap();

        assert_eq!(2, pnl_vers.iter_children().count());

        for wgt in pnl_vers.iter_children() {
            assert_eq!(pnl_vers.link.own_idx, wgt.link.parent_idx);
        }
    }

    // iterate over ... checkbox
    {
        let chbx = wgt::find_by_id(&WND_TEST_WGTS, Id::ChbxEnbl.into());
        assert!(chbx.is_some());
        assert_eq!(0, wgt::ChildrenIter::new(chbx.unwrap()).count());
    }
}

#[test]
fn screen_coord() {
    let mut ws = WndTestState::new(&WND_TEST_WGTS);
    let lbl = wgt::find_by_id(&WND_TEST_WGTS, Id::LabelAbout.into());
    assert!(lbl.is_some());
    let coord = wgt::get_screen_coord(&mut ws, lbl.unwrap());
    assert_eq!(33, coord.col);
    assert_eq!(7, coord.row);
}

#[test]
fn is_visible() {
    let mut ws = WndTestState::new(&WND_TEST_WGTS);
    let lbl = wgt::find_by_id(&WND_TEST_WGTS, Id::LabelAbout.into());
    assert!(lbl.is_some());

    assert!(!wgt::is_visible(&mut ws, lbl.unwrap()));
    ws.lbl_about_visible = true;
    assert!(!wgt::is_visible(&mut ws, lbl.unwrap()));
    ws.wnd_visible = true;
    assert!(wgt::is_visible(&mut ws, lbl.unwrap()));
}

#[test]
fn is_enabled() {
    let mut ws = WndTestState::new(&WND_TEST_WGTS);
    let lbl = wgt::find_by_id(&WND_TEST_WGTS, Id::LabelAbout.into());
    assert!(lbl.is_some());

    assert!(!wgt::is_enabled(&mut ws, lbl.unwrap()));
    ws.lbl_about_enabled = true;
    assert!(!wgt::is_enabled(&mut ws, lbl.unwrap()));
    ws.wnd_enabled = true;
    assert!(wgt::is_enabled(&mut ws, lbl.unwrap()));
}

#[test]
fn widget_at() {
    let mut ws = WndTestState::new(&WND_TEST_WGTS);
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
            assert!(wgt.id == Id::WndTest);
            assert!(wgt::is_parent(wgt));
        }
    }

    // origin of button - this one is always visible
    {
        let btn = wgt::find_by_id(&WND_TEST_WGTS, Id::BtnYes.into());
        assert!(btn.is_some());
        let btn_coord = wgt::get_screen_coord(&mut ws, btn.unwrap());
        assert_eq!(60, btn_coord.col);
        assert_eq!(10, btn_coord.row);

        let opt_b = wgt::find_at(&mut ws, btn_coord.col + 2, btn_coord.row, &mut wgt_r);
        assert!(opt_b.is_some());
        if let Some(wgt) = opt_b {
            assert!(wgt.id == Id::BtnYes);
            assert!(!wgt::is_parent(wgt));
        }
    }
}

#[test]
fn page_idx() {
    // correct Page 0
    {
        let page = wgt::find_by_id(&WND_TEST_WGTS, Id::PageVer.into());
        assert!(page.is_some());
        let idx = wgt::page_page_idx(&page.unwrap());
        assert!(idx.is_some());
        assert_eq!(0, idx.unwrap());
    }

    // correct Page 1
    {
        let page = wgt::find_by_id(&WND_TEST_WGTS, Id::PageServ.into());
        assert!(page.is_some());
        let idx = wgt::page_page_idx(&page.unwrap());
        assert!(idx.is_some());
        assert_eq!(1, idx.unwrap());
    }

    // wrong widget type
    {
        let btn = wgt::find_by_id(&WND_TEST_WGTS, Id::BtnYes.into());
        assert!(btn.is_some());
        let idx = wgt::page_page_idx(&btn.unwrap());
        assert!(idx.is_none());
    }
}

#[test]
fn page_wid() {
    // correct Page 1
    {
        let pgctrl = wgt::find_by_id(&WND_TEST_WGTS, Id::PgControl.into());
        assert!(pgctrl.is_some());
        let pgctrl = pgctrl.unwrap();
        let wid = wgt::pagectrl_page_wid(&pgctrl, 1);
        assert_eq!(Id::PageServ.into(), wid);
    }

    // incorrect Page 7
    {
        let pgctrl = wgt::find_by_id(&WND_TEST_WGTS, Id::PgControl.into());
        assert!(pgctrl.is_some());
        let pgctrl = pgctrl.unwrap();
        let wid = wgt::pagectrl_page_wid(&pgctrl, 7);
        assert_eq!(WIDGET_ID_NONE, wid);
    }

    // wrong widget type
    {
        let btn = wgt::find_by_id(&WND_TEST_WGTS, Id::BtnYes.into());
        assert!(btn.is_some());
        let btn = btn.unwrap();
        let wid = wgt::pagectrl_page_wid(&btn, 0);
        assert_eq!(WIDGET_ID_NONE, wid);
    }
}

#[test]
fn pagectrl_select_next() {
    let mut ws = WndTestState::new(&WND_TEST_WGTS);
    let pgctrl = wgt::find_by_id(&WND_TEST_WGTS, Id::PgControl.into());
    assert!(pgctrl.is_some());
    let pgctrl = pgctrl.unwrap();

    // initial
    assert_eq!(0, ws.get_page_ctrl_page_index(&pgctrl));
    // next
    wgt::pagectrl_select_next_page(&mut ws, Id::PgControl.into(), true);
    assert_eq!(1, ws.get_page_ctrl_page_index(&pgctrl));
    // prev; this should wrap to 0
    wgt::pagectrl_select_next_page(&mut ws, Id::PgControl.into(), true);
    assert_eq!(0, ws.get_page_ctrl_page_index(&pgctrl));
    // prev; this should wrap back to 1
    wgt::pagectrl_select_next_page(&mut ws, Id::PgControl.into(), false);
    assert_eq!(1, ws.get_page_ctrl_page_index(&pgctrl));
    // prev again
    wgt::pagectrl_select_next_page(&mut ws, Id::PgControl.into(), false);
    assert_eq!(0, ws.get_page_ctrl_page_index(&pgctrl));
}

#[test]
fn pagectrl_select_page() {
    let mut ws = WndTestState::new(&WND_TEST_WGTS);
    let pgctrl = wgt::find_by_id(&WND_TEST_WGTS, Id::PgControl.into());
    assert!(pgctrl.is_some());
    let pgctrl = pgctrl.unwrap();

    // initial
    assert_eq!(0, ws.get_page_ctrl_page_index(&pgctrl));
    wgt::pagectrl_select_page(&mut ws, Id::PgControl.into(), Id::PageVer.into());
    assert_eq!(0, ws.get_page_ctrl_page_index(&pgctrl));
    wgt::pagectrl_select_page(&mut ws, Id::PgControl.into(), Id::PageServ.into());
    assert_eq!(1, ws.get_page_ctrl_page_index(&pgctrl));
}
