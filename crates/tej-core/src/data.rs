use rand::RngCore;

const BUFFER_SIZE: usize = 1024 * 1024; // 1MB

/// Pre-generates a 1MB buffer of incompressible random data.
/// This buffer is cycled for larger payloads to avoid CPU overhead
/// from continuous random generation (important on mobile).
pub fn generate_random_buffer() -> Vec<u8> {
    let mut buf = vec![0u8; BUFFER_SIZE];
    rand::thread_rng().fill_bytes(&mut buf);
    buf
}

/// Creates a Vec of random data of the specified size by cycling the 1MB buffer.
pub fn random_payload(size: usize) -> Vec<u8> {
    let base = generate_random_buffer();
    let mut payload = Vec::with_capacity(size);
    while payload.len() < size {
        let remaining = size - payload.len();
        let chunk = remaining.min(base.len());
        payload.extend_from_slice(&base[..chunk]);
    }
    payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_buffer_size() {
        let buf = generate_random_buffer();
        assert_eq!(buf.len(), BUFFER_SIZE);
    }

    #[test]
    fn test_random_buffer_not_all_zeros() {
        let buf = generate_random_buffer();
        assert!(buf.iter().any(|&b| b != 0));
    }

    #[test]
    fn test_random_payload_exact_size() {
        let payload = random_payload(5_000_000);
        assert_eq!(payload.len(), 5_000_000);
    }

    #[test]
    fn test_random_payload_small() {
        let payload = random_payload(100);
        assert_eq!(payload.len(), 100);
    }
}
