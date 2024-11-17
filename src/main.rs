mod database;
mod openai;
mod utils;
use anyhow::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use database::database::{history, register_prompt};
use openai::api::chat;
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;
use std::io;
use tui_textarea::{Input, Key, TextArea};
use utils::getenv::envkeys;
use utils::queries::find_query;

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
                textarea.insert_newline();
                textarea.insert_str(
                    "===============================================================================",
                );
                let text_vec = textarea.lines();
                let query_text = find_query(text_vec.to_vec());
                let text: String;
                if query_text.len() > 0 {
                    text = find_query(text_vec.to_vec()).join("\n");
                } else {
                    text = text_vec.join("\n");
                }
                let chat_histoy = history().await?;
                match chat(api_key.clone(), &text, chat_histoy).await {
                    Ok(response) => {
                        let _ = register_prompt(&text, &response).await;
                        textarea.insert_newline();
                        textarea.insert_str(response);
                        textarea.insert_newline();
                        textarea.insert_str(
                            "===============================================================================",
                        );
                        textarea.insert_newline();
                    }
                    Err(e) => eprintln!("Erro: {}", e),
                };
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
