# favlist
[![Build Status](https://travis-ci.com/spenserblack/favlist.svg?branch=master)](https://travis-ci.com/spenserblack/favlist)

A tool to make lists of my favorite things

# (Planned) Usage
```bash
# create a new list of movies with an important (not-null) title and an integer year
favlist new Movies *Title Year@int
# list movies with "Curse" in the title made in 2006 and print in YAML format
favlist list Movies -f Title Curse --filter Year 2006 --yaml
```
- [x] Create a new list with `favlist new`
- [ ] Delete a list with `favlist rem`
- [ ] Add to list with `favlist add`
  - [ ] Prompts for data if not passed via command-line
- [ ] Remove from list with `favlist sub`
  - [ ] Will prompt for a row number and provide previews
  - [ ] Can use similar `--filter` options as `list` to remove multiple rows
- [ ] `favlist edit` to edit a row
  - [ ] Provides preview and prompts for row if not provided
- [ ] List entries with `favlist list`
- [ ] TUI interface opened by simply running `favlist` (feature must be enabled)
- [ ] Defaults to storing with sqlite, but can use other formats, such as a folder of JSON files instead
- [ ] `.favlistrc.yml` file for config options such as default directory

# Build
## Dependencies
```bash
apt install libsqlite3-dev
```
