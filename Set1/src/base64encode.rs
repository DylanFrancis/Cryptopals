use std::collections::HashMap;
use crate::util::basic;

pub fn encode_string(to_base64: &str, map: &HashMap<u8, char>) -> String {
    let main = concat_binary(to_base64);
    encode_bits(&main, map)
}

pub fn encode_bits(to_base64: &Vec<u8>, map: &HashMap<u8, char>) -> String {
    let mut encoded = to_string(&to_base64, map);
    encoded = padding(encoded, to_base64.len());

    encoded
}

fn padding(mut encoded: String, len: usize) -> String {
    match (len * 8) % 24 {
        8 => {
            encoded.push('=');
            encoded.push('=');
        },
        16 => encoded.push('='),
        _ => {}
    }

    encoded
}

fn to_string(main: &Vec<u8>, map: &HashMap<u8, char>) -> String {
    let mut count: usize = 0;
    let mut index: [u8; 6] = [0; 6];
    let mut encode: String = String::new();

    for bit in main {
        // println!("{}", bit);
        index[count] = *bit;

        if count == 5 {
            let i = basic::to_decimal_6bit_binary(index);
            encode.push(*map.get(&i).unwrap());

            // reset for next 8 bits
            for x in 0..6 {
                index[x] = 0;
            }
            count = 0;
        }
        else {
            count += 1;
        }
    }

    if count != 0 {
        let i = basic::to_decimal_6bit_binary(index);
        encode.push(*map.get(&i).unwrap());
    }

    encode
}


fn concat_binary(to_binary: &str) -> Vec<u8> {
    let mut main: Vec<u8> = Vec::new();

    for character_byte in to_binary.as_bytes() {
        let character_binary = basic::decimal_to_8bit_binary(&character_byte);
        for num in &character_binary {
            main.push(*num);
        }
    }

    main
}
