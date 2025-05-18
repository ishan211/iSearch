/*
Name: results.rs
Author: Ishan Leung
Language: Rust
Description: Terminal UI for displaying search results with better formatting, highlighting, and file browsing.
*/

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    fs,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Terminal,
};

#[derive(PartialEq)]
enum Mode {
    Results,
    File,
}

pub fn interactively_display_results(results: &[(String, f64)], query: &str) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut selected = 0;
    let mut mode = Mode::Results;
    let mut file_content = String::new();
    let mut scroll_offset: usize = 0;

    let mut last_key_time = Instant::now();

    loop {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if last_key_time.elapsed() < Duration::from_millis(120) {
                    continue;
                }
                last_key_time = Instant::now();

                let mut should_redraw = false;

                match mode {
                    Mode::Results => match key.code {
                        KeyCode::Up => {
                            if selected > 0 {
                                selected -= 1;
                                should_redraw = true;
                            }
                        }
                        KeyCode::Down => {
                            if selected < results.len().saturating_sub(1) {
                                selected += 1;
                                should_redraw = true;
                            }
                        }
                        KeyCode::Enter => {
                            file_content = fs::read_to_string(format!("cleaned/{}", results[selected].0))?;
                            scroll_offset = 0;
                            mode = Mode::File;
                            should_redraw = true;
                        }
                        KeyCode::Esc => break,
                        _ => {}
                    },
                    Mode::File => match key.code {
                        KeyCode::Up => {
                            if scroll_offset > 0 {
                                scroll_offset -= 1;
                                should_redraw = true;
                            }
                        }
                        KeyCode::Down => {
                            scroll_offset += 1;
                            should_redraw = true;
                        }
                        KeyCode::PageUp => {
                            scroll_offset = scroll_offset.saturating_sub(10);
                            should_redraw = true;
                        }
                        KeyCode::PageDown => {
                            scroll_offset += 10;
                            should_redraw = true;
                        }
                        KeyCode::Enter | KeyCode::Esc => {
                            mode = Mode::Results;
                            scroll_offset = 0;
                            should_redraw = true;
                        }
                        _ => {}
                    },
                }

                if should_redraw {
                    match mode {
                        Mode::Results => terminal.draw(|f| draw_results(f, results, query, selected))?,
                        Mode::File => terminal.draw(|f| draw_file(f, &file_content, scroll_offset))?,
                    };
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn draw_results(f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, results: &[(String, f64)], query: &str, selected: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let header = Paragraph::new(format!("Results for '{}' ({} found)", query, results.len()))
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = results
        .iter()
        .enumerate()
        .map(|(i, (file, score))| {
            let snippet = get_text_snippet(file, query).unwrap_or_default();
            let line = if i == selected {
                Spans::from(vec![
                    Span::styled("> ", Style::default().fg(Color::Blue)),
                    Span::styled(
                        format!("{}. {} (score: {:.2})", i + 1, file, score),
                        Style::default().fg(Color::White).bg(Color::Blue),
                    ),
                    Span::raw("\n   "),
                    Span::raw(snippet),
                ])
            } else {
                Spans::from(vec![
                    Span::raw("  "),
                    Span::styled(format!("{}. {} ", i + 1, file), Style::default().fg(Color::Yellow)),
                    Span::styled(format!("(score: {:.2})", score), Style::default().fg(Color::White)),
                    Span::raw("\n   "),
                    Span::styled(snippet, Style::default().fg(Color::Gray)),
                ])
            };
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::NONE));
    f.render_widget(list, chunks[1]);

    let footer = Paragraph::new("↑/↓: Navigate  Enter: View  Esc: Exit")
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(footer, chunks[2]);
}

fn draw_file(f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, content: &str, scroll_offset: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let header = Paragraph::new("File Content")
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(header, chunks[0]);

    let paragraph = Paragraph::new(Text::from(content))
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset as u16, 0));
    f.render_widget(paragraph, chunks[1]);

    let footer = Paragraph::new("↑/↓: Scroll  Enter/Esc: Return to results")
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(footer, chunks[2]);
}

fn get_text_snippet(filename: &str, query: &str) -> Result<String> {
    let file_path = format!("cleaned/{}", filename);
    let content = match fs::read_to_string(&file_path) {
        Ok(text) => text,
        Err(e) => return Ok(format!("Error reading file: {}", e)),
    };

    let content_lower = content.to_lowercase();
    let query_lower = query.to_lowercase();
    let query_terms: Vec<&str> = query_lower.split_whitespace().collect();

    let mut best_pos = 0;
    for term in &query_terms {
        if let Some(pos) = content_lower.find(term) {
            best_pos = pos;
            break;
        }
    }

    let char_pos = content[..best_pos].chars().count();
    let chars: Vec<char> = content.chars().collect();
    let start = if char_pos > 75 { char_pos - 75 } else { 0 };
    let end = (char_pos + 75).min(chars.len());

    let snippet: String = chars[start..end].iter().collect();
    let mut result = if end < chars.len() {
        format!("{}...", snippet)
    } else {
        snippet
    };

    result = result.replace('\n', " ");
    Ok(result)
}
