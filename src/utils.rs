#![allow(non_snake_case)]

/// Right Cyclic Shift
#[inline(always)]
pub fn S(chunk: u32, shift_by: u32) -> u32 {
    chunk.rotate_right(shift_by)
}

/// Right Logical Shift
#[inline(always)]
pub fn R(chunk: u32, shift_by: usize) -> u32 {
    (chunk & 0xFFFFFFFF) >> shift_by
}

#[inline(always)]
pub fn sigma0(chunk: u32) -> u32 {
    S(chunk, 2) ^ S(chunk, 13) ^ S(chunk, 22)
}

#[inline(always)]
pub fn sigma1(chunk: u32) -> u32 {
    S(chunk, 6) ^ S(chunk, 11) ^ S(chunk, 25)
}

#[inline(always)]
pub fn gamma0(chunk: u32) -> u32 {
    S(chunk, 7) ^ S(chunk, 18) ^ R(chunk, 3)
}

#[inline(always)]
pub fn gamma1(chunk: u32) -> u32 {
    S(chunk, 17) ^ S(chunk, 19) ^ R(chunk, 10)
}

/// (A | F) XOR (-A | B)
#[inline(always)]
pub fn ch(chunks: [u32; 3]) -> u32 {
    chunks[2] ^ (chunks[0] & (chunks[1] ^ chunks[2]))
}

/// (A | B) XOR (A | C) XOR (B | C)
#[inline(always)]
pub fn maj(chunks: [u32; 3]) -> u32 {
    ((chunks[0] | chunks[1]) & chunks[2]) | (chunks[0] & chunks[1])
}

/*#[inline(always)]
pub fn compose_4bytes(bytes: [u8; 4]) -> u32 {
    (bytes[0] as u32 & 0xFF) << 24 | (bytes[1] as u32 & 0xFF) << 16 | (bytes[2] as u32 & 0xFF) << 8 | bytes[3] as u32 & 0xFF
}

pub fn decompose_4bytes(val: u32) -> [u8; 4] {
    let mut decomposed = [0u8; 4];

    decomposed[0] = ((val >> 24) & 0xFF) as u8;
    decomposed[1] = ((val >> 16) & 0xFF) as u8;
    decomposed[2] = ((val >> 8) & 0xFF) as u8;
    decomposed[3] = (val & 0xFF) as u8;

    decomposed
}
*/