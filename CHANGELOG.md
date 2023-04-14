# Changelog

All notable changes history.
See the [keep a changelog](https://keepachangelog.com/en/1.1.0/) for details.
Remember to update version in `Cargo.toml`.

## [0.5.1] - 2023-04-09

### Added

* demo_full accepts argument --tty=&lt;number&gt; to use input
    from external terminal when running from Gdb session
* regenerated launch settings for VSCode:
    debugging from gdb-multiarch or from the QEMU

### Changed

* CharBuff::utf8str -> as_str
* reorganized demo_full
* widgets RuntimeState redesigned - no more state variants in single enum

### Deprecated

### Removed

* dependency on `libc` crate

### Fixed

* rendering of disabled window widgets:
  attributes BOLD/NORMAL does not break the FAINT atribute used to render disabled widget

### Security

## [0.5.0] - 2023-04-09

### Added

### Changed

* make the lib `no_std`

### Deprecated

### Removed

### Fixed

### Security
