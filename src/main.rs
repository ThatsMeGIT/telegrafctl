use std::io;

use color_eyre::eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    prelude::*,
    widgets::{Block, Borders},
};

fn main() -> Result<()> {
    color_eyre::install()?;

    // 1) Terminal vorbereiten
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2) App laufen lassen (wichtig: Fehler "hochreichen")
    let result = run(&mut terminal);

    // 3) Terminal garantiert wiederherstellen (auch wenn run() fehlschlägt)
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // 4) Ergebnis zurückgeben
    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|f| {
            let area = f.area();
            let block = Block::default()
                .title("Meine erste TUI (q = quit)")
                .borders(Borders::ALL);
            f.render_widget(block, area);
        })?;

        // Input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }
    Ok(())
}
