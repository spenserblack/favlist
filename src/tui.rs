#[cfg(feature = "favlist-tui")]
use rusqlite::Connection;

#[cfg(not(feature = "favlist-tui"))]
const NO_TUI_ERROR: &str = "\
It looks like this was compiled without the `favlist-tui` feature.
This means that you can't launch the TUI, and will have to interact with your
favlist solely through the CLI.";

#[cfg(feature = "favlist-tui")]
pub fn start_ui(conn: Connection) {
    unimplemented!("start the TUI");
}

#[cfg(not(feature = "favlist-tui"))]
pub fn start_ui<T>(_conn: T) {
    eprintln!("{}", NO_TUI_ERROR);
    println!("Please run `favlist -h` to get help on how to use favlist.");
}
