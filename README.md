# Noogle Search 

A terminal UI for searching and browsing [Noogle](https://noogle.dev) - the Nix function search engine. Search through Nix functions with fuzzy finding, preview documentation, and jump to source code.

## Features

- **Fuzzy search** through all Nix functions using fzf
- **Namespace filtering** - filter by lib, builtins, or pkgs with a single keypress
- **Live preview** of function documentation, type signatures, and examples
- **Quick navigation**: Jump to GitHub source code (Ctrl-O) or Noogle page (Ctrl-N)
- **Offline caching** - data cached for 24 hours
- **Syntax highlighting** via bat

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

**Or run directly:**

```bash
nix run github:argosnothing/noogle-search
```

## Usage

Start the search interface:

```bash
# Search all namespaces
noogle-search

# Start with lib namespace filtered
noogle-search filter lib

# Start with builtins namespace filtered
noogle-search filter builtins

# Start with pkgs namespace filtered
noogle-search filter pkgs
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

### Namespace Filtering

When you apply a namespace filter:
- Only functions from that namespace are shown
- The namespace prefix is stripped from the display (e.g., "optionalString" instead of "lib.optionalString")
- Functions are matched by their primary title, not aliases from other namespaces

### Notes for impermanence.
`noogle-search` uses a cache to avoid excessive calls to noogle on a 24h timer in `~/.cache/noogle-search`

## Credits

- Data provided by [Noogle.dev](https://noogle.dev) 
- the Nix function search engine by [@hsjobeki](https://github.com/hsjobeki). 
- The clankers who did most of the work.

## Dependencies

- `fzf` - fuzzy finder
- `bat` - syntax highlighting (optional, falls back to plain text)
- `xdg-open` - opening URLs in browser
