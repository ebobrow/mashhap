pub fn seahash(src: &str) -> u32 {
    seahash::hash(src.as_bytes()) as u32
}

pub fn fnv_1a(src: &str) -> u32 {
    src.chars().fold(2166136261, |acc, c| {
        (acc ^ (c as u32)).wrapping_mul(16777619)
    })
}

pub fn murmurhash3(src: &str) -> u32 {
    do_murmurhash3(src, 0)
}
fn do_murmurhash3(key: &str, seed: u32) -> u32 {
    let c1 = 0xcc9e2d51;
    let c2 = 0x1b873593;
    let r1 = 15;
    let r2 = 13;
    let m = 5;
    let n = 0xe6546b64;

    let mut hash = seed;

    for chunk in key.as_bytes().chunks(4) {
        if chunk.len() == 4 {
            let mut k = u32::from_ne_bytes(chunk.try_into().unwrap());
            k = k.wrapping_mul(c1);
            k = k.rotate_left(r1);
            k = k.wrapping_mul(c2);

            hash ^= k;
            hash = hash.rotate_left(r2);
            hash = hash.wrapping_mul(m).wrapping_add(n);
        } else {
            let mut padded_chunk = [0].repeat(4 - chunk.len());
            padded_chunk.extend(chunk);
            let mut remaining_bytes = u32::from_ne_bytes(padded_chunk.try_into().unwrap()).to_le();
            remaining_bytes = remaining_bytes.wrapping_mul(c1);
            remaining_bytes = remaining_bytes.rotate_left(r1);
            remaining_bytes = remaining_bytes.wrapping_mul(c2);

            hash ^= remaining_bytes;
        }
    }
    hash ^= key.len() as u32;

    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x85ebca6b);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0xc2b2ae35);
    hash ^= hash >> 16;
    hash
}
