# pg_temporal

A PostgreSQL extension that enables support for [Temporal ZonedDateTime](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime) date time values.

## Installation

This extension uses `pgrx` to compile Rust to a PostgreSQL extension.

### Local postgresql instance
To install for a version of postgres locally, ensure:
- You have installed the [System Requirements](https://github.com/pgcentralfoundation/pgrx#system-requirements) for `pgrx`
- You have installed the correct version of `pg_config`, which comes from the package `libpq-dev` on ubuntu

Then, run `cargo pgrx install`.

### Remote postgresql instance
To install for a version on another computer by bundling into an installation package, ensure:
- You have installed the [System Requirements](https://github.com/pgcentralfoundation/pgrx#system-requirements) for `pgrx`
- You have installed the correct version of `pg_config`, which comes from the package `libpq-dev` on ubuntu

Then, run `cargo pgrx package`. The output is stored in `target/release/pg_temporal-pg<version>`. This can then be built into a tarball, `.deb` or `.rpm` package, depending on what platform you want to target.
