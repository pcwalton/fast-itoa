// fast-itoa/examples/roundtrip.rs

use fast_itoa;
use std::env;

fn main() {
    let string = env::args().nth(1).expect("usage: roundtrip NUMBER");
    let number: u64 = str::parse(&string).unwrap();
    let (mut fast_result, mut simple_result) = ([0; 32], [0; 32]);
    let fast_size = fast_itoa::format_u64(&mut fast_result, number);
    let simple_size = fast_itoa::format_u64_simple(&mut simple_result, number);
    println!(
        "{} {}",
        String::from_utf8(fast_result[0..fast_size].to_vec()).unwrap(),
        String::from_utf8(simple_result[0..simple_size].to_vec()).unwrap()
    );
}
