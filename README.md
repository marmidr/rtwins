# RTWins in brief

`RTWins` is a Rust library designed for easy creation of visual terminal applications on non-os platforms, like bare Cortex-M3.
It provides basic facilities required by interactive applications such as screen and cursor management, keyboard input, keymaps, color codes.

## References

Implementation is based on:

* https://github.com/marmidr/twins
* [Wiki: reference color tables for different terminals](https://en.m.wikipedia.org/wiki/ANSI_escape_code)

## Primary goals

- text properties
    - [ ] foreground and background color codes
    - [ ] attributes (bold, inversion)
- operations
    - [ ] clear screen
    - [ ] go to home
    - [ ] go to location
- reading input
    - [ ] regular characters (a..z)
    - [ ] control codes (Up/Down, Del, Ctrl, Home, ...)
- [ ] buffered terminal output
- [ ] platform abstraction layer (PAL) to ease porting
- [ ] make it compile in clang
- [ ] command line interface with history (CLI)

## Secondary goals

- widgets (controls) to implement
    - [ ] window
    - [ ] panel
    - [ ] static label / led
    - [ ] check box
    - [ ] edit field (text/number)
    - [ ] radio button
    - [ ] page control
    - [ ] progress bar
    - [ ] list box
    - [ ] combo box
    - [ ] scrollable text box
    - [ ] custom widget base
    - [ ] scrollbar
    - [ ] horizontal page control
    - [ ] popup windows
    - [ ] layers - to control visibility of groups of widgets
    - [ ] password input
- navigation
    - [ ] widgets navigation by Tab/Esc key
    - [ ] render focused widget state
    - [ ] blinking cursor instead of inversed colors
    - [ ] select widget by mouse
- notifications
    - [ ] notify event per widget type (button clicked, checkbox toggled)
- [ ] color theme for window
- [ ] keyboard handler returns if key was handled by active widget
- [ ] support for mouse click
- [ ] double-width character support (emoticons 😁)
- [ ] multiline solid button


# Prerequisites


# How to build demo


### Run GUI demo:


## How to build unit tests


### Build and run the tests

