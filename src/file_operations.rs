use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn read_file(filepath: &str) {
    let mut counter: i32 = 0;

    // String säubern


    // Datei öffnen
    let file = File::open(filepath).expect("Datei wurde nicht gefunden");

    // Buffered Reader erstellen
    let reader = BufReader::new(file);

    // Zeile für Zeile Inhalt ausgeben
    for line in reader.lines() {
        counter = counter + 1;
        let line = line.expect("Fehler beim Lesen der Zeile");
        println!("{}", line);
    }

    println!("\nDie ausgelesene Datei hat {} Zeilen", counter);
}
