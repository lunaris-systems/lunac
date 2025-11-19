# lunac

The Lunaris compiler - build tool and plugin manager.

## Installation

```bash
cargo install lunac
```

Or from source:
```bash
cargo install --path tools/lunac
```

## Usage

### Build & Run

```bash
lunac build              # Build debug
lunac build --release    # Build release
lunac run                # Run debug
lunac run --release      # Run release
```

Pass additional arguments to cargo:
```bash
lunac build --release --features foo
lunac run -- --help      # Pass --help to Lunaris
```

### Development

```bash
lunac check          # Check without building
lunac clippy         # Run linter
lunac test           # Run tests
lunac update         # Update plugin linker
```

### Plugin Management (Coming Soon)

```bash
lunac add <plugin>          # Add plugin dependency
lunac remove <plugin>       # Remove plugin
lunac new <type> <name>     # Create new plugin
lunac align                 # Align plugin versions
lunac validate              # Validate lunaris.toml
```

## Cargo-like Interface

`lunac` mirrors `cargo` commands:

| lunac | cargo equivalent |
|-------|------------------|
| `lunac build` | `cargo build --package lunaris_core` |
| `lunac build --release` | `cargo build --package lunaris_core --release` |
| `lunac run` | `cargo run --package lunaris_core` |
| `lunac check` | `cargo check --package lunaris_core` |
| `lunac clippy` | `cargo clippy --package lunaris_core` |
| `lunac test` | `cargo test --package lunaris_core` |

All commands accept additional cargo arguments via `--`.

## Future Features

- Plugin registry integration
- Dependency resolution
- Plugin scaffolding  
- Version alignment
- lunaris.toml validation
- Build caching
- Watch mode

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

