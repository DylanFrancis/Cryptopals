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
    let map = indices();

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    if args.len() > 3 {
        match args.get(1).unwrap().as_str() {
            "encode" => {
                encode(args.split_at(2).1, &map);
            },
            "decode" => {
                decode(args.split_at(2).1, &map);
            },
            &_ => {}
        }
    }
    else {
        println!("too few arguments");
    }
}

fn decode(to_decode: &[String], map: &HashMap<u8, char>) {
    for word in to_decode {
    }
}

fn encode(to_encode: &[String], map: &HashMap<u8, char>) {
    for word in to_encode {
        let encode = base64binary::run(word.as_str(), &map);
        println!("{} -> {}", word, encode);
    }
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
