use std::io::stdin;
use crate::file_operations::read_file;
/*
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};*/

mod file_operations;
fn main() {
    let mut input = String::new();

    println!("Bitte geben Sie den Pfad zur Datei an:");

    // User Input
    stdin().read_line(&mut input).unwrap();
    let filepath = input.trim().to_string();
    println!("{}", filepath);

    // Read file
    read_file(&filepath);


}
