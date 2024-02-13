pub fn random(strength_divider: u8) -> u8 {
    if strength_divider == 0 {
        return 128;
    }
    return rand::random::<u8>() / strength_divider + 128
}

pub fn metal(strength_divider: u8, size: u32, x: u32, y: u32) -> u8 {
    if strength_divider == 0 {
        return 128;
    }
    let mut val = (x + y) % size;
    if val > size/2 {
        val = size - val;
    }

    return ((val * 255/size) as u8) / strength_divider + 128
}
