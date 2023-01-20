// fast-itoa/src/lib.rs

use std::arch::x86_64::{
    __m128i, _mm_cmpgt_epi8, _mm_extract_epi64, _mm_i32gather_epi32, _mm_movemask_epi8,
    _mm_mul_epi32, _mm_mul_epu32, _mm_set1_epi32, _mm_set1_epi64x, _mm_set1_epi8, _mm_set_epi32,
    _mm_shuffle_epi8, _mm_srl_epi64, _mm_sub_epi64,
};
use std::mem;
use std::ptr;

use crate::lut::LUT;

#[path = "itoa.rs"]
pub mod rapidjson;

mod lut;
mod test;

static SHUFFLES: [u8; 32] = [
    0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

const SUBCHUNK_SIZE: u64 = 1_0000;
const CHUNK_SIZE: u64 = 1_0000_0000;

#[inline(never)]
pub fn format_u64(dest: &mut [u8; 32], mut value: u64) -> usize {
    if value == 0 {
        dest[0] = b'0';
        return 1;
    }

    unsafe {
        if value < CHUNK_SIZE {
            return format_chunk(dest.as_mut_ptr(), 0, value as i32);
        }
        if value < CHUNK_SIZE * CHUNK_SIZE {
            return format_chunk(
                dest.as_mut_ptr(),
                (value / CHUNK_SIZE) as i32,
                (value % CHUNK_SIZE) as i32,
            );
        }

        let len_0 = format_chunk(
            dest.as_mut_ptr(),
            0,
            (value / (CHUNK_SIZE * CHUNK_SIZE)) as i32,
        );
        value %= CHUNK_SIZE * CHUNK_SIZE;
        let len_1 = format_chunk(
            dest.as_mut_ptr().offset(len_0 as isize),
            (value / CHUNK_SIZE) as i32,
            (value % CHUNK_SIZE) as i32,
        );
        len_0 + len_1
    }
}

unsafe fn format_chunk(dest: *mut u8, hi: i32, lo: i32) -> usize {
    let halves = _mm_set_epi32(0, hi, 0, lo);
    let quotients = _mm_srl_epi64(
        _mm_mul_epu32(halves, _mm_set1_epi32(-776530087)),
        _mm_set1_epi64x(45),
    );
    let remainders = _mm_sub_epi64(
        halves,
        _mm_mul_epi32(quotients, _mm_set1_epi32(SUBCHUNK_SIZE as i32)),
    );

    let data = _mm_i32gather_epi32::<4>(
        &LUT[0] as *const u32 as *const i32,
        _mm_set_epi32(
            _mm_extract_epi64::<1>(quotients) as i32,
            _mm_extract_epi64::<1>(remainders) as i32,
            _mm_extract_epi64::<0>(quotients) as i32,
            _mm_extract_epi64::<0>(remainders) as i32,
        ),
    );

    let mask = _mm_movemask_epi8(_mm_cmpgt_epi8(data, _mm_set1_epi8(b'0' as i8))) as u32;
    let leading = (mask as u16).leading_zeros() as usize;

    let shuffle =
        *mem::transmute::<*const u8, *const __m128i>(SHUFFLES.as_ptr().offset(leading as isize));
    let digits = _mm_shuffle_epi8(data, shuffle);
    ptr::copy_nonoverlapping(&digits as *const __m128i as *const u8, dest, 16);

    16 - leading
}

#[inline(never)]
pub fn format_u64_simple(dest: &mut [u8; 32], mut x: u64) -> usize {
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
