#[cfg(feature = "favlist-tui")]
use rusqlite::{Connection, NO_PARAMS};
#[cfg(feature = "favlist-tui")]

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
    use super::query_builder;
    use tui::{
        backend::CrosstermBackend,
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, Row, Table, Tabs, Widget},
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
    let header_style = default_style.modifier(Modifier::BOLD);

    let mut tab_tracker = utils::TabTracker { current_position: 0, length: 0 };
    'main: loop {
        let table_names = data::available_tables(&conn);
        tab_tracker.length = table_names.len();

        let mut row_ids: Vec<u32> = Vec::new();

        terminal
            .draw(|mut f| {
                let size = f.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Percentage(80),
                    ].as_ref())
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
                // Table definition {{{
                // NOTE Highlighting specific row
                // let rows = rows.into_iter().enumerate().map(|(i, item)| {
                //     if i == 1 {
                //         Row::StyledData(item.into_iter(), selected_style)
                //     } else {
                //         Row::StyledData(item.into_iter(), default_style)
                //     }
                // });
                if let Some(table_name) = table_names.get(tab_tracker.current_position) {
                    let mut stmt = conn.
                        prepare(&query_builder::List::new(table_name, None).to_string())
                        .unwrap();
                    let header = stmt.column_names().into_iter().map(String::from).collect::<Vec<_>>().into_iter().skip(1);
                    let width = stmt.column_count();
                    let widths: Vec<_> = (1..width).map(|_| {Constraint::Percentage((100 / width - 1) as u16)}).collect();
                    let mut rows = stmt.query(NO_PARAMS).unwrap();
                    let mut tui_rows = Vec::new();
                    while let Ok(Some(row)) = rows.next() {
                        row_ids.push(row.get_unwrap("id"));
                        let row = (1..width)
                            .map(|i| row.get_raw(i))
                            .map(|v| {
                                use rusqlite::types::ValueRef::*;
                                use std::str::from_utf8;
                                match v {
                                    Null => "<NULL>".to_string(),
                                    Integer(i) => i.to_string(),
                                    Real(r) => r.to_string(),
                                    Text(utf8) => from_utf8(utf8).unwrap().to_string(),
                                    Blob(utf8) => from_utf8(utf8).unwrap().to_string(),
                                }
                            });
                        let row = Row::StyledData(row.collect::<Vec<_>>().into_iter(), default_style);
                        tui_rows.push(row);
                    }
                    Table::new(header.into_iter(), tui_rows.into_iter())
                        .widths(&widths)
                        .header_style(header_style)
                        .render(&mut f, chunks[1]);
                }
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
