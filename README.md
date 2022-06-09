# yaml-to-json

Convert yaml to json and vice versa.

The project consists of two binaries y2j and j2y.
The first one will convert yaml to json and the second one json to yaml.

They both have a similar command line interface and will read from stdin by default.
Currently, both binaries can only output to stdout.
Run `y2j --help` or `j2y --help` for more information.

## Build

Both binaries can be built using cargo.

Run `cargo build --release` to build release binaries.
The binaries will be available at `target/release/y2j` and `target/release/j2y`.
