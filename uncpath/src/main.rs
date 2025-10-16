use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::io;

fn main() -> Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    if let Err(e) = run_app(&mut terminal) {
        cleanup_terminal(&mut terminal)?; // 出错也要恢复
        return Err(e);
    }

    cleanup_terminal(&mut terminal)?;
    Ok(())
}

fn cleanup_terminal<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let area = f.size();
            let block = Block::default().title(" unc ").borders(Borders::ALL);
            let text = Paragraph::new("Hello World")
                .block(block)
                .alignment(Alignment::Center);
            f.render_widget(text, area);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

