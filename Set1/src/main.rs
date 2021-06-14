#![feature(type_ascription)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Read, Error};
use std::{io, env};
use std::process::exit;

mod base64;
mod base64binary;
mod base64decode;

fn main() {
    let indices = indices();
    let bit_to_character_map = indices.0;
    let character_to_bit_map = indices.1;

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    if args.len() > 2 {
        match args.get(1).unwrap().as_str() {
            "encode" => {
                encode(args.split_at(2).1, &bit_to_character_map);
            },
            "decode" => {
                decode(args.split_at(2).1, &character_to_bit_map);
            },
            &_ => {}
        }
    }
    else {
        println!("too few arguments");
    }
}

fn decode(to_decode: &[String], map: &HashMap<char, u8>) {
    for word in to_decode {

    }
}

fn encode(to_encode: &[String], map: &HashMap<u8, char>) {
    for word in to_encode {
        let encode = base64binary::run(word.as_str(), &map);
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
