// fast-itoa/examples/roundtrip.rs

use fast_itoa;
use std::env;

fn main() {
    let string = env::args().nth(1).expect("usage: roundtrip NUMBER");
    let number: u64 = str::parse(&string).unwrap();
    let mut result = [0; 32];
    let size = fast_itoa::fmt_u64(&mut result, number);
    println!("{}", String::from_utf8(result[0..size].to_vec()).unwrap());
}
