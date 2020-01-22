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

    let mut tab_tracker = utils::TabTracker { current_position: 0, length: 0 };
    'main: loop {
        let table_names = data::available_tables(&conn);
        tab_tracker.length = table_names.len();
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
                    .select(tab_tracker.current_position)
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
                KeyCode::Char('<') => {
                    tab_tracker.next_back();
                }
                KeyCode::Char('>') => {
                    tab_tracker.next();
                }
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

#[cfg(feature = "favlist-tui")]
mod data;

#[cfg(feature = "favlist-tui")]
mod utils {
    pub struct TabTracker {
        pub current_position: usize,
        /// Exclusive
        pub length: usize,
    }

    impl Iterator for TabTracker {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            self.current_position += 1;
            if self.current_position >= self.length {
                self.current_position = 0;
            }
            Some(self.current_position)
        }
    }

    impl DoubleEndedIterator for TabTracker {
        fn next_back(&mut self) -> Option<Self::Item> {
            use std::cmp::max;
            if self.current_position == 0 {
                // In case length == 0
                self.current_position = max(1, self.length);
            }
            self.current_position -= 1;
            Some(self.current_position)
        }
    }

    #[cfg(test)]
    mod tests {
        pub use super::*;
        mod tab_tracker {
            use super::*;

            #[test]
            fn tab_tracker_overflow() {
                let mut tracker = TabTracker { current_position: 4, length: 5 };
                assert_eq!(Some(0), tracker.next());
            }

            #[test]
            fn tab_tracker_underflow() {
                let mut tracker = TabTracker { current_position: 0, length: 5 };
                assert_eq!(Some(4), tracker.next_back());
            }

            #[test]
            fn tab_tracker_underflow_with_0_length() {
                let mut tracker = TabTracker { current_position: 0, length: 0 };
                assert_eq!(Some(0), tracker.next_back());
            }
        }
    }
}
