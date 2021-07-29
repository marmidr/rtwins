//! RTwins demo app

extern crate rtwins;
use rtwins::esc;

const _WINDOW: rtwins::Widget = rtwins::Widget {
    id: 0,
    coord: rtwins::Coord::dflt(),
    size: rtwins::Size::dflt(),
    typ: rtwins::Type::None,
};

fn main() {
    rtwins::init();
    println!("Normal {}Bold{} {}Italic{}", esc::BOLD, esc::NORMAL, esc::ITALICS_ON, esc::ITALICS_OFF);

    let mut _w = rtwins::Widget {
        id: 0,
        coord: rtwins::Coord::dflt(),
        size: rtwins::Size::dflt(),
        typ: rtwins::Type::None,
    };

}
