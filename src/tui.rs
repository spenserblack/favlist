#[cfg(feature = "favlist-tui")]
use rusqlite::Connection;

#[cfg(not(feature = "favlist-tui"))]
const NO_TUI_ERROR: &str = "\
It looks like this was compiled without the `favlist-tui` feature.
This means that you can't launch the TUI, and will have to interact with your
favlist solely through the CLI.";

#[cfg(feature = "favlist-tui")]
pub fn start_ui(conn: Connection) {
    use crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use std::io::{self, Write};
    use tui::{
        backend::CrosstermBackend,
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, Tabs, Widget},
        Terminal,
    };

    enable_raw_mode().unwrap();

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();

    terminal.clear().unwrap();

    let default_style = Style::default().fg(Color::Yellow);
    let selected_style = Style::default().fg(Color::Black).bg(Color::Yellow);

    'main: loop {
        let table_names = data::available_tables(&conn);
        terminal
            .draw(|mut f| {
                let size = f.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Length(3), Constraint::Percentage(80)].as_ref())
                    .split(size);
                // Surrounding block {{{
                Block::default()
                    .borders(Borders::ALL)
                    .title("favlist")
                    .render(&mut f, size);
                // }}}
                // Table Tabs {{{
                Tabs::default()
                    // .block(Block::default().title("lists"))
                    .titles(&table_names)
                    .style(default_style)
                    .highlight_style(selected_style)
                    .render(&mut f, chunks[0]);
                // }}}
            })
            .unwrap();

        match event::read().unwrap() {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Esc => break 'main,
                _ => {}
            },
            _ => {}
        }
    }

    // Cleanup {{{
    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();
    // }}}
}

#[cfg(not(feature = "favlist-tui"))]
pub fn start_ui<T>(_conn: T) {
    eprintln!("{}", NO_TUI_ERROR);
    println!("Please run `favlist -h` to get help on how to use favlist.");
}

mod data;
