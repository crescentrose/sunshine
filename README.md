# sunshine

A simple CLI tool to determine sunrise and sunset times.

## Usage

```
sunshine [--simple] [--format="fmtstr"] <location>
```

`location` can be:

- latitude and longitude prefixed with an @ character
- **unimplemented** location name prefixed with an # character (I need to figure
  out the best way to do this without pulling huge databases from the net).
- prefix the location string with an `!` to draw location data from macOS
  CoreLocation if available. The rest of the location string will be used as a
  fallback.

Examples:

```
@45.8 15.9
  => sunset and sunrise times in Zagreb, Croatia

!@45.8 15.9
  => attempt to get GPS data, default to (45.8, 15.9) if unavailable

#Venice, US
  => Not yet implemented, will result in a panic!
```

`format` option is forwarded directly to [chrono's format
function](https://docs.rs/chrono/0.4.11/chrono/format/strftime/index.html).

If `simple` is passed, the only result will be either `day` or `night`,
depending on the time of the day (useful for easy scripting).

## Building

A `cargo build --release` should suffice. Tested on Rust 1.43.

## Contributing

This is a glorified shell script that I made to help teach myself Rust, so don't
expect much in the way of code quality or maintainability. Since I don't intend
on maintaining this project and turning it into the Next Big Thing I won't (at
least for now) accept feature pull requests. However, if you find a bug or an
especially egregious offense against Rust standards, please let me know.
