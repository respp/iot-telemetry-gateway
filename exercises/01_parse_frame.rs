#[derive(Debug, PartialEq)]
struct TelemetryFrame {
    latitude_deg: f32,
    longitude_deg: f32,
    battery_level: u8,
}

impl TelemetryFrame {
    fn from_bytes(raw: &[u8]) -> Result<Self, String> {
        if raw.len() != 9 {
            return Err(format!(
                "invalid length: expected 9 bytes, got {}",
                raw.len()
            ));
        }

        let lat_bytes: [u8; 4] = raw[0..4]
            .try_into()
            .map_err(|_| "failed to read latitude".to_string())?;
        let lon_bytes: [u8; 4] = raw[4..8]
            .try_into()
            .map_err(|_| "failed to read longitude".to_string())?;

        let lat_microdeg = i32::from_be_bytes(lat_bytes);
        let lon_microdeg = i32::from_be_bytes(lon_bytes);
        let battery_level = raw[8];

        Ok(Self {
            latitude_deg: lat_microdeg as f32 / 1_000_000.0,
            longitude_deg: lon_microdeg as f32 / 1_000_000.0,
            battery_level,
        })
    }
}

fn main() {
    // Buenos Aires-ish scordenadas de ejemplo
    let lat_microdeg: i32 = -349_999_999; // -34.0 approx
    let lon_microdeg: i32 = -580_000_001; // -58.0 approx
    let battery: u8 = 87;

    let mut frame_bytes = Vec::new();
    frame_bytes.extend_from_slice(&lat_microdeg.to_be_bytes());
    frame_bytes.extend_from_slice(&lon_microdeg.to_be_bytes());
    frame_bytes.push(battery);

    let parsed = TelemetryFrame::from_bytes(&frame_bytes).expect("parse should succeed");

    assert!((parsed.latitude_deg + 34.0).abs() < 0.001);
    assert!((parsed.longitude_deg + 58.0).abs() < 0.001);
    assert_eq!(parsed.battery_level, battery);

    println!("OK - parsed frame: {:?}", parsed);
}
