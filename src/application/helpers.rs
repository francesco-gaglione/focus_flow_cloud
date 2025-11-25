use rand::Rng;

pub fn random_hex_color() -> String {
    let mut rng = rand::thread_rng();
    let color: u32 = rng.gen_range(0..0xFFFFFF);
    format!("#{:06X}", color)
}
