use std::io;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    loop {
        terminal.draw(|mut frame| {
            let size = frame.size();
            let block = Block::default().title("Block").borders(Borders::ALL);
            frame.render_widget(block, size);
        })?
    }
}
