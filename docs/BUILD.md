# Build Sonora

Use Windows x64, Rust 1.94+ MSVC, Visual Studio C++ build tools and Windows SDK.

Clone this repository only, then enter `apps/sonora`:

```powershell
cargo fmt --package sonora -- --check
cargo clippy --locked --release --all-targets --no-deps -- -D warnings
cargo test --locked --release --all-targets -- --test-threads=1
cargo build --locked --release
```

Keep Cargo.lock unchanged. Shared UI/art source is included in this repository.
Runtime users do not need developer tools. Format tests use owned generated media;
conditional Windows codec tests can report explicit skips when extensions are absent.
Physical input, media keys and perceptual checks are separate from CI.

## Package

Python 3.11+ and NSIS are developer prerequisites. From the root:

```powershell
python scripts/package-release.py --app sonora --exe apps/sonora/target/release/sonora.exe --out dist/release
```

Pass --nsis with the path to makensis.exe if it is not at the script's default location.
Use a fresh output directory. It creates a complete portable ZIP, per-user installer,
notices, documentation and manifest. Preserve every dependency notice and included MPL source.
