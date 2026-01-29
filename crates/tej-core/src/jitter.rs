/// Calculate jitter as the mean absolute difference between consecutive samples (RFC 3550).
pub fn calculate_jitter(samples: &[f64]) -> f64 {
    if samples.len() < 2 {
        return 0.0;
    }

    let sum: f64 = samples.windows(2).map(|w| (w[1] - w[0]).abs()).sum();

    sum / (samples.len() - 1) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jitter_empty() {
        assert_eq!(calculate_jitter(&[]), 0.0);
    }

    #[test]
    fn test_jitter_single() {
        assert_eq!(calculate_jitter(&[5.0]), 0.0);
    }

    #[test]
    fn test_jitter_constant() {
        assert_eq!(calculate_jitter(&[10.0, 10.0, 10.0]), 0.0);
    }

    #[test]
    fn test_jitter_known_values() {
        // |11-10| + |9-11| + |10-9| = 1 + 2 + 1 = 4, / 3 = 1.333...
        let jitter = calculate_jitter(&[10.0, 11.0, 9.0, 10.0]);
        assert!((jitter - 1.3333).abs() < 0.01);
    }
}
