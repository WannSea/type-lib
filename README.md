# type-lib
Defines all common types shared in the different WannSea components as individual libraries for the used languages.

## Metrics
Every metric that we collect on the boat is defined in the [metrics.yaml](metrics.yaml)
A readable overview can be found [here](Metrics.md)

## Rust
The rust folder contains a cargo library which can be embedded inside Rust projects. 
It exposes all the defined Metrics including an ID and their corresponding data type.
This is used for the transport from our boat to the VPS allowing us to reduce data size by using a custom binary protocol, rather than relying on serialization techniques like JSON.

**NOTE: The MetricIDs and the Rust Data Types are automatically generated using the metrics.yaml definition file. This is done by using a build script (build.rs)**