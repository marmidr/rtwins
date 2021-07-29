//! RTwins demo app

extern crate rtwins;
use rtwins::esc;

/// Simple widget-based interface definition as const
mod tui
{
use rtwins::*;
use rtwins::colors::{ColorBG, ColorFG};

pub const NO_CHILDS: [Widget;0] = [];

pub const WINDOW: Widget = Widget {
    id: 0,
    coord: Coord{col: 1, row: 2},
    size: Size{width: 25, height: 12},
    typ: Type::Window {
        title   : "** DEMO **",
        fg_color: ColorFG::White,
        bg_color: ColorBG::Blue,
        is_popup: false,
    },
    link: &[
        Widget {
            id: 1,
            coord: Coord::cdeflt(),
            size: Size::cdeflt(),
            typ: Type::None,
            link: &NO_CHILDS
        },
        Widget {
            id: 2,
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
    println!("Normal {}Bold{} {}Italic{}", esc::BOLD, esc::NORMAL, esc::ITALICS_ON, esc::ITALICS_OFF);

    let w_none = rtwins::Widget {
        id: 0,
        coord: rtwins::Coord::cdeflt(),
        size: rtwins::Size::cdeflt(),
        typ: rtwins::Type::None,
        link: &tui::NO_CHILDS
    };

    println!("w_none childs: {}", w_none.link.len() );
    println!("WINDOW childs: {}", tui::WINDOW.link.len() );

    let title = |w: &rtwins::Widget| match w.typ {
        rtwins::Type::Window{ title, .. } => title,
        _                                 => "<?>"
    };

    println!("WINDOW title: {}", title(&tui::WINDOW) );
}
