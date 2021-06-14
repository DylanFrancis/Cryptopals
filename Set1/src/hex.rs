use crate::base64::encode_to_base64;
use std::collections::HashMap;

pub fn hex_to_bits(to_decode: &str) -> Vec<u8> {
    let mut binary : Vec<u8> = Vec::new();

    to_decode.chars()
        .map(|c| c.to_digit(16).unwrap())
        .map(|d| to_binary(&d))
        .for_each(|bits| {
            for bit in &bits {
                binary.push(*bit);
            }
        });

    binary
}

fn to_binary(num: &u32) -> [u8; 4] {
    let mut x = *num;
    let mut arr: [u8; 4] = [0; 4];
    let mut count = 4;

    while x > 0 {
        count -= 1;
        arr[count] = (x % 2) as u8;
        x = &x / 2;
    }

    arr
}
