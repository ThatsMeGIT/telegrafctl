//use crate::file_operations::read_file;
use crate::ui::build_ui;
//use std::io::stdin;

mod file_operations;
mod ui;
fn main() {
    /*
        let mut input = String::new();

        println!("Bitte geben Sie den Pfad zur Datei an:");

        // User Input
        stdin().read_line(&mut input).unwrap();
        let filepath = input.trim().to_string();
        //###### DEBUG println!("{}", filepath);

        // Read file
        read_file(&filepath);
    */
    build_ui();
}
