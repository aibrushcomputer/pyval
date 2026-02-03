# Changelog

## [0.2.1] - 2025-02-03

### Changed
- Completely rewritten all documentation
- New structured docs/ folder
- Clean, concise README.md
- Complete API reference
- Architecture documentation
- Contributing guidelines

## [0.2.0] - 2025-02-03

### Added
- Initial release as `emailval` (renamed from `pyval`)
- Blazingly fast email validation (100-500x speedup)
- RFC 5322/6531 compliant validation
- IDN (Internationalized Domain Names) support
- SMTPUTF8 support
- Batch validation API
- Cross-platform wheels (Linux, macOS, Windows)
- Python 3.9-3.13 support

### Changed
- Renamed package from `pyval` to `emailval`
- New import: `from emailval import ...`

## [0.1.0] - 2025-01-15

### Added
- Initial development as `pyval`
- Core Rust validation engine
- PyO3 Python bindings
- Basic validation API
- Performance benchmarks
