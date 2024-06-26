use std::io::{Result, stdout};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use crossterm::event::KeyModifiers;
use rand::Rng;
use ratatui::style::Color;
use ratatui::text::Line;
use ratatui::widgets::Wrap;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Terminal},
    style::Style,
    text::Span,
    widgets::Paragraph,
};

use crate::game_logic::Element;
use crate::word_list::WORD_LIST;

mod game_logic;
mod word_list;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut input: String = "".to_string();
    let mut text_list: Vec<Line> = vec![];
    let mut game_state = GameState::NotStarted;
    let mut solution = rand_from_array(&WORD_LIST);

    loop {
        terminal.draw(|frame| {
            let hor = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                ])
                .split(frame.size());

            let left_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Max(1),
                    Constraint::Max(1),
                    Constraint::Min(6),
                    Constraint::Max(1),
                ])
                .split(hor[0]);

            let mid_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Max(1),
                    Constraint::Max(1),
                    Constraint::Min(6),
                    Constraint::Max(1),
                ])
                .split(hor[1]);

            let right_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Max(1),
                    Constraint::Max(1),
                    Constraint::Min(6),
                    Constraint::Max(1),
                ])
                .split(hor[2]);

            let _area = frame.size();

            // Header
            frame.render_widget(
                Paragraph::new("").style(Style::new().bg(Color::Green)),
                left_area[0],
            );
            frame.render_widget(
                Paragraph::new("").style(Style::new().bg(Color::Green)),
                right_area[0],
            );
            frame.render_widget(
                Paragraph::new("WORDLE")
                    .centered()
                    .style(Style::new().bg(Color::Green)),
                mid_area[0],
            );

            // Body
            match game_state {
                GameState::Running => {
                    frame.render_widget(Paragraph::new(text_list.clone()).centered(), mid_area[2])
                }
                GameState::Won => {
                    let winning_text = format!(
                        "YOU'VE WON!\nThe word was: {}\nPress Enter to restart.",
                        solution.to_uppercase()
                    );
                    frame.render_widget(Paragraph::new(winning_text).centered(), mid_area[2]);
                }
                GameState::Lost => {
                    let losing_text = format!(
                        "YOU'VE LOST!\nThe word was: {}\nPress Enter to restart.",
                        solution.to_uppercase()
                    );
                    frame.render_widget(Paragraph::new(losing_text).centered(), mid_area[2]);
                }
                GameState::NotStarted => {
                    let not_started_text =
                        "Type 5-character-words and press ENTER to submit them.\n\n\
                        You have 6 tries to guess the correct word.\n\n\
                        Green means the letter is at the right place.\n\n\
                        Yellow means the letter is in the word but not in the right place.\n\n\
                        Press ESC to exit.";
                    frame.render_widget(
                        Paragraph::new(not_started_text)
                            .wrap(Wrap::default())
                            .centered(),
                        mid_area[2],
                    );
                }
                GameState::WrongWord => {
                    let wrong_word_text = "The Word was not valid¸\n Please enter a valid word";
                    frame.render_widget(Paragraph::new(wrong_word_text).centered(), mid_area[2]);
                }
            };
            // Footer
            let input_prompt: String = format!(
                "Input: {}{}",
                input.to_uppercase().clone(),
                "_".repeat(5 - input.len())
            );
            frame.render_widget(Paragraph::new("").centered(), left_area[3]);
            frame.render_widget(Paragraph::new(input_prompt).centered(), mid_area[3]);
            frame.render_widget(Paragraph::new("").centered(), right_area[3]);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc {
                    break;
                }

                if game_state == GameState::NotStarted {
                    game_state = GameState::Running
                }

                match key.code {
                    KeyCode::Char(c) => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            break;
                        }

                        if input.len() < 5 && c.is_ascii() && c.is_alphabetic() {
                            input.push(c.to_ascii_lowercase())
                        }
                    }
                    KeyCode::Enter => {
                        if game_state == GameState::Won || game_state == GameState::Lost {
                            game_state = GameState::Running;
                            text_list = vec![];
                            input = "".to_string();
                            solution = rand_from_array(&WORD_LIST)
                        }
                        else if WORD_LIST.contains(&input.as_str()) {
                            if game_state == GameState::WrongWord {
                                game_state = GameState::Running;
                            }
                            if input.len() < 5 { continue; }

                            let elements = game_logic::check_word(
                                input.to_string(),
                                solution.to_string(),
                            );
                            text_list.push(list_to_span(&elements));

                            if input == solution { game_state = GameState::Won }
                            if text_list.len() > 5 { game_state = GameState::Lost }
                            input = "".to_string();
                        } else {
                            game_state = GameState::WrongWord;
                            input = "".to_string();
                        }
                    }
                    KeyCode::Backspace => {
                        if !input.is_empty() {
                            input.remove(input.len() - 1);
                        }
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

#[derive(PartialEq, Debug, Clone)]
enum GameState {
    NotStarted,
    Running,
    WrongWord,
    Won,
    Lost,
}

fn list_to_span(elements: &[Element]) -> Line<'static> {
    let mut styled_chars: Vec<Span> = elements
        .iter()
        .map(|e| {
            Span::styled(
                e.c.to_string().to_uppercase(),
                Style::default().fg(e.status.color()),
            )
        })
        .collect();
    styled_chars.push(Span::styled("\n".to_string(), Style::default()));

    Line::from(styled_chars)
}

fn rand_from_array(array: &[&str]) -> String {
    let random = rand::thread_rng().gen_range(0..array.len());
    array[random].to_string()
}
