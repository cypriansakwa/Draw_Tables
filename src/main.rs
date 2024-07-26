use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Terminal,
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, ClearType};
use crossterm::{execute, terminal::Clear};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal initialization
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Data for the table
    let header = [" ", "a", "b", "c", "d"];
    let rows = vec![
        vec!["a", "c", "a", "d", "b"],
        vec!["b", "a", "b", "c", "d"],
        vec!["c", "d", "c", "b", "a"],
        vec!["d", "b", "d", "a", "c"],
    ];

    // Table state for potential user interaction
    let mut table_state = TableState::default();
    table_state.select(Some(0));

    // Drawing the table
    terminal.draw(|f| {
        // Layout to center the table
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let table = Table::new(rows.iter().map(|row| {
            Row::new(row.iter().map(|cell| Cell::from(*cell).style(Style::default().fg(Color::White))))
        }))
        .header(
            Row::new(header.iter().map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow))))
                .bottom_margin(1),
        )
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Length(3),  // Adjust length for proper spacing
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ]);

        f.render_stateful_widget(table, chunks[0], &mut table_state);
    })?;

    disable_raw_mode()?;
    Ok(())
}





