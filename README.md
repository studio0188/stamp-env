# stamp-env

A Rust CLI tool for quickly deploying preset-based folder/file structures via symlinks.

[한국어](README.ko.md)

## Installation

### Homebrew (macOS/Linux)

```bash
brew install studio0188/tap/stamp-env
```

### Shell (macOS/Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/studio0188/stamp-env/releases/latest/download/stamp-env-installer.sh | sh
```

### PowerShell (Windows)

```powershell
powershell -c "irm https://github.com/studio0188/stamp-env/releases/latest/download/stamp-env-installer.ps1 | iex"
```

### Cargo

```bash
cargo install stamp-env
```

## Usage

### Core Commands

- `stamp link <preset> [target]` - Create symlinks from a preset to the target path
- `stamp unlink [target]` - Remove created symlinks
- `stamp commit <name> [-p <patterns>...]` - Save current structure as a preset (supports glob pattern filtering)
- `stamp list` - List saved presets
- `stamp show <preset>` - Display preset contents

### Examples

```bash
# Save current directory structure as 'my-preset'
stamp commit my-preset

# Save only files matching specific patterns as a preset
stamp commit my-preset -p "*.rs" -p "Cargo.toml"

# Create symlinks from preset to target directory
stamp link my-preset ./target-dir

# Remove created symlinks
stamp unlink ./target-dir
```

## Preset Storage Location

Presets are stored as TOML files in `~/.config/stamp.env/presets/`.

## License

MIT License
