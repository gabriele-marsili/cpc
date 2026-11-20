//! The k of the query: k = ⌊log2 len⌋ = position of the most significant bit of len.
//! A student asked: "isn't computing the logarithm logarithmic?". The prof: no — the CPU does it in constant
//! time (Rust: `leading_zeros`, which compiles to `lzcnt`/`clz`); without it, a TABLE: split the 32 bits in
//! 4 bytes, a table with the 256 answers for one byte, a loop of 4 iterations. He also pointed to the
//! bit tricks in Knuth's TAOCP (he thought volume 4), one based on de Bruijn sequences.

/// msb with the CPU instruction. `x > 0`.
pub fn msb_cpu(x: u32) -> u32 {
    31 - x.leading_zeros()
}

/// The prof's tabulation: MSB_TABLE[b] = msb of the byte b (unused for b = 0).
pub const MSB_TABLE: [u8; 256] = {
    let mut t = [0u8; 256];
    let mut b = 2;
    while b < 256 {
        t[b] = t[b / 2] + 1;
        b += 1;
    }
    t
};

/// msb with the 256-entry table: look at the bytes from the most significant one. `x > 0`.
pub fn msb_table(x: u32) -> u32 {
    for byte in (0..4).rev() {
        let v = (x >> (8 * byte)) & 0xFF;
        if v != 0 {
            return 8 * byte + MSB_TABLE[v as usize] as u32;
        }
    }
    panic!("msb of 0 is undefined");
}

/// [extra] the de Bruijn trick (the prof only named it): round x up to 2^(msb+1) - 1 by smearing the bits,
/// multiply by a de Bruijn constant, the top 5 bits index a table of 32 entries.
pub fn msb_debruijn(mut x: u32) -> u32 {
    const DEBRUIJN: u32 = 0x07C4_ACDD;
    const TABLE: [u32; 32] = [
        0, 9, 1, 10, 13, 21, 2, 29, 11, 14, 16, 18, 22, 25, 3, 30, 8, 12, 20, 28, 15, 17, 24, 7, 19, 27, 23, 6,
        26, 5, 4, 31,
    ];
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    TABLE[(x.wrapping_mul(DEBRUIJN) >> 27) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_ways_agree() {
        for x in (1u32..70_000).chain([u32::MAX, 1 << 31, (1 << 31) + 5, 123_456_789]) {
            assert_eq!(msb_table(x), msb_cpu(x), "x={x}");
            assert_eq!(msb_debruijn(x), msb_cpu(x), "x={x}");
        }
        assert_eq!(msb_cpu(7), 2); // the lecture's query: length 7 -> k = 2
        assert_eq!(msb_cpu(15), 3); // the student's example: length 15 -> column k = 3
    }
}
