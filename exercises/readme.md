Exercise 01: Parsing a telemetry frame from raw bytes.

Goal:

- Implement `TelemetryFrame::from_bytes` so that all `assert!` calls in `main` pass.

Raw frame layout (big-endian):

- 4 bytes: latitude in microdegrees (i32)
- 4 bytes: longitude in microdegrees (i32)
- 1 byte : battery level (0-100)

 Exercise 02: Decoding a hex payload into bytes.

 Goal:

- Implement `decode_hex` using the `hex` crate (same as the final project).

 Run with:

 ```bash
 rustc 02_hex_decode.rs && ./02_hex_decode
```

 Exercise 03: Minimal Tokio TCP echo server.

 This is a simplified example to practice working with `tokio` and TCP.
 It is structurally similar to the final project, but instead of parsing
 telemetry it just echoes whatever it receives.

 This file is standalone on purpose; you can copy it into a small Cargo
 project if you want to run it with `cargo run`.
