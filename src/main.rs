mod database;
mod openai;
mod utils;
use anyhow::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;
use std::io;
use tui_textarea::{Input, Key, TextArea};
use utils::getenv::envkeys;
use utils::queries::query_response;

#[tokio::main]
async fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let api_key = match envkeys(".env", "OPENAI_API_KEY")? {
        Some(key) => key,
        None => {
            println!("API Key is not defined.");
            return Ok(());
        }
    };

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::default();
    textarea.set_block(Block::default().borders(Borders::ALL).title("CHAT"));

    loop {
        term.draw(|f| {
            f.render_widget(&textarea, f.area());
        })?;
        match crossterm::event::read()?.into() {
            Input {
                key: Key::Char('a'),
                ctrl: true,
                alt: false,
                shift: false,
            } => {
                query_response(&mut textarea, api_key.clone()).await?;
            }
            Input { key: Key::Esc, .. } => break,
            input => {
                textarea.input(input);
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    println!("Lines: {:?}", textarea.lines());
    Ok(())
}
