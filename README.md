# favlist
[![Build Status](https://travis-ci.com/spenserblack/favlist.svg?branch=master)](https://travis-ci.com/spenserblack/favlist)

A tool to make lists of my favorite things

# (Planned) Usage
```bash
# create a new list of movies with an important (not-null) title and an integer year
favlist new Movies ~Title Year@int
# Add a new entry to the Movies list
favlist add Movies -c Title "The Curse of the Cursed Curse" -c Year 2006
# list movies with "Curse" in the title made in 2006 and print in YAML format
favlist list Movies -f Title Curse --filter Year 2006 --yaml
```
- Create a new list with `favlist new`
- Delete a list with `favlist rem`
- Add a new row to the list with `favlist add`

# Build
## Dependencies
```bash
apt install libsqlite3-dev
```
