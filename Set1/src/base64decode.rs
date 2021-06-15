use std::collections::HashMap;

use crate::util::basic;

pub fn run(word: &str, map: &HashMap<char, u8>) -> String {
    let bits = get_bits(&word, &map);
    decode(bits)
}

fn decode(bits: Vec<u8>) -> String {
    let mut index: usize = 0;
    let mut bit_8: [u8; 8] = [0; 8];
    let mut decoded = String::new();

    for bit in bits {
        bit_8[index] = bit;

        if index == 7 {
            let decimal = basic::to_decimal_8bit_binary(&bit_8);
            decoded.push(decimal as char);

            // reset for next 8 bits
            for x in 0..8 {
                bit_8[x] = 0;
            }
            index = 0;
        }
        else {
            index += 1;
        }
    }

    decoded
}

fn get_bits(word: &str, map: &HashMap<char, u8>) -> Vec<u8> {
    let characters = word.as_bytes();
    let mut all_the_bits: Vec<u8> = Vec::new();

    for x in characters {
        // println!("{} -> {}", *x, &(*x as char));
        if *x == 61 { continue }
        let six_bit = map.get(&(*x as char)).unwrap();
        let six_bit_binary = basic::decimal_to_6bit_binary(&six_bit);

        for b in &six_bit_binary {
            all_the_bits.push(*b);
        }
    }

    all_the_bits
}
