pub fn seahash(src: &str) -> u32 {
    seahash::hash(src.as_bytes()) as u32
}

pub fn fnv_1a(src: &str) -> u32 {
    src.chars().fold(2166136261, |acc, c| {
        (acc ^ (c as u32)).wrapping_mul(16777619)
    })
}
