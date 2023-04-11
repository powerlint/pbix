# pbix

pbix is a crate that lets you parse and extract metadata from Power BI report files (`.pbix`).

## Parsing files

You can use `pbix::parse_file(path: AsRef<Path>)` to parse a Power BI report file for a given path:

```rs
match pbix::parse_file("Example.pbix") {
    Ok(report) => println!("Parsed {} pages from the report file!", report.pages.len()),
    Err(e) => eprintln!("Failed to parse report: {e}"),
};
```

### `from_bytes`

Alternatively, you can use `pbix::from_bytes(bytes: &[u8])` to parse a report file if you would like to work with bytes (instead of filenames).

```rs
let bytes = ...;

match pbix::from_bytes(&bytes) {
    Ok(report) => println!("Parsed {} pages from the report file!", report.pages.len()),
    Err(e) => eprintln!("Failed to parse report: {e}"),
};
```

## Features

Below you can find documentation on the different feature flags that this crate exposes.

### `rayon`

This feature uses the [rayon][rayon] crate to introduce parallelism in report data parsing and transforming.

You can enable this feature by including it in your `Cargo.toml`: no extra configuration or code is required!

```toml
[dependencies]
pbix = { version = ..., features = ["rayon"] }
```

*Basic testing on a Power BI report sourced from Microsoft's examples indicated a ~15% improvement in parsing time when using this feature.*

[rayon]: https://crates.io/crates/rayon
