# favlist
[![Build Status](https://travis-ci.com/spenserblack/favlist.svg?branch=master)](https://travis-ci.com/spenserblack/favlist)
[![Dependabot Status](https://api.dependabot.com/badges/status?host=github&repo=spenserblack/favlist)](https://dependabot.com)

Easily make lists, backed up with a SQLite database

# Usage
```bash
# create a new list of movies with an important (not-null) title and an integer year
favlist new Movies ~Title Year@int
# Add a new entry to the Movies list
favlist add Movies -c Title "The Curse of the Cursed Curse" -c Year 2006
# list movies with "Curse" in the title made in 2006 and print in YAML format
favlist list Movies -f Title Curse --filter Year 2006 --yaml
# Remove row 1 from Movies
favlist sub Movies 1
```
- Create a new list with `favlist new`
- Delete a list with `favlist rem`
- Add a new row to the list with `favlist add`
- Subtract a row from the list with `favlist sub`
- Print values of list with `favlist list`

# Build
```bash
# Just to build
cargo build --release # Executable will be in ./target/release

# Build and install
cargo install --path .
```

## Dependencies
### Linux
- libsqlite3-dev
