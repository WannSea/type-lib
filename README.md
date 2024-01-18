# type-lib
Defines all common types shared in the different WannSea components as individual libraries for the used languages.

## Metrics
Every metric that we collect on the boat is defined in the [wannsea.proto](protos/wannsea.proto)
A readable overview can be found [here](Metrics.md) which is automatically generated on each commit.

## Rust
The rust folder contains a cargo library which can be embedded inside Rust projects. 
It generates the corresponding rust structs by using the [prost crate](https://github.com/tokio-rs/prost/) and [pbjson](https://github.com/influxdata/pbjson) for implementing serde-traits. 
**This is done inside the build script, so there are no build artifacts inside this repo**