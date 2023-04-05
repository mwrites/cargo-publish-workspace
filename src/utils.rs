#![allow(dead_code)]

use super::*;
use std::io::Write;
use termcolor::{
    Color,
    ColorChoice,
    ColorSpec,
    StandardStream,
    WriteColor,
};

#[macro_export]
macro_rules! print_status {
    ($action:expr, $message:expr) => {
        print_style($action, $message, termcolor::Color::Green, true)
    };
}

#[macro_export]
macro_rules! print_note {
    ($message:expr) => {
        print_style("note", $message, termcolor::Color::Cyan, false)
    };
}

#[macro_export]
macro_rules! print_warn {
    ($message:expr) => {
        print_style("warning", $message, termcolor::Color::Yellow, false)
    };
}

#[macro_export]
macro_rules! print_error {
    ($message:expr) => {
        print_style("ERROR", $message, termcolor::Color::Red, false)
    };
}

/// Print a message with a colored title in the style of Cargo shell messages.
pub fn print_style(status: &str, message: &str, color: Color, justified: bool) {
    let mut output = StandardStream::stderr(ColorChoice::Auto);
    output
        .set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))
        .unwrap();
    if justified {
        write!(output, "{status:>12}").unwrap();
    } else {
        write!(output, "{status}").unwrap();
        output.set_color(ColorSpec::new().set_bold(true)).unwrap();
        write!(output, ":").unwrap();
    }
    output.reset().unwrap();
    writeln!(output, " {message}").unwrap();
}
