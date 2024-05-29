mod word_list;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut text_list: String = "".to_string();
    let mut input: String = "".to_string();

    loop {
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Min(6), Constraint::Max(1)])
                .split(frame.size());

            let _area = frame.size();
            frame.render_widget(Paragraph::new(text_list.clone()), layout[0]);
            frame.render_widget(Paragraph::new(input.clone()), layout[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }

                match key.code {
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Enter => {
                        text_list.push_str(&input.clone());
                        text_list.push('\n');
                        input = "".to_string();
                    }
                    _ => continue,
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

use rand::Rng;

fn rand_from_array(array: &[&str]) -> String{
    let random = rand::thread_rng().gen_range(0..array.len());
    array[random].to_string()
}
