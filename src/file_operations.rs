use std::fs::File;
use std::io::{BufReader};
/*
pub struct config {
    global: global,
    agent: agent,
    plugins: plugins
}

pub struct global {}

pub struct agent {}

pub struct plugins {}
*/
pub fn read_file(mut filepath: &str) {
    let counter: i32 = 0;

    // String säubern
    filepath = filepath.trim_matches('"');

    // Datei öffnen
    let file = File::open(filepath).expect("Datei wurde nicht gefunden");

    // Buffered Reader erstellen
    let _reader = BufReader::new(file);

    // Zeile für Zeile Inhalt ausgeben
    /*
    for line in reader.lines() {
        counter = counter + 1;
        let line = line.expect("Fehler beim Lesen der Zeile");
        println!("{}", line);
    }
    */

    println!("\nDie ausgelesene Datei hat {} Zeilen", counter);
}
