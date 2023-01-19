use std::arch::x86_64::{
    _mm_extract_epi64, _mm_mul_epi32, _mm_mul_epu32, _mm_set1_epi32, _mm_set1_epi64x,
    _mm_set_epi32, _mm_srl_epi64, _mm_sub_epi64,
};

use crate::lut::LUT;

mod lut;

// FIXME: doesn't work with numbers >= 10**16 yet, needs another loop
#[inline(never)]
pub fn fmt_u64(dest: &mut [u8; 32], x: u64) -> usize {
    unsafe {
        let halves = _mm_set_epi32(0, (x / 1_0000_0000) as i32, 0, (x % 1_0000_0000) as i32);
        let magics = _mm_set1_epi32(-776530087);
        let products = _mm_mul_epu32(halves, magics);
        let quotients = _mm_srl_epi64(products, _mm_set1_epi64x(45));
        let remainders = _mm_sub_epi64(halves, _mm_mul_epi32(quotients, _mm_set1_epi32(1_0000)));

        let mut index = 0;
        for chunk in [
            _mm_extract_epi64::<1>(quotients),
            _mm_extract_epi64::<1>(remainders),
            _mm_extract_epi64::<0>(quotients),
            _mm_extract_epi64::<0>(remainders),
        ] {
            if chunk != 0 {
                let (length, piece) = LUT[chunk as usize];
                *(((&mut dest[0]) as *mut u8).offset(index as isize) as *mut u32) = piece;
                index += length as usize;
            }
        }

        if index == 0 {
            dest[0] = b'0';
            return 1;
        }

        index
    }
}

#[inline(never)]
pub fn fmt_u64_simple(dest: &mut [u8; 32], mut x: u64) -> usize {
    let mut i = 0;
    while x != 0 {
        dest[i] = b'0' + (x % 10) as u8;
        x /= 10;
        i += 1;
    }
    for j in 0..(i / 2) {
        dest.swap(j, i - j - 1);
    }
    i
}
