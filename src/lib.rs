use std::{
    arch::x86_64::{
        __m128i, _mm_adds_epu8, _mm_cmpeq_epi16, _mm_cmpeq_epi8, _mm_extract_epi16,
        _mm_extract_epi64, _mm_movemask_epi8, _mm_mul_epi32, _mm_mul_epu32, _mm_mulhi_epu16,
        _mm_mullo_epi16, _mm_packs_epi32, _mm_set1_epi16, _mm_set1_epi32, _mm_set1_epi64x,
        _mm_set1_epi8, _mm_set_epi16, _mm_set_epi32, _mm_setzero_si128, _mm_shuffle_pd,
        _mm_srl_epi16, _mm_srl_epi64, _mm_sub_epi16, _mm_sub_epi64, _mm_unpackhi_pd,
        _mm_unpackhi_ps, _mm_unpacklo_ps,
    },
    mem, ptr,
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

#[inline]
fn unpacklo_pi32(a: __m128i, b: __m128i) -> __m128i {
    unsafe { mem::transmute(_mm_unpacklo_ps(mem::transmute(a), mem::transmute(b))) }
}

// FIXME: doesn't work with numbers >= 10**16 yet, needs another loop
#[inline(never)]
pub fn fmt_u64_nolut(dest: &mut [u8; 32], x: u64) -> usize {
    unsafe {
        let halves_8 = _mm_set_epi32(0, (x / 1_0000_0000) as i32, 0, (x % 1_0000_0000) as i32);

        // divide by 1000
        let products_8 = _mm_mul_epu32(halves_8, _mm_set1_epi32(-776530087));
        let quotients_4 = _mm_srl_epi64(products_8, _mm_set1_epi64x(45));
        let remainders_4 =
            _mm_sub_epi64(halves_8, _mm_mul_epi32(quotients_4, _mm_set1_epi32(1_0000)));
        //let blended_4 = unpacklo_pi32(quotients_4, remainders_4);
        let blended_4 = _mm_set_epi32(
            _mm_extract_epi64::<1>(quotients_4) as i32,
            _mm_extract_epi64::<1>(remainders_4) as i32,
            _mm_extract_epi64::<0>(quotients_4) as i32,
            _mm_extract_epi64::<0>(remainders_4) as i32,
        );

        // divide by 100
        // TODO: only using half the lanes here
        let products_4 = _mm_mulhi_epu16(
            _mm_srl_epi16(blended_4, _mm_set_epi32(0, 0, 0, 2)),
            _mm_set1_epi16(5243),
        );
        let quotients_2 = _mm_srl_epi16(products_4, _mm_set_epi32(0, 0, 0, 1));
        let remainders_2 = _mm_sub_epi16(
            products_4,
            _mm_mullo_epi16(quotients_2, _mm_set1_epi16(100)),
        );

        // divide by 10
        let products_2 = _mm_mullo_epi16(remainders_2, _mm_set1_epi16(205));
        let quotients_1 = _mm_srl_epi16(products_2, _mm_set_epi32(0, 0, 0, 11));
        let remainders_1 =
            _mm_sub_epi16(products_2, _mm_mullo_epi16(quotients_1, _mm_set1_epi16(10)));
        let blended_1 = _mm_set_epi16(
            _mm_extract_epi16::<3>(quotients_1) as i16,
            _mm_extract_epi16::<3>(remainders_1) as i16,
            _mm_extract_epi16::<2>(quotients_1) as i16,
            _mm_extract_epi16::<2>(remainders_1) as i16,
            _mm_extract_epi16::<1>(quotients_1) as i16,
            _mm_extract_epi16::<1>(remainders_1) as i16,
            _mm_extract_epi16::<0>(quotients_1) as i16,
            _mm_extract_epi16::<0>(remainders_1) as i16,
        );

        // figure out how many digits we have
        let length = _mm_movemask_epi8(_mm_cmpeq_epi8(blended_1, _mm_setzero_si128()))
            .leading_zeros();

        // convert to ascii and store
        let chars = _mm_adds_epu8(blended_1, _mm_set1_epi8(b'0' as i8));
        ptr::copy_nonoverlapping(&chars as *const __m128i as *const u8, &mut dest[0], 16);

        length as usize
    }
}

#[inline(never)]
pub fn fmt_u64_nosimd(dest: &mut [u8; 32], x: u64) -> usize {
    let mut index = 0;

    let halves = [x / 1_0000_0000, x % 1_0000_0000];
    for half in halves {
        let quarters = [half / 1_0000, half % 1_0000];
        for chunk in quarters {
            unsafe {
                let (length, piece) = LUT[chunk as usize];
                *(((&mut dest[0]) as *mut u8).offset(index as isize) as *mut u32) = piece;
                index += length as usize;
            }
        }
    }

    if index == 0 {
        dest[0] = b'0';
        return 1;
    }

    index
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
