use std::collections::HashMap;

use crate::base64::encode_to_base64;
use crate::util::basic;

pub fn hex_to_ascii(hex: &String) -> String {
    let bytes = hex_to_bytes(hex.as_str());

    let result = bytes.iter()
        .map(|byte| basic::to_decimal_8bit_binary(&byte))
        .map(|d| char::from(d))
        .collect();

    result
}

pub fn xor(hex1: &String, hex2: &String) -> String {
    let hex1_bits = hex_to_nibbles(hex1.as_str());
    let hex2_bits = hex_to_nibbles(hex2.as_str());

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

fn hex_to_bytes(to_bytes: &str) -> Vec<[u8; 8]> {
    let bytes = to_bytes.chars()
        .step_by(2)
        .zip(to_bytes.chars().skip(1).step_by(2))
        .map(|(x, y)| (x.to_digit(16).unwrap(), y.to_digit(16).unwrap()))
        .map(|(x, y)| (to_binary(&x), to_binary(&y)))
        .map(|(x, y)| concatenate_arrays(x, y))
        .collect();

    bytes
}

fn hex_to_nibbles(to_nibbles: &str) -> Vec<[u8; 4]> {
    let mut nibbles: Vec<[u8; 4]> = Vec::new();

    to_nibbles.chars()
        .map(|c| c.to_digit(16).unwrap())
        .map(|d| to_binary(&d))
        .for_each(|bits| nibbles.push(bits));

    nibbles
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

fn concatenate_arrays(nibble1: [u8; 4], nibble2: [u8; 4]) -> [u8; 8] {
    let mut arr: [u8; 8] = [0; 8];
    let mut count: usize = 0;

    for x in &nibble1 {
        arr[count] = *x;
        count += 1;
    }

    for y in &nibble2 {
        arr[count] = *y;
        count += 1;
    }

    arr
}

fn to_decimal(hex: char) -> u8 {
    match hex {
        '0'..'9' => hex as u8,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        _ => 23
    }
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
