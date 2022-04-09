//! # RTWins ESC tests

extern crate rtwins;

use rtwins::esc::*;

fn count_esc(s: &str) -> u32
{
    let n_esc = s.chars().fold(
        0u32, |acc, item| if item == '\x1B' { acc + 1 } else { acc }
    );
    n_esc
}

#[test]
fn esc_macros() {
    assert_eq!("\x1b[{0}G", rtwins::cursor_column!());
    assert_eq!("\x1b[{0}G", CURSOR_COLUMN_FMT);
    assert_eq!("\x1b[42G", rtwins::cursor_column!(42));

    assert_eq!("\x1b[48;5;123m", rtwins::bg_color!(123));
    assert_eq!("\x1b[48;2;255;192;203m", BG_PINK);

    assert_eq!("\x1b[38;5;213m", rtwins::fg_color!(213));
    assert_eq!("\x1b[38;2;220;20;60m", FG_CRIMSON);
}

#[test]
fn colors_attributes() {
    let s = String::new()
        + BOLD
        + FAINT
        + NORMAL
        + ITALICS_ON
        + ITALICS_OFF
        + UNDERLINE_ON
        + UNDERLINE_OFF
        + BLINK
        + BLINK_OFF
        + INVERSE_ON
        + INVERSE_OFF
        + INVISIBLE_ON
        + INVISIBLE_OFF
        + STRIKETHROUGH_ON
        + STRIKETHROUGH_OFF
        + ATTRIBUTES_DEFAULT;

    assert_eq!(count_esc(&s), 16);
}

/// Check if all colors are available
#[test]
fn colors_fg() {
    let s = String::new()
        + FG_BLACK
        + FG_BLACK_INTENSE
        + FG_RED
        + FG_RED_INTENSE
        + FG_GREEN
        + FG_GREEN_INTENSE
        + FG_YELLOW
        + FG_YELLOW_INTENSE
        + FG_BLUE
        + FG_BLUE_INTENSE
        + FG_MAGENTA
        + FG_MAGENTA_INTENSE
        + FG_CYAN
        + FG_CYAN_INTENSE
        + FG_WHITE
        + FG_WHITE_INTENSE
        + FG_DEFAULT
        + FG_PINK
        + FG_LIGHT_PINK
        + FG_HOT_PINK
        + FG_DEEP_PINK
        + FG_PALE_VIOLET_RED
        + FG_MEDIUM_VIOLET_RED
        + FG_LIGHT_SALMON
        + FG_SALMON
        + FG_DARK_SALMON
        + FG_LIGHT_CORAL
        + FG_INDIAN_RED
        + FG_CRIMSON
        + FG_FIREBRICK
        + FG_DARK_RED
        + FG_RED_RGB
        + FG_ORANGE_RED
        + FG_TOMATO
        + FG_CORAL
        + FG_DARK_ORANGE
        + FG_ORANGE
        + FG_YELLOW_RGB
        + FG_LIGHT_YELLOW
        + FG_LEMON_CHIFFON
        + FG_LIGHT_GOLDENROD_YELLOW
        + FG_PAPAYA_WHIP
        + FG_MOCCASIN
        + FG_PEACH_PUFF
        + FG_PALE_GOLDENROD
        + FG_KHAKI
        + FG_DARK_KHAKI
        + FG_GOLD
        + FG_CORNSILK
        + FG_BLANCHED_ALMOND
        + FG_BISQUE
        + FG_NAVAJO_WHITE
        + FG_WHEAT
        + FG_BURLYWOOD
        + FG_TAN
        + FG_ROSY_BROWN
        + FG_SANDY_BROWN
        + FG_GOLDENROD
        + FG_DARK_GOLDENROD
        + FG_PERU
        + FG_CHOCOLATE
        + FG_SADDLE_BROWN
        + FG_SIENNA
        + FG_BROWN
        + FG_MAROON
        + FG_DARK_OLIVE_GREEN
        + FG_OLIVE
        + FG_OLIVE_DRAB
        + FG_YELLOW_GREEN
        + FG_LIME_GREEN
        + FG_LIME
        + FG_LAWN_GREEN
        + FG_CHARTREUSE
        + FG_GREEN_YELLOW
        + FG_SPRING_GREEN
        + FG_MEDIUM_SPRING_GREEN
        + FG_LIGHT_GREEN
        + FG_PALE_GREEN
        + FG_DARK_SEA_GREEN
        + FG_MEDIUM_AQUAMARINE
        + FG_MEDIUM_SEA_GREEN
        + FG_SEA_GREEN
        + FG_FOREST_GREEN
        + FG_GREEN_RGB
        + FG_DARK_GREEN
        + FG_AQUA
        + FG_CYAN_RGB
        + FG_LIGHT_CYAN
        + FG_PALE_TURQUOISE
        + FG_AQUAMARINE
        + FG_TURQUOISE
        + FG_MEDIUM_TURQUOISE
        + FG_DARK_TURQUOISE
        + FG_LIGHT_SEA_GREEN
        + FG_CADET_BLUE
        + FG_DARK_CYAN
        + FG_TEAL
        + FG_LIGHT_STEEL_BLUE
        + FG_POWDER_BLUE
        + FG_LIGHT_BLUE
        + FG_SKY_BLUE
        + FG_LIGHT_SKY_BLUE
        + FG_DEEP_SKY_BLUE
        + FG_DODGER_BLUE
        + FG_CORNFLOWER_BLUE
        + FG_STEEL_BLUE
        + FG_ROYAL_BLUE
        + FG_BLUE_RGB
        + FG_MEDIUM_BLUE
        + FG_DARK_BLUE
        + FG_NAVY
        + FG_MIDNIGHT_BLUE
        + FG_LAVENDER
        + FG_THISTLE
        + FG_PLUM
        + FG_VIOLET
        + FG_ORCHID
        + FG_FUCHSIA
        + FG_MAGENTA_RGB
        + FG_MEDIUM_ORCHID
        + FG_MEDIUM_PURPLE
        + FG_BLUE_VIOLET
        + FG_DARK_VIOLET
        + FG_DARK_ORCHID
        + FG_DARK_MAGENTA
        + FG_PURPLE
        + FG_INDIGO
        + FG_DARK_SLATE_BLUE
        + FG_SLATE_BLUE
        + FG_MEDIUM_SLATE_BLUE
        + FG_GAINSBORO
        + FG_LIGHT_GRAY
        + FG_SILVER
        + FG_DARK_GRAY
        + FG_GRAY
        + FG_DIM_GRAY
        + FG_LIGHT_SLATE_GRAY
        + FG_SLATE_GRAY
        + FG_DARK_SLATE_GRAY
        + FG_BLACK_RGB;

    assert_eq!(count_esc(&s), 140);
}

/// Check if all constants are available
#[test]
fn colors_bg() {
    let s = String::new()
        + BG_BLACK
        + BG_BLACK_INTENSE
        + BG_RED
        + BG_RED_INTENSE
        + BG_GREEN
        + BG_GREEN_INTENSE
        + BG_YELLOW
        + BG_YELLOW_INTENSE
        + BG_BLUE
        + BG_BLUE_INTENSE
        + BG_MAGENTA
        + BG_MAGENTA_INTENSE
        + BG_CYAN
        + BG_CYAN_INTENSE
        + BG_WHITE
        + BG_WHITE_INTENSE
        + BG_DEFAULT
        + BG_PINK
        + BG_LIGHT_PINK
        + BG_HOT_PINK
        + BG_DEEP_PINK
        + BG_PALE_VIOLET_RED
        + BG_MEDIUM_VIOLET_RED
        + BG_LIGHT_SALMON
        + BG_SALMON
        + BG_DARK_SALMON
        + BG_LIGHT_CORAL
        + BG_INDIAN_RED
        + BG_CRIMSON
        + BG_FIREBRICK
        + BG_DARK_RED
        + BG_RED_RGB
        + BG_ORANGE_RED
        + BG_TOMATO
        + BG_CORAL
        + BG_DARK_ORANGE
        + BG_ORANGE
        + BG_YELLOW_RGB
        + BG_LIGHT_YELLOW
        + BG_LEMON_CHIFFON
        + BG_LIGHT_GOLDENROD_YELLOW
        + BG_PAPAYA_WHIP
        + BG_MOCCASIN
        + BG_PEACH_PUFF
        + BG_PALE_GOLDENROD
        + BG_KHAKI
        + BG_DARK_KHAKI
        + BG_GOLD
        + BG_CORNSILK
        + BG_BLANCHED_ALMOND
        + BG_BISQUE
        + BG_NAVAJO_WHITE
        + BG_WHEAT
        + BG_BURLYWOOD
        + BG_TAN
        + BG_ROSY_BROWN
        + BG_SANDY_BROWN
        + BG_GOLDENROD
        + BG_DARK_GOLDENROD
        + BG_PERU
        + BG_CHOCOLATE
        + BG_SADDLE_BROWN
        + BG_SIENNA
        + BG_BROWN
        + BG_MAROON
        + BG_DARK_OLIVE_GREEN
        + BG_OLIVE
        + BG_OLIVE_DRAB
        + BG_YELLOW_GREEN
        + BG_LIME_GREEN
        + BG_LIME
        + BG_LAWN_GREEN
        + BG_CHARTREUSE
        + BG_GREEN_YELLOW
        + BG_SPRING_GREEN
        + BG_MEDIUM_SPRING_GREEN
        + BG_LIGHT_GREEN
        + BG_PALE_GREEN
        + BG_DARK_SEA_GREEN
        + BG_MEDIUM_AQUAMARINE
        + BG_MEDIUM_SEA_GREEN
        + BG_SEA_GREEN
        + BG_FOREST_GREEN
        + BG_GREEN_RGB
        + BG_DARK_GREEN
        + BG_AQUA
        + BG_CYAN_RGB
        + BG_LIGHT_CYAN
        + BG_PALE_TURQUOISE
        + BG_AQUAMARINE
        + BG_TURQUOISE
        + BG_MEDIUM_TURQUOISE
        + BG_DARK_TURQUOISE
        + BG_LIGHT_SEA_GREEN
        + BG_CADET_BLUE
        + BG_DARK_CYAN
        + BG_TEAL
        + BG_LIGHT_STEEL_BLUE
        + BG_POWDER_BLUE
        + BG_LIGHT_BLUE
        + BG_SKY_BLUE
        + BG_LIGHT_SKY_BLUE
        + BG_DEEP_SKY_BLUE
        + BG_DODGER_BLUE
        + BG_CORNFLOWER_BLUE
        + BG_STEEL_BLUE
        + BG_ROYAL_BLUE
        + BG_BLUE_RGB
        + BG_MEDIUM_BLUE
        + BG_DARK_BLUE
        + BG_NAVY
        + BG_MIDNIGHT_BLUE
        + BG_LAVENDER
        + BG_THISTLE
        + BG_PLUM
        + BG_VIOLET
        + BG_ORCHID
        + BG_FUCHSIA
        + BG_MAGENTA_RGB
        + BG_MEDIUM_ORCHID
        + BG_MEDIUM_PURPLE
        + BG_BLUE_VIOLET
        + BG_DARK_VIOLET
        + BG_DARK_ORCHID
        + BG_DARK_MAGENTA
        + BG_PURPLE
        + BG_INDIGO
        + BG_DARK_SLATE_BLUE
        + BG_SLATE_BLUE
        + BG_MEDIUM_SLATE_BLUE
        + BG_GAINSBORO
        + BG_LIGHT_GRAY
        + BG_SILVER
        + BG_DARK_GRAY
        + BG_GRAY
        + BG_DIM_GRAY
        + BG_LIGHT_SLATE_GRAY
        + BG_SLATE_GRAY
        + BG_DARK_SLATE_GRAY
        + BG_BLACK_RGB;

    assert_eq!(count_esc(&s), 140);
}

/// Check cursor navigation
#[test]
fn cursor_move() {
    let s = String::new()
        + rtwins::cursor_backward!(1)
        + rtwins::cursor_forward!(1)
        + rtwins::cursor_up!(1)
        + rtwins::cursor_down!(1)
        + rtwins::cursor_column!(45)
        + rtwins::cursor_goto!(3, 14);

    assert_eq!(count_esc(&s), 6);
}

/// Check lines manipulation
#[test]
fn line_insert_delete() {
    let s = String::new()
        + rtwins::line_insert!(1)
        + rtwins::line_delete!(1);

    assert_eq!(count_esc(&s), 2);
}

/// Check character manipulation
#[test]
fn character_erase_insert() {
    let s = String::new()
        + rtwins::char_repeat_last!(1)
        + rtwins::char_erase!(1)
        + rtwins::char_delete!(1)
        + rtwins::char_insert!(1);

    assert_eq!(count_esc(&s), 4);
}

/// Check screen manipulation
#[test]
fn screen_scroll() {
    let s = String::new()
        + rtwins::screen_scroll_up!(1)
        + rtwins::screen_scroll_down!(1);

    assert_eq!(count_esc(&s), 2);
}

#[test]
fn link_url() {
    let s = String::new() + rtwins::url_link!("https://github.com/marmidr/rtwins", "RTWins");

    assert!(s.len() > 0);
}
