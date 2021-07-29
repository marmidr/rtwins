//! RTwins demo app

extern crate rtwins;
use rtwins::esc;

/// Simple widget-based interface definition as const
mod tui
{
use rtwins::{wp, Coord, Size, Type, Widget};
use rtwins::colors::{ColorBG, ColorFG};
use rtwins::widget::WIDGET_ID_NONE;

#[allow(dead_code)]
pub enum Id
{
    WndMain = rtwins::WIDGET_ID_NONE as isize + 1,
        Lbl1,
        Lbl2,
        PnlGreen,
            BtnOk,
            BtnCancel,
        PnlWhite,
}

/// Easy conversion from enum to i16
impl Id {
    const fn into(self) -> u16 { self as u16 }
}

pub const NO_CHILDS: [Widget; 0] = [];

pub const WINDOW: Widget = Widget {
    id : Id::WndMain.into(),
    parent: WIDGET_ID_NONE,
    coord: Coord{col: 1, row: 2},
    size: Size{width: 25, height: 12},
    typ: wp::Window {
        title   : "** DEMO **",
        fg_color: ColorFG::White,
        bg_color: ColorBG::Blue,
        is_popup: false,
    }.into(),
    // link: &[]
    link: &[
        Widget {
            id: Id::Lbl1.into(),
            parent: rtwins::WIDGET_ID_NONE,
            coord: Coord::cdeflt(),
            size: Size::cdeflt(),
            typ: Type::None,
            link: &NO_CHILDS
        },
        Widget {
            id: 2,
            parent: rtwins::WIDGET_ID_NONE,
            coord: Coord::cdeflt(),
            size: Size::cdeflt(),
            typ: Type::None,
            link: &[]
        },
    ]
};

}

// -----------------------------------------------------------------------------------------------

fn main()
{
    rtwins::init();
    println!("RTWins demo; lib v{}", rtwins::VER);
    println!("Normal {}Bold{} {}Italic{}", esc::BOLD, esc::NORMAL, esc::ITALICS_ON, esc::ITALICS_OFF);

    let w_none = rtwins::Widget {
        id      : 0,
        parent  : rtwins::WIDGET_ID_NONE,
        coord   : rtwins::Coord::cdeflt(),
        size    : rtwins::Size::cdeflt(),
        typ     : rtwins::Type::None,
        link    : &tui::NO_CHILDS
    };

    println!("w_none childs: {}", w_none.link.len() );
    println!("WINDOW childs: {}", tui::WINDOW.link.len() );

    let title = |wgt: &rtwins::Widget| match wgt.typ {
        rtwins::Type::Window(ref wp) => wp.title,
        _                            => "<?>"
    };

    println!("WINDOW title: {}", title(&tui::WINDOW) );
    println!("WINDOW title: {}", wnd_prop(&tui::WINDOW).title );
    println!("WINDOW title: {}", tui::WINDOW.typ.prop_wnd().title );
    println!("sizeof Widget: {}", std::mem::size_of::<rtwins::widget::Widget>());
    println!("sizeof Type: {}", std::mem::size_of::<rtwins::widget::Type>());
    println!("sizeof Id: {}", std::mem::size_of::<tui::Id>());

    if let rtwins::Type::Window(ref wp) = tui::WINDOW.typ {
        println!("WINDOW title: {}", wp.title );
    }
}

/// Extract window properties from enum
fn wnd_prop<'a>(wgt: &'a rtwins::Widget) -> &'a rtwins::widget::wp::Window {
    match wgt.typ {
        rtwins::Type::Window(ref wp) => wp,
        _ => panic!()
    }
}
