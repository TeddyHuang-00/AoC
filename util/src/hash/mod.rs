use seq_macro::seq;

pub mod md5;

#[must_use]
#[allow(
    clippy::identity_op,
    clippy::erasing_op,
    reason = "False positive clippy warnings"
)]
pub const fn digest16_to_hex_byte(digest: [u8; 16]) -> [u8; 32] {
    let mut result = [0u8; 32];
    seq! {
        N in 0..16 {
            result[2 * N] = digest[N] >> 4;
            result[2 * N + 1] = digest[N] & 0xf;
        }
    }
    result
}

#[must_use]
pub const fn digest16_to_hex_char(digest: [u8; 16]) -> [u8; 32] {
    let hex = digest16_to_hex_byte(digest);
    let mut result = [0u8; 32];
    seq! {
        N in 0..32 {
            result[N] = match hex[N] {
                0..=9 => b'0' + hex[N],
                10..=15 => b'a' + hex[N] - 10,
                _ => unreachable!(),
            };
        }
    }
    result
}
