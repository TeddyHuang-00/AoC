use std::{
    collections::{HashMap, HashSet},
    hash::{BuildHasherDefault, Hasher},
};

use paste::paste;

pub type FxHash = BuildHasherDefault<FastHasher>;
pub type FxHashMap<K, V> = HashMap<K, V, FxHash>;
pub type FxHashSet<T> = HashSet<T, FxHash>;

const HASH_K: u64 = 0x517c_c1b7_2722_0a95;

macro_rules! hash_primitive_integer {
    (internal $type:ident) => {
        paste! {
            #[inline]
            fn [<write_ $type>](&mut self, i: $type) {
                self.write_u64(i as u64);
            }
        }
    };
    ($($types:ident)+) => {
        $(
            hash_primitive_integer!(internal $types);
        )+
    };
}

#[derive(Clone, Copy, Default)]
pub struct FastHasher(u64);

#[allow(
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    reason = "For hashing only"
)]
impl Hasher for FastHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.write_u64(u64::from(b));
        }
    }

    #[inline]
    fn write_u64(&mut self, n: u64) {
        self.0 = (self.0.rotate_left(5) ^ n).wrapping_mul(HASH_K);
    }

    hash_primitive_integer!(i8 i16 i32 i64 isize);
    hash_primitive_integer!(u8 u16 u32 usize);

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.write_u64(i as u64);
        self.write_u64((i >> 64) as u64);
    }

    #[inline]
    fn write_i128(&mut self, i: i128) {
        self.write_u128(i as u128);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}
