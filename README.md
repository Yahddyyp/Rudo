# rudo

A fast terminal todo list written in rust!

## Install

```bash
brew tap Yahddyyp/rudo
brew install rudo
```

Or with cargo:

```bash
cargo install rudo
```

## Usage

```bash
rudo                  # open the TUI
rudo list             # list tasks in the active list
rudo lists            # show all lists
rudo add Buy milk     # add a task (no quotes needed)
rudo done 1           # mark task 1 done
rudo undo 1           # uncheck task 1
rudo rm 1             # remove task 1
rudo use Work         # switch active list
rudo status           # completion stats
```

## TUI Keybinds

| Key | Action |
|-----|--------|
| `i` | Add item |
| `s` | Add sub-item |
| `h` | Add header |
| `-` | Add separator |
| `E` | Edit selected |
| `d` | Delete selected |
| `u` | Uncheck item|
| `Enter` | Toggle check |
| `j/k` | Move cursor |
| `Tab` | Switch panel |
| `/` | Search |
| `v` | Toggle completed |
| `Esc` | Menu |

## Data

State is stored at `~/.config/rudo/appdata.json`.

## License

MIT
