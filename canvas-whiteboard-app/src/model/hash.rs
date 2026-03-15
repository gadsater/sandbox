use sha2::{Digest, Sha256};

/// Generate a short hash-based reference id from arbitrary seed bytes.
/// Returns a hex-encoded 16-character (64-bit) prefix of SHA-256.
pub fn generate_id(seed: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    let result = hasher.finalize();
    hex::encode(&result[..8]) // 16 hex chars
}

/// Convenience: generate an id from a name + current timestamp-like counter.
/// In wasm we use `js_sys::Date::now()` as the entropy source.
pub fn make_id(name: &str) -> String {
    let ts = timestamp_nanos();
    let seed = format!("{name}:{ts}");
    generate_id(seed.as_bytes())
}

fn timestamp_nanos() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        (js_sys::Date::now() * 1_000_000.0) as u64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_length() {
        let id = generate_id(b"hello");
        assert_eq!(id.len(), 16);
    }

    #[test]
    fn deterministic() {
        assert_eq!(generate_id(b"abc"), generate_id(b"abc"));
    }
}
