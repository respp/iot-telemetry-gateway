fn decode_hex(input: &str) -> Result<Vec<u8>, String> {
    hex::decode(input).map_err(|e| format!("invalid hex: {e}"))
}

fn main() {
    let payload = "0102FF";
    let bytes = decode_hex(payload).expect("decode should work");

    assert_eq!(bytes, vec![0x01, 0x02, 0xFF]);

    println!("OK - decoded bytes: {:?}", bytes);
}
