// fast-itoa/examples/roundtrip.rs

use fast_itoa;
use std::env;

fn main() {
    let string = env::args().nth(1).expect("usage: roundtrip NUMBER");
    let number: u64 = str::parse(&string).unwrap();
    let (mut fast_result, mut nolut_result, mut nosimd_result, mut simple_result) =
        ([0; 32], [0; 32], [0; 32], [0; 32]);
    let fast_size = fast_itoa::fmt_u64(&mut fast_result, number);
    let nolut_size = fast_itoa::fmt_u64_nolut(&mut nolut_result, number);
    let nosimd_size = fast_itoa::fmt_u64_nosimd(&mut nosimd_result, number);
    let simple_size = fast_itoa::fmt_u64_simple(&mut simple_result, number);
    println!(
        "{} {} {} {}",
        String::from_utf8(fast_result[0..fast_size].to_vec()).unwrap(),
        String::from_utf8(nolut_result[0..nolut_size].to_vec()).unwrap(),
        String::from_utf8(nosimd_result[0..nosimd_size].to_vec()).unwrap(),
        String::from_utf8(simple_result[0..simple_size].to_vec()).unwrap()
    );
}
