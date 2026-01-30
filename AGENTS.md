# stamp.env

A Rust CLI tool for quickly deploying preset-based folder/file structures via symlinks.

## Core Commands

- `stamp link <preset> [target]` - Create symlinks from a preset to the target path
- `stamp unlink [target]` - Remove created symlinks
- `stamp commit <name> [-p <patterns>...]` - Save current structure as a preset (supports glob pattern filtering)
- `stamp list` - List saved presets
- `stamp show <preset>` - Display preset contents

## Project Structure

```
src/
├── main.rs            # CLI entry point (using clap)
├── commands/          # CLI command handlers
│   ├── mod.rs
│   ├── link.rs        # link command implementation
│   ├── unlink.rs      # unlink command implementation
│   ├── commit.rs      # commit command implementation
│   └── list.rs        # list/show command implementation
├── preset/            # Preset management logic
│   ├── mod.rs
│   └── manager.rs     # Preset save/load
└── symlink/           # Symlink creation logic
    ├── mod.rs
    └── builder.rs
```

## Coding Rules

- **Error handling**: Use `anyhow` crate
- **Preset format**: TOML serialization (`serde`, `toml`)
- **Symlink conflicts**: Require user confirmation when existing files exist
- **CLI parsing**: Use `clap` derive macro

## Dependencies

| Crate    | Purpose                    |
| -------- | -------------------------- |
| clap     | CLI argument parsing       |
| anyhow   | Error handling             |
| serde    | Serialization/deserialization |
| toml     | Preset file format         |
| walkdir  | Recursive directory traversal |
| glob     | File pattern matching      |

## Preset Storage Location

Presets are stored as TOML files in `~/.config/stamp.env/presets/`.
