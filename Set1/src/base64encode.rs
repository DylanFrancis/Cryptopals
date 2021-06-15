use std::collections::HashMap;

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
            let i = to_decimal(index);
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
        let i = to_decimal(index);
        encode.push(*map.get(&i).unwrap());
    }

    encode
}

fn to_decimal(arr: [u8; 6]) -> u8 {
    let mut num: u8 = 0;
    let mut count: u32 = 6;

    for bit in &arr {
        count -= 1;
        if *bit == 1 {
            num += 2_u8.pow(count)
        }
    }

    num
}

fn concat_binary(to_binary: &str) -> Vec<u8> {
    let mut main: Vec<u8> = Vec::new();

    for character_byte in to_binary.as_bytes() {
        let character_binary = binary(&character_byte);
        for num in &character_binary {
            main.push(*num);
        }
    }

    main
}

fn binary(num: &u8) -> [u8; 8] {
    let mut x = *num;
    let mut arr: [u8; 8] = [0; 8];
    let mut count = 7;

    while x > 0 {
        arr[count] = x % 2;
        x = &x / 2;
        count -= 1;
    }

    arr
}
