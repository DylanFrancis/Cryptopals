#![feature(type_ascription)]
#![feature(exclusive_range_pattern)]
#![feature(assoc_char_funcs)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Read, Error};
use std::{io, env};
use std::process::exit;

mod base64;
mod base64encode;
mod base64decode;
mod hex;
mod util;

fn main() {
    let indices = indices();
    let bit_to_character_map = indices.0;
    let character_to_bit_map = indices.1;

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    if args.len() > 2 {
        let input = args.split_at(2).1;

        match args.get(1).unwrap().as_str() {
            "b64-encode" => {
                encode(input, &bit_to_character_map)
            },
            "b64-decode" => {
                decode(input, &character_to_bit_map)
            },
            "hex-b64" => {
                hex_to_b64(input, &bit_to_character_map)
            },
            "xor-hex" => {
                xor_hex(input)
            },
            "hex-decode" => {
                hex_decode(input)
            },
            "hex-decode-file" => {
                hex_decode_file(input)
            },
            &_ => {}
        }
    }
    else {
        println!("too few arguments");
    }
}

fn hex_decode(to_decode: &[String]) {
    for h in to_decode {
        let decoded = hex::hex_to_ascii(&h);
        println!("{} -> {}", h, decoded);
    }
}

fn hex_decode_file(file_names: &[String]) {
    for file in file_names {
        let file = File::open(file).unwrap();
        let file_reader = BufReader::new(file);

        for line in file_reader.lines() {
            match line {
                Ok(x) => {
                    let ascii = hex::hex_to_ascii(&x);
                    println!("{}", ascii);
                }
                Err(x) => { panic!("{}", x) }
            }
        }
    }
}

fn xor_hex(to_xor: &[String]) {
    if to_xor.len() < 2 {
        println!("need 2 strings to xor");
        return;
    }

    let hex1 = &to_xor[0];
    let hex2 = &to_xor[1];

    let result = hex::xor(hex1, hex2);

    println!("{}", result);
}

fn hex_to_b64(to_decode: &[String], map: &HashMap<u8, char>) {
    for word in to_decode {
        let bits = hex::hex_to_bits(word);
        let b64 = base64encode::encode_bits(&bits, map);
        println!("{}", b64);
    }
}

fn decode(to_decode: &[String], map: &HashMap<char, u8>) {
    for word in to_decode {
        let decoded = base64decode::run(&word, &map);
        println!("{} -> {}", word, decoded);
    }
}

fn encode(to_encode: &[String], map: &HashMap<u8, char>) {
    for word in to_encode {
        let encode = base64encode::encode_string(word.as_str(), &map);
        println!("{} -> {}", word, encode);
    }
}

fn indices() -> (HashMap<u8, char>, HashMap<char, u8>) {
    let file = File::open("indices.txt").unwrap();
    let file_reader = BufReader::new(file);

    let mut bit_to_character = HashMap::new();
    let mut character_to_bit = HashMap::new();

    for line_result in file_reader.lines() {
        let line = line_result.unwrap();
        let mut split = line.split(" ");

        let key = split.next().unwrap().parse::<u8>().unwrap();
        let value = split.next().unwrap().as_bytes()[0] as char;

        bit_to_character.insert(key, value);
        character_to_bit.insert(value, key);
    }

    (bit_to_character, character_to_bit)
}
