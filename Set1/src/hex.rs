use std::collections::HashMap;

use crate::base64::encode_to_base64;
use crate::util::basic;

pub fn xor(hex1: &String, hex2: &String) -> String {
    let hex1_bits = hex_to_grouped_bits(hex1.as_str());
    let hex2_bits = hex_to_grouped_bits(hex2.as_str());

    let xor_result = do_xor(&hex1_bits, &hex2_bits);

    bits_to_hex(&xor_result)
}

fn do_xor(hex1: &Vec<[u8; 4]>, hex2: &Vec<[u8; 4]>) -> Vec<[u8; 4]> {
    let mut result: Vec<[u8; 4]> = Vec::new();
    let max;

    if hex1.len() <= hex2.len() {
        max = hex1.len();
    } else {
        max = hex2.len();
    }

    for index in 0..max {
        let nibble1 = hex1.get(index).unwrap();
        let nibble2 = hex2.get(index).unwrap();
        let mut res: [u8; 4] = [0; 4];

        for x in 0..4 {
            res[x] = match (nibble1[x], nibble2[x]) {
                (1, 1) => 0,
                (1, 0) => 1,
                (0, 1) => 1,
                (0, 0) => 0,
                (_, _) => panic!("{:?} {:?}", nibble1, nibble2)
            }
        }

        result.push(res);
    }

    result
}

fn hex_to_grouped_bits(to_bits: &str) -> Vec<[u8; 4]> {
    let mut binary : Vec<[u8; 4]> = Vec::new();

    to_bits.chars()
        .map(|c| c.to_digit(16).unwrap())
        .map(|d| to_binary(&d))
        .for_each(|bits| binary.push(bits));

    binary
}

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

pub fn bits_to_hex(to_encode: &Vec<[u8; 4]>) -> String {
    let mut hex= String::new();

    to_encode.iter()
        .map(|arr| to_char(&arr))
        .for_each(|char| hex.push(char));

    hex
}

fn to_char(bits: &[u8; 4]) -> char {
    let value = basic::to_decimal_4bit_binary(&bits);

    let char = match value {
        0..10 => char::from_digit(value as u32, 10).unwrap(),
        10 => 'A',
        11 => 'B',
        12 => 'C',
        13 => 'D',
        14 => 'E',
        15 => 'F',
        _ => ' '
    };

    char
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
