//! This module implements the MD5 hashing algorithm in Rust.
//!
//! All numbers are in little-endian format.
//!
//! References:
//! - [Wikipedia](https://en.wikipedia.org/wiki/MD5#Pseudocode)

const SHIFTS: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];
#[allow(clippy::unreadable_literal)]
const CONSTANTS: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

#[allow(clippy::many_single_char_names)]
#[must_use]
pub fn digest(message: &[u8]) -> [u8; 16] {
    // Initialize variables
    #[allow(clippy::unreadable_literal)]
    let (mut a0, mut b0, mut c0, mut d0) =
        (0x67452301u32, 0xefcdab89u32, 0x98badcfeu32, 0x10325476u32);
    // Pre-processing the message
    let message = {
        let mut m = message.to_vec();
        // Push the bit '1' to the message,
        // as well as the necessary number of '0' bits
        m.push(0x80);
        // Calculate how many '0' bits to add
        // so that the number of bits is congruent to 448 mod 512;
        // i.e., the number of bytes is congruent to 56 mod 64.
        let padding_length = (64 + 56 - (m.len() % 64)) % 64;
        m.extend(vec![0; padding_length]);
        // Append the original length of the message (in bits) as a 64-bit
        // little-endian integer
        m.extend_from_slice(&((message.len() as u64) * 8).to_le_bytes());
        m
    };
    message.as_chunks::<64>().0.iter().for_each(|chunk| {
        let words = chunk
            .as_chunks::<4>()
            .0
            .iter()
            .copied()
            .map(u32::from_le_bytes)
            .collect::<Vec<_>>();
        // Initialize hash value for this chunk:
        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;
        // Main loop
        for i in 0..64 {
            let (f, g) = match i {
                0..16 => ((b & c) | ((!b) & d), i),
                16..32 => ((d & b) | ((!d) & c), (5 * i + 1) % 16),
                32..48 => (b ^ c ^ d, (3 * i + 5) % 16),
                48..64 => (c ^ (b | (!d)), (7 * i) % 16),
                _ => unreachable!(),
            };
            let f = f
                .wrapping_add(a)
                .wrapping_add(CONSTANTS[i])
                .wrapping_add(words[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(SHIFTS[i]));
        }
        // Add this chunk's hash to result so far
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    });
    [
        a0.to_le_bytes(),
        b0.to_le_bytes(),
        c0.to_le_bytes(),
        d0.to_le_bytes(),
    ]
    .concat()
    .try_into()
    .unwrap_or_else(|_| unreachable!("digest must have 16 bytes"))
}
