use std::net::SocketAddr;

use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

/// Represents a decoded telemetry frame coming from the IoT device.
///
/// Raw frame layout (big-endian):
/// - 4 bytes: latitude in microdegrees (i32)
/// - 4 bytes: longitude in microdegrees (i32)
/// - 1 byte : battery level (0-100)
///
/// The raw bytes are received as a hexadecimal string over TCP, for example:
/// "0134F97CFFECB5C864" (without quotes).
#[derive(Debug, Serialize)]
struct TelemetryFrame {
    timestamp: DateTime<Utc>,
    latitude_deg: f32,
    longitude_deg: f32,
    battery_level: u8,
}

impl TelemetryFrame {
    /// Map a raw byte buffer to a typed `TelemetryFrame`.
    fn from_bytes(raw: &[u8]) -> Result<Self, String> {
        if raw.len() != 9 {
            return Err(format!(
                "invalid frame length: expected 9 bytes, got {}",
                raw.len()
            ));
        }

        let lat_bytes: [u8; 4] = raw[0..4]
            .try_into()
            .map_err(|_| "failed to extract latitude bytes".to_string())?;
        let lon_bytes: [u8; 4] = raw[4..8]
            .try_into()
            .map_err(|_| "failed to extract longitude bytes".to_string())?;

        let lat_microdeg = i32::from_be_bytes(lat_bytes);
        let lon_microdeg = i32::from_be_bytes(lon_bytes);
        let battery_level = raw[8];

        Ok(Self {
            timestamp: Utc::now(),
            latitude_deg: lat_microdeg as f32 / 1_000_000.0,
            longitude_deg: lon_microdeg as f32 / 1_000_000.0,
            battery_level,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration is intentionally simple for teaching purposes.
    let bind_addr = std::env::var("TELEMETRY_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:9000".into());
    let metrics_addr =
        std::env::var("METRICS_FORWARD_ADDR").unwrap_or_else(|_| "127.0.0.1:9100".into());

    println!("IoT Telemetry Parser & Forwarder");
    println!("Listening for telemetry on TCP {}", bind_addr);
    println!(
        "Forwarding parsed telemetry to metrics endpoint at TCP {} (simulated server)",
        metrics_addr
    );

    let listener = TcpListener::bind(&bind_addr).await?;

    loop {
        let (socket, peer_addr) = listener.accept().await?;
        let metrics_addr = metrics_addr.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, peer_addr, &metrics_addr).await {
                eprintln!("connection error from {}: {}", peer_addr, e);
            }
        });
    }
}

async fn handle_connection(
    socket: TcpStream,
    peer_addr: SocketAddr,
    metrics_addr: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("New connection from {}", peer_addr);

    let (reader, _writer) = socket.into_split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = buf_reader.read_line(&mut line).await?;
        if bytes_read == 0 {
            // Connection closed by client.
            println!("Connection from {} closed", peer_addr);
            break;
        }

        // Trim whitespace and newlines.
        let hex_str = line.trim();
        if hex_str.is_empty() {
            continue;
        }

        match decode_and_parse_frame(hex_str) {
            Ok(frame) => {
                println!("Decoded frame from {}: {:?}", peer_addr, frame);
                if let Err(e) = forward_to_metrics(&frame, metrics_addr).await {
                    eprintln!("Failed to forward telemetry to metrics server: {}", e);
                }
            }
            Err(err) => {
                eprintln!("Failed to parse frame from {}: {}", peer_addr, err);
            }
        }
    }

    Ok(())
}

fn decode_and_parse_frame(hex_str: &str) -> Result<TelemetryFrame, String> {
    let raw_bytes = hex::decode(hex_str).map_err(|e| format!("invalid hex data: {e}"))?;
    TelemetryFrame::from_bytes(&raw_bytes)
}

/// Simulated "metrics server" forwarder.
///
/// In a real production system this might:
/// - Send the data as JSON over HTTP to a metrics API.
/// - Push metrics to a time-series database.
/// - Emit Prometheus-style metrics over TCP/UDP.
///
/// Here we simply open a TCP connection and send one line per frame.
async fn forward_to_metrics(
    frame: &TelemetryFrame,
    metrics_addr: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut stream = TcpStream::connect(metrics_addr).await?;

    // Simple, human-readable line protocol for teaching purposes.
    let line = format!(
        "telemetry lat={:.6},lon={:.6},battery={} timestamp={}\n",
        frame.latitude_deg, frame.longitude_deg, frame.battery_level, frame.timestamp
    );

    stream.write_all(line.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}

