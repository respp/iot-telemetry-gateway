Final Project for Rust Advanced Certification Udemy

- A final project in the `final-project` folder: an **IoT Telemetry Parser & Forwarder** that:
  - Listens on a TCP port for incoming raw hexadecimal telemetry frames.
  - Parses the byte stream into strongly typed Rust structures (latitude, longitude, battery level, timestamp).
  - Forwards the parsed data to a metrics endpoint (simulated in this project).

You can build and run the final project with:

```bash
cargo run --bin final-project
```


You can see the project at my portfolio: https://renzobarcos.world/
