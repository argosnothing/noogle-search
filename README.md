# Noogle Search 

Update: just save yourself the trouble and use [nix-search-tv)(https://github.com/3timeslazy/nix-search-tv) as that now supports noogle search, and gives you a bunch of other stuffs :p

A terminal UI for searching and browsing [Noogle](https://noogle.dev) - the Nix function search engine. Search through Nix functions with fuzzy finding, preview documentation, and jump to source code.

![tmp O7pKaesVCx](https://github.com/user-attachments/assets/5dc04a3c-6133-4801-a0df-08e9bbca4d0a)

## Features

- **Fuzzy search** through all Nix functions using fzf
- **Flexible filtering** - filter by any namespace prefix
- **Initial query** - start with a search term pre-populated
- **Live preview** of function documentation, type signatures, and examples
- **Quick navigation**: Jump to GitHub source code (Ctrl-O) or Noogle page (Ctrl-N)
- **Offline caching** - data cached for 24 hours
- **Syntax highlighting** via bat

## Try It!
```bash
nix run github:argosnothing/noogle-search
```
## Installation

### Nix Flakes

**Add to your flake inputs:**

```nix
{
  inputs = {
    noogle-search.url = "github:argosnothing/noogle-search";
  };
}
```

**Use in your configuration:**

```nix
# NixOS configuration.nix
{ inputs, pkgs, ... }:
{
  environment.systemPackages = [
    inputs.noogle-search.packages.${pkgs.system}.default
  ];
}
```



## Usage

Start the search interface:

```bash
# Search all namespaces
noogle-search

# Start with a search query
noogle-search shell

# Filter to lib namespace
noogle-search -f lib

# Filter to lib namespace with initial query
noogle-search -f lib shell

# Filter to any prefix (e.g., lib.strings)
noogle-search -f lib.strings trim

# Filter to builtins
noogle-search -f builtins

# Filter to pkgs
noogle-search -f pkgs
```

### Keybinds

**Namespace Filters:**
- **Ctrl-L**: Filter to lib.* functions only
- **Ctrl-B**: Filter to builtins.* functions only
- **Ctrl-P**: Filter to pkgs.* functions only
- **Ctrl-A**: Show all functions (remove filter)

**Actions:**
- **Ctrl-O**: Open function source code on GitHub
- **Ctrl-N**: Open function page on Noogle.dev
- **Ctrl-/**: Toggle preview pane

### Filtering

When you apply a filter with `-f` or `--filter`:
- Only functions matching that prefix are shown
- The prefix is stripped from the display (e.g., "optionalString" instead of "lib.optionalString")
- You can filter by any prefix, not just top-level namespaces
- Functions are matched by their primary title, not aliases from other namespaces

### Notes for impermanence

`noogle-search` uses a cache to avoid excessive calls to noogle on a 24h timer in `~/.cache/noogle-search`

## Credits

- Data provided by [Noogle.dev](https://noogle.dev) 
- the Nix function search engine by [@hsjobeki](https://github.com/hsjobeki). 
- The clankers who did most of the work.

## Dependencies

- `fzf` - fuzzy finder
- `bat` - syntax highlighting (optional, falls back to plain text)
- `xdg-open` - opening URLs in browser
