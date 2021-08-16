use std::io::stdout;

use crossterm::{cursor, style, terminal, ExecutableCommand, Result};

/// Prints a string in the middle of the terminal output
fn main() -> Result<()> {
    // get terminal size
    let (terminal_width, terminal_height) = terminal::size()?;
    // string to print out
    let string_to_print = "this string is in the middle of the terminal";
    // define coordinates of where the string should be printed
    let move_to_x = (terminal_width / 2) - (string_to_print.len() as u16 / 2);
    let move_to_y = terminal_height / 2;

    // print string in the middle of the terminal
    stdout()
        .execute(terminal::Clear(terminal::ClearType::All))? // clear all other output in the terminal
        .execute(cursor::MoveTo(move_to_x, move_to_y))? // move cursor to the middle of the terminal
        .execute(style::SetForegroundColor(style::Color::Black))? // set the color of the text
        .execute(style::SetBackgroundColor(style::Color::Red))? // set the color of the background of the text
        .execute(style::Print(string_to_print))? // print the string
        .execute(cursor::MoveTo(terminal_width, terminal_height))? // move cursor to the end of the terminal
        .execute(style::ResetColor)?; // reset colors back to the default

    Ok(())
}
