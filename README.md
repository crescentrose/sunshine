# sunshine

A simple CLI tool to determine sunrise and sunset times.

## Installation

### Via Homebrew (macOS only)

Use the [Homebrew tap](https://github.com/crescentrose/homebrew-sunshine):

```bash
brew tap crescentrose/sunshine
brew install sunshine
```

### Manually

The latest releases should be available on the
[Releases](https://github.com/crescentrose/sunshine/releases) page, with
precompiled versions for macOS and Linux. If you want to build the project
yourself, you'll need the Rust compiler - follow the steps laid out in the
[Building](#Building) section.

Note that if you're running `sunshine` on macOS, you may need to grant the
application location permissions if you're using CoreLocation (an authorization
window will pop up).

## Usage

```
sunshine [--simple] [--format="fmtstr"] <location>
```

`location` can be:

- latitude and longitude prefixed with an @ character
- `.` to fetch the location from the network using
    [FreeGeoIP](https://freegeoip.app)
- location name prefixed with an # character, fetched from
    [OpenStreetMap](http://nominatim.openstreetmap.org) and cached locally
- prefix the location string with an `!` to draw location data from macOS
  CoreLocation if available, or the GeoIP API. The rest of the location string
  will be used as a fallback.

Examples:

```
@45.8 15.9
  => sunset and sunrise times in Zagreb, Croatia

!@45.8 15.9
  => attempt to get GPS data or the GeoIP API, default to (45.8, 15.9) if unavailable

.
  => attempt to get data from the GeoIP API

!#Amsterdam
  => get GPS or GeoIP data if available, otherwise fall back to Amsterdam

#Venice, US
  => get the sunrise and sunset times in Venice, US
```

`format` option is forwarded directly to [chrono's format
function](https://docs.rs/chrono/0.4.11/chrono/format/strftime/index.html).

If `simple` is passed, the only result will be either `day` or `night`,
depending on the time of the day (useful for easy scripting).

The times will **always** be in your system's timezone, **not** the timezone of
the location.

## Building

A `cargo build --release` should suffice. Tested on Rust 1.44.

## Contributing

This is a glorified shell script that I made to help teach myself Rust, so don't
expect much in the way of code quality or maintainability. Since I don't intend
on maintaining this project and turning it into the Next Big Thing I won't (at
least for now) accept feature pull requests. However, if you find a bug or an
especially egregious offense against Rust standards, please let me know.

## License

This program is licensed under the terms of the Apache license, version 2.0. The
full text is available in the [LICENSE](LICENSE) file.
