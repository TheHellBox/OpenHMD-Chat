use rand::random;

pub fn rand_string(len: u32) -> String {
    (0..len).map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char).collect()
}
