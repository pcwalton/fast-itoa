#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use libc;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
#[no_mangle]
pub unsafe extern "C" fn GetDigitsLut() -> *const libc::c_char {
    static mut cDigitsLut: [libc::c_char; 200] = [
        '0' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
    ];
    return cDigitsLut.as_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn u64toa(
    mut value: uint64_t,
    mut buffer: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut cDigitsLut: *const libc::c_char = GetDigitsLut();
    let kTen8: uint64_t = 100000000 as libc::c_int as uint64_t;
    let kTen9: uint64_t = kTen8.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
    let kTen10: uint64_t = kTen8.wrapping_mul(100 as libc::c_int as libc::c_ulonglong);
    let kTen11: uint64_t = kTen8.wrapping_mul(1000 as libc::c_int as libc::c_ulonglong);
    let kTen12: uint64_t = kTen8.wrapping_mul(10000 as libc::c_int as libc::c_ulonglong);
    let kTen13: uint64_t = kTen8
        .wrapping_mul(100000 as libc::c_int as libc::c_ulonglong);
    let kTen14: uint64_t = kTen8
        .wrapping_mul(1000000 as libc::c_int as libc::c_ulonglong);
    let kTen15: uint64_t = kTen8
        .wrapping_mul(10000000 as libc::c_int as libc::c_ulonglong);
    let kTen16: uint64_t = kTen8.wrapping_mul(kTen8);
    if value < kTen8 {
        let mut v: uint32_t = value as uint32_t;
        if v < 10000 as libc::c_int as libc::c_uint {
            let d1: uint32_t = v.wrapping_div(100 as libc::c_int as libc::c_uint)
                << 1 as libc::c_int;
            let d2: uint32_t = v.wrapping_rem(100 as libc::c_int as libc::c_uint)
                << 1 as libc::c_int;
            if v >= 1000 as libc::c_int as libc::c_uint {
                let fresh0 = buffer;
                buffer = buffer.offset(1);
                *fresh0 = *cDigitsLut.offset(d1 as isize);
            }
            if v >= 100 as libc::c_int as libc::c_uint {
                let fresh1 = buffer;
                buffer = buffer.offset(1);
                *fresh1 = *cDigitsLut
                    .offset(d1.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
            }
            if v >= 10 as libc::c_int as libc::c_uint {
                let fresh2 = buffer;
                buffer = buffer.offset(1);
                *fresh2 = *cDigitsLut.offset(d2 as isize);
            }
            let fresh3 = buffer;
            buffer = buffer.offset(1);
            *fresh3 = *cDigitsLut
                .offset(d2.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        } else {
            let b: uint32_t = v.wrapping_div(10000 as libc::c_int as libc::c_uint);
            let c: uint32_t = v.wrapping_rem(10000 as libc::c_int as libc::c_uint);
            let d1_0: uint32_t = b.wrapping_div(100 as libc::c_int as libc::c_uint)
                << 1 as libc::c_int;
            let d2_0: uint32_t = b.wrapping_rem(100 as libc::c_int as libc::c_uint)
                << 1 as libc::c_int;
            let d3: uint32_t = c.wrapping_div(100 as libc::c_int as libc::c_uint)
                << 1 as libc::c_int;
            let d4: uint32_t = c.wrapping_rem(100 as libc::c_int as libc::c_uint)
                << 1 as libc::c_int;
            if value >= 10000000 as libc::c_int as libc::c_ulonglong {
                let fresh4 = buffer;
                buffer = buffer.offset(1);
                *fresh4 = *cDigitsLut.offset(d1_0 as isize);
            }
            if value >= 1000000 as libc::c_int as libc::c_ulonglong {
                let fresh5 = buffer;
                buffer = buffer.offset(1);
                *fresh5 = *cDigitsLut
                    .offset(
                        d1_0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                    );
            }
            if value >= 100000 as libc::c_int as libc::c_ulonglong {
                let fresh6 = buffer;
                buffer = buffer.offset(1);
                *fresh6 = *cDigitsLut.offset(d2_0 as isize);
            }
            let fresh7 = buffer;
            buffer = buffer.offset(1);
            *fresh7 = *cDigitsLut
                .offset(d2_0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
            let fresh8 = buffer;
            buffer = buffer.offset(1);
            *fresh8 = *cDigitsLut.offset(d3 as isize);
            let fresh9 = buffer;
            buffer = buffer.offset(1);
            *fresh9 = *cDigitsLut
                .offset(d3.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
            let fresh10 = buffer;
            buffer = buffer.offset(1);
            *fresh10 = *cDigitsLut.offset(d4 as isize);
            let fresh11 = buffer;
            buffer = buffer.offset(1);
            *fresh11 = *cDigitsLut
                .offset(d4.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        }
    } else if value < kTen16 {
        let v0: uint32_t = value.wrapping_div(kTen8) as uint32_t;
        let v1: uint32_t = value.wrapping_rem(kTen8) as uint32_t;
        let b0: uint32_t = v0.wrapping_div(10000 as libc::c_int as libc::c_uint);
        let c0: uint32_t = v0.wrapping_rem(10000 as libc::c_int as libc::c_uint);
        let d1_1: uint32_t = b0.wrapping_div(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        let d2_1: uint32_t = b0.wrapping_rem(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        let d3_0: uint32_t = c0.wrapping_div(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        let d4_0: uint32_t = c0.wrapping_rem(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        let b1: uint32_t = v1.wrapping_div(10000 as libc::c_int as libc::c_uint);
        let c1: uint32_t = v1.wrapping_rem(10000 as libc::c_int as libc::c_uint);
        let d5: uint32_t = b1.wrapping_div(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        let d6: uint32_t = b1.wrapping_rem(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        let d7: uint32_t = c1.wrapping_div(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        let d8: uint32_t = c1.wrapping_rem(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        if value >= kTen15 {
            let fresh12 = buffer;
            buffer = buffer.offset(1);
            *fresh12 = *cDigitsLut.offset(d1_1 as isize);
        }
        if value >= kTen14 {
            let fresh13 = buffer;
            buffer = buffer.offset(1);
            *fresh13 = *cDigitsLut
                .offset(d1_1.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        }
        if value >= kTen13 {
            let fresh14 = buffer;
            buffer = buffer.offset(1);
            *fresh14 = *cDigitsLut.offset(d2_1 as isize);
        }
        if value >= kTen12 {
            let fresh15 = buffer;
            buffer = buffer.offset(1);
            *fresh15 = *cDigitsLut
                .offset(d2_1.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        }
        if value >= kTen11 {
            let fresh16 = buffer;
            buffer = buffer.offset(1);
            *fresh16 = *cDigitsLut.offset(d3_0 as isize);
        }
        if value >= kTen10 {
            let fresh17 = buffer;
            buffer = buffer.offset(1);
            *fresh17 = *cDigitsLut
                .offset(d3_0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        }
        if value >= kTen9 {
            let fresh18 = buffer;
            buffer = buffer.offset(1);
            *fresh18 = *cDigitsLut.offset(d4_0 as isize);
        }
        let fresh19 = buffer;
        buffer = buffer.offset(1);
        *fresh19 = *cDigitsLut
            .offset(d4_0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        let fresh20 = buffer;
        buffer = buffer.offset(1);
        *fresh20 = *cDigitsLut.offset(d5 as isize);
        let fresh21 = buffer;
        buffer = buffer.offset(1);
        *fresh21 = *cDigitsLut
            .offset(d5.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        let fresh22 = buffer;
        buffer = buffer.offset(1);
        *fresh22 = *cDigitsLut.offset(d6 as isize);
        let fresh23 = buffer;
        buffer = buffer.offset(1);
        *fresh23 = *cDigitsLut
            .offset(d6.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        let fresh24 = buffer;
        buffer = buffer.offset(1);
        *fresh24 = *cDigitsLut.offset(d7 as isize);
        let fresh25 = buffer;
        buffer = buffer.offset(1);
        *fresh25 = *cDigitsLut
            .offset(d7.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        let fresh26 = buffer;
        buffer = buffer.offset(1);
        *fresh26 = *cDigitsLut.offset(d8 as isize);
        let fresh27 = buffer;
        buffer = buffer.offset(1);
        *fresh27 = *cDigitsLut
            .offset(d8.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
    } else {
        let a: uint32_t = value.wrapping_div(kTen16) as uint32_t;
        value = (value as libc::c_ulonglong).wrapping_rem(kTen16) as uint64_t
            as uint64_t;
        if a < 10 as libc::c_int as libc::c_uint {
            let fresh28 = buffer;
            buffer = buffer.offset(1);
            *fresh28 = ('0' as i32 + a as libc::c_char as libc::c_int) as libc::c_char;
        } else if a < 100 as libc::c_int as libc::c_uint {
            let i: uint32_t = a << 1 as libc::c_int;
            let fresh29 = buffer;
            buffer = buffer.offset(1);
            *fresh29 = *cDigitsLut.offset(i as isize);
            let fresh30 = buffer;
            buffer = buffer.offset(1);
            *fresh30 = *cDigitsLut
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        } else if a < 1000 as libc::c_int as libc::c_uint {
            let fresh31 = buffer;
            buffer = buffer.offset(1);
            *fresh31 = ('0' as i32
                + a.wrapping_div(100 as libc::c_int as libc::c_uint) as libc::c_char
                    as libc::c_int) as libc::c_char;
            let i_0: uint32_t = a.wrapping_rem(100 as libc::c_int as libc::c_uint)
                << 1 as libc::c_int;
            let fresh32 = buffer;
            buffer = buffer.offset(1);
            *fresh32 = *cDigitsLut.offset(i_0 as isize);
            let fresh33 = buffer;
            buffer = buffer.offset(1);
            *fresh33 = *cDigitsLut
                .offset(i_0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        } else {
            let i_1: uint32_t = a.wrapping_div(100 as libc::c_int as libc::c_uint)
                << 1 as libc::c_int;
            let j: uint32_t = a.wrapping_rem(100 as libc::c_int as libc::c_uint)
                << 1 as libc::c_int;
            let fresh34 = buffer;
            buffer = buffer.offset(1);
            *fresh34 = *cDigitsLut.offset(i_1 as isize);
            let fresh35 = buffer;
            buffer = buffer.offset(1);
            *fresh35 = *cDigitsLut
                .offset(i_1.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
            let fresh36 = buffer;
            buffer = buffer.offset(1);
            *fresh36 = *cDigitsLut.offset(j as isize);
            let fresh37 = buffer;
            buffer = buffer.offset(1);
            *fresh37 = *cDigitsLut
                .offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        }
        let v0_0: uint32_t = value.wrapping_div(kTen8) as uint32_t;
        let v1_0: uint32_t = value.wrapping_rem(kTen8) as uint32_t;
        let b0_0: uint32_t = v0_0.wrapping_div(10000 as libc::c_int as libc::c_uint);
        let c0_0: uint32_t = v0_0.wrapping_rem(10000 as libc::c_int as libc::c_uint);
        let d1_2: uint32_t = b0_0.wrapping_div(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        let d2_2: uint32_t = b0_0.wrapping_rem(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        let d3_1: uint32_t = c0_0.wrapping_div(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        let d4_1: uint32_t = c0_0.wrapping_rem(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        let b1_0: uint32_t = v1_0.wrapping_div(10000 as libc::c_int as libc::c_uint);
        let c1_0: uint32_t = v1_0.wrapping_rem(10000 as libc::c_int as libc::c_uint);
        let d5_0: uint32_t = b1_0.wrapping_div(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        let d6_0: uint32_t = b1_0.wrapping_rem(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        let d7_0: uint32_t = c1_0.wrapping_div(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        let d8_0: uint32_t = c1_0.wrapping_rem(100 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        let fresh38 = buffer;
        buffer = buffer.offset(1);
        *fresh38 = *cDigitsLut.offset(d1_2 as isize);
        let fresh39 = buffer;
        buffer = buffer.offset(1);
        *fresh39 = *cDigitsLut
            .offset(d1_2.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        let fresh40 = buffer;
        buffer = buffer.offset(1);
        *fresh40 = *cDigitsLut.offset(d2_2 as isize);
        let fresh41 = buffer;
        buffer = buffer.offset(1);
        *fresh41 = *cDigitsLut
            .offset(d2_2.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        let fresh42 = buffer;
        buffer = buffer.offset(1);
        *fresh42 = *cDigitsLut.offset(d3_1 as isize);
        let fresh43 = buffer;
        buffer = buffer.offset(1);
        *fresh43 = *cDigitsLut
            .offset(d3_1.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        let fresh44 = buffer;
        buffer = buffer.offset(1);
        *fresh44 = *cDigitsLut.offset(d4_1 as isize);
        let fresh45 = buffer;
        buffer = buffer.offset(1);
        *fresh45 = *cDigitsLut
            .offset(d4_1.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        let fresh46 = buffer;
        buffer = buffer.offset(1);
        *fresh46 = *cDigitsLut.offset(d5_0 as isize);
        let fresh47 = buffer;
        buffer = buffer.offset(1);
        *fresh47 = *cDigitsLut
            .offset(d5_0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        let fresh48 = buffer;
        buffer = buffer.offset(1);
        *fresh48 = *cDigitsLut.offset(d6_0 as isize);
        let fresh49 = buffer;
        buffer = buffer.offset(1);
        *fresh49 = *cDigitsLut
            .offset(d6_0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        let fresh50 = buffer;
        buffer = buffer.offset(1);
        *fresh50 = *cDigitsLut.offset(d7_0 as isize);
        let fresh51 = buffer;
        buffer = buffer.offset(1);
        *fresh51 = *cDigitsLut
            .offset(d7_0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        let fresh52 = buffer;
        buffer = buffer.offset(1);
        *fresh52 = *cDigitsLut.offset(d8_0 as isize);
        let fresh53 = buffer;
        buffer = buffer.offset(1);
        *fresh53 = *cDigitsLut
            .offset(d8_0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
    }
    return buffer;
}
