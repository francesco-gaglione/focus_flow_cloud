use rand::Rng;

pub fn random_hex_color() -> String {
    let mut rng = rand::thread_rng();
    let color: u32 = rng.gen_range(0..0xFFFFFF);
    format!("#{:06X}", color)
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
}
