use std::io;
use tui::Terminal;
use tui::widgets::{Block, Borders};
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("my block")
            .borders(Borders::ALL);

        f.render_widget(block, size);
    })?;

    Ok(())
}
