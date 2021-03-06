# favlist
[![Crates.io](https://img.shields.io/crates/v/favlist)](https://crates.io/crates/favlist/)
![Crates.io](https://img.shields.io/crates/d/favlist)
[![Build Status](https://travis-ci.com/spenserblack/favlist.svg?branch=master)](https://travis-ci.com/spenserblack/favlist)

Easily make lists, backed up with a SQLite database

# Usage
```bash
# create a new list of movies with an important (not-null) title and an integer year
favlist new Movies ~Title Year@int
# Add a new entry to the Movies list
favlist add Movies -c Title "The Curse of the Cursed Curse" -c Year 2006
# list movies with "Curse" in the title made in 2006 and print in YAML format
favlist list Movies -f Title Curse --filter Year 2006 --yaml
# Change the Year in row 1 of Movies
favlist edit Movies 1 -c Year 2005
# Remove row 1 from Movies
favlist sub Movies 1

# Launch the TUI
favlist

# Help
favlist --help
favlist [SUBCOMMAND] --help
```
- Create a new list with `favlist new`
- Delete a list with `favlist rem`
- Add a new row to the list with `favlist add`
- Subtract a row from the list with `favlist sub`
- Edit values in a row with `favlist edit`
- Print values of list with `favlist list`

## TUI

![TUI Screenshot](https://github.com/spenserblack/favlist/blob/master/images/screenshot.png?raw=true)

The TUI is included by default. If you don't want this feature included, pass the
`--no-default-features` flag when installing/building with `cargo`.

For now, you cannot *edit* any data with the TUI, but you can *view* data.
Switch between tables with `<` and `>`, and highlight rows with `↑` and `↓`
on your keyboard. Exit the TUI with `ESC`.

# Install
```bash
# Latest released version
cargo install favlist

# Don't want the TUI feature?
cargo install --no-default-features favlist

# Want to get notified about new releases?
cargo install favlist --features check-latest
```

## Dependencies
### Linux
- libsqlite3-dev
