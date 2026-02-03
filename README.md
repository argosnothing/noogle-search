# Noogle Search TV

A terminal UI for searching and browsing [Noogle](https://noogle.dev) - the Nix function search engine. Search through Nix functions with fuzzy finding, preview documentation, and jump to source code.

## Features

- **Fuzzy search** through all Nix functions using fzf
- **Live preview** of function documentation, type signatures, and examples
- **Quick navigation**: Jump to GitHub source code (Ctrl-O) or Noogle page (Ctrl-N)
- **Offline caching** - data cached for 24 hours
- **Syntax highlighting** via bat

## Installation

### Using Nix Flakes

Add to your `flake.nix`:

```nix
{
  inputs = {
    noogle-search.url = "github:argosnothing/noogle-search";
  };

  outputs = { self, nixpkgs, noogle-search-tv }: {
    # In your NixOS configuration or home-manager
    environment.systemPackages = [
      noogle-search.packages.${system}.default
    ];
  };
}
```

Or run directly:

```bash
nix run github:argosnothing/noogle-search
```

### Building from source

```bash
nix build
./result/bin/noogle-search-tv search
```

## Usage

```bash
noogle-search
```

### Keybinds

- **Ctrl-O**: Open function source code on GitHub
- **Ctrl-N**: Open function page on Noogle.dev
- **Ctrl-/**: Toggle preview pane

## Example Output

When you select a function like `lib.strings.optionalString`, you'll see:

- Type signature
- Function definition
- Documentation with inputs/outputs
- Usage examples
- Source location
- Aliases

## Credits

- Data provided by [Noogle.dev](https://noogle.dev) - the Nix function search engine by [@hsjobeki](https://github.com/hsjobeki).
- The clankers who did most of the work.

## Dependencies

- `fzf` - fuzzy finder
- `bat` - syntax highlighting (optional, falls back to plain text)
- `xdg-open` - opening URLs in browser
