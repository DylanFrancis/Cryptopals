use std::collections::HashMap;

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
            let decimal = to_decimal(&bit_8);
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

fn to_decimal(arr: &[u8; 8]) -> u8 {
    let mut num: u8 = 0;
    let mut count: u32 = 8;

    for bit in arr {
        count -= 1;
        if *bit == 1 {
            num += 2_u8.pow(count)
        }
    }

    num
}

fn get_bits(word: &str, map: &HashMap<char, u8>) -> Vec<u8> {
    let characters = word.as_bytes();
    let mut all_the_bits: Vec<u8> = Vec::new();

    for x in characters {
        // println!("{} -> {}", *x, &(*x as char));
        if *x == 61 { continue }
        let six_bit = map.get(&(*x as char)).unwrap();
        let six_bit_binary = binary(&six_bit);

        for b in &six_bit_binary {
            all_the_bits.push(*b);
        }
    }

    all_the_bits
}

fn binary(num: &u8) -> [u8; 6] {
    let mut x = *num;
    let mut arr: [u8; 6] = [0; 6];
    let mut count = 6;

    while x > 0 {
        count -= 1;
        arr[count] = x % 2;
        x = &x / 2;
    }

    arr
}
