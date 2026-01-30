# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-01-31

### Added

- `stampenv delete` - Delete saved presets
  - Support multiple preset deletion (`stampenv delete preset1 preset2`)
  - `--yes` flag to skip confirmation prompt
  - `--unlink` flag to also remove symlinks from linked locations

## [0.1.1] - 2026-01-31

### Changed

- Rename CLI command from `stamp` to `stampenv`

### Added

- English README and Korean translation
- Translate all Korean text to English

## [0.1.0] - 2026-01-31

### Added

- Initial release
- `stamp link` - Create symlinks from a preset to the target path
- `stamp unlink` - Remove created symlinks
- `stamp commit` - Save current structure as a preset (supports glob pattern filtering)
- `stamp list` - List saved presets
- `stamp show` - Display preset contents
- `--sync` flag for link tracking and auto-synchronization
