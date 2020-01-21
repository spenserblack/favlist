# favlist
[![Crates.io](https://img.shields.io/crates/v/favlist)](https://crates.io/crates/favlist/)
![Crates.io](https://img.shields.io/crates/d/favlist)
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
# Change the Year in row 1 of Movies
favlist edit Movies 1 -c Year 2005
# Remove row 1 from Movies
favlist sub Movies 1
```
- Create a new list with `favlist new`
- Delete a list with `favlist rem`
- Add a new row to the list with `favlist add`
- Subtract a row from the list with `favlist sub`
- Edit values in a row with `favlist edit`
- Print values of list with `favlist list`

# Install
```bash
# Latest released version
cargo install favlist
```

## Dependencies
### Linux
- libsqlite3-dev
