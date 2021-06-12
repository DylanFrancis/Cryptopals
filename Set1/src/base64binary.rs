use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
// use byte::*;

pub fn run() {
    let s = "abcasasqweqae";
    let encoded = encode_to_base64(s);
    println!("{} -> {}", s, encoded);
}

pub fn encode_to_base64(to_base64: &str) -> String {
    let main = concat_binary(to_base64);
    to_string(main)
}

fn to_string(main: Vec<u8>) -> String {
    let mut count: usize = 0;
    let mut index: [u8; 6] = [0; 6];
    let mut encode: String = String::new();
    let map= indices();

    for bit in main {
        // println!("{}", bit);
        index[count] = bit;

        if count == 5 {
            let i = to_decimal(index);
            println!("{:?} -> {}", index, i);
            encode.push(*map.get(&i).unwrap());

            count = 0;
            for x in 0..6 {
                index[x] = 0;
            }
        }
        else {
            count += 1;
        }
    }

    if count != 0 {
        println!("{}", count);
        let i = to_decimal(index);
        println!("{:?} -> {}", index, i);
        encode.push(*map.get(&i).unwrap());
    }

    encode
}

fn to_decimal(arr: [u8; 6]) -> u8{
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
            print!("{}", num);
            main.push(*num);
        }
    }

    println!("\n");
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


fn indices() -> HashMap<u8, char> {
    let file = File::open("indices.txt").unwrap();
    let file_reader = BufReader::new(file);

    let mut indices = HashMap::new();

    for line_result in file_reader.lines() {
        let line = line_result.unwrap();
        let mut split = line.split(" ");

        let key = split.next().unwrap().parse::<u8>().unwrap();
        let value = split.next().unwrap().as_bytes()[0] as char;

        indices.insert(key, value);
    }

    indices
}
