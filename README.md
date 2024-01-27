# aprilasr-sys

Sys crate builds rust bindings for [april-asr](https://github.com/abb128/april-asr) from source.

## Development

First clone vendored source:

```
git submodule update --init --recursive
```

To generate bindings run command:

```
cargo build [--release]
```

To inspect bindings generated:

```
bat $(echo $(exa target/*/build/*/out/bindings.rs) | head -1)
```

Inspection command requires `bat` and `exa` rust libraries.

## Versioning

Consider using `chrono` to parse the data format unless april-asr adopts [semantic versioning](https://semver.org/):

```
let date_str = "2023.05.12";
let native_date = chrono::NaiveDate::parse_from_str(date_str, "%Y.%m.%d");
p!("{:?}", native_date);
```

Here `p!` is a debug helper in `build.rs` and `date_str` represents the `VERSION` in `vendor/april-asr/CMakeLists.txt` file. With some additional work [cmake-parser](https://crates.io/crates/cmake-parser) looks well-suited for parsing the file to get the version.

Date-based versioning is not currently implemented in the `build.rs` build script. Once versioning is implemented it would also ideal to use it as an input to [pkg-config](https://crates.io/crates/pkg-config) to scan the system for the library.

See [Making a \*-sys crate](https://kornel.ski/rust-sys-crate) for other possible enhancements.
