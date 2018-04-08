pub fn xorshift32(seed: u32) -> u32 {
    let y = seed ^ (seed << 13);
    let y = y ^ (y >> 17);
    y ^ (y << 15)
}
