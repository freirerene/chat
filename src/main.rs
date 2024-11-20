mod backend;
mod llms;
mod utils;
use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Padding},
    Terminal,
};
use std::io;
use tui_textarea::{Input, Key, TextArea};
use utils::{
    locals::{envkeys, read_preferences},
    queries::query_response,
};

#[tokio::main]
async fn main() -> Result<()> {
    let bg_color = Color::Rgb(49, 50, 68);
    let border_color = Color::Rgb(180, 190, 254);

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let (llm, model) = match read_preferences("preferences.json") {
        Ok((x, y)) => (x.clone(), y.clone()),
        Err(e) => {
            eprintln!("Error reading preferences: {}", e);
            (String::from("undefined"), String::from("undefined"))
        }
    };

    let models = " ".to_owned() + &llm + "  -  " + &model + " ";

    let api_key = match envkeys(".env", "OPENAI_API_KEY")? {
        Some(key) => key,
        None => {
            println!("API Key is not defined.");
            return Ok(());
        }
    };

    enable_raw_mode()?;
    crossterm::execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        EnableLineWrap
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(bg_color))
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color))
            .padding(Padding::horizontal(10))
            .title(Line::from("Chat").centered())
            .title_bottom(Line::from(models).centered()),
    );

    // let style = Style::default().bg(Color::Red);
    // textarea.set_style(style);

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
