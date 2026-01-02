use rand::Rng;

pub fn random_hex_color() -> String {
    let mut rng = rand::thread_rng();
    let color: u32 = rng.gen_range(0..0xFFFFFF);
    format!("#{:06X}", color)
}

pub fn validate_hex_color(color: &str) -> bool {
    color.len() == 7 && color.starts_with('#') && color[1..].chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_hex_color() {
        let color = random_hex_color();
        assert_eq!(color.len(), 7);
        assert!(color.starts_with('#'));
        assert!(color[1..].chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_validate_hex_color() {
        assert!(validate_hex_color("#FFFFFF"));
        assert!(validate_hex_color("#000000"));
        assert!(validate_hex_color("#123456"));
        assert!(validate_hex_color("#ABCDEF"));
        assert!(!validate_hex_color("#GHIJKL"));
        assert!(!validate_hex_color("#12345"));
        assert!(!validate_hex_color("#1234567"));
        assert!(!validate_hex_color("#12345678"));
    }
}
