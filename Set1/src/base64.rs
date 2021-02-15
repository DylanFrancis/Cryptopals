use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn run() {
    let s = "my first base64 encoder";
    let res = encode_to_base64(s);
    println!("{}", res);
}

pub fn encode_to_base64(to_base64: &str) -> String {
    let concat = concat_binary(to_base64);

    let mut groups = to_group(concat, 6);

    let eight_bit_groups = group_to_eight_bit(groups);

    let indices = indices();
    let mut encoded = String::new();

    for x in eight_bit_groups {
        let d = binary_to_decimal(&x);
        let b64 = indices.get(&d).unwrap();
        encoded.push(*b64);
    }

    encoded
}

fn to_group(to_group: String, size: u32) -> Vec<String> {
    let mut groups = Vec::new();
    let mut count = 0;
    let mut cur_string = String::new();

    for character in to_group.chars() {
        count += 1;
        cur_string.push(character);

        if count == size {
            groups.push(cur_string);

            cur_string = String::new();
            count = 0
        }
    }

    if cur_string.len() > 0 {
        while cur_string.len() < 6 {
            cur_string.push('0');
        }
        groups.push(cur_string);
    }


    groups
}

fn concat_binary(to_binary: &str) -> String {
    let mut concat = String::new();
    for character_byte in to_binary.as_bytes() {
        concat.push_str(to_eight_bit(decimal_to_binary(&character_byte)).as_str());
    }
    concat
}

fn group_to_eight_bit(mut groups: Vec<String>) -> Vec<String> {
    let mut eight_bit_groups = Vec::new();

    for mut s in groups {
        s.insert(0, '0');
        s.insert(0, '0');

        eight_bit_groups.push(s);
    }

    eight_bit_groups
}

fn to_eight_bit(mut x: String) -> String {
    while x.len() < 8 {
        x.insert_str(0, "0");
    }
    x
}

fn binary_to_decimal(binary: &String) -> u8 {
    let mut count = 0;
    let mut decimal: u8 = 0;

    for c in binary.chars().rev() {
        if c == '1' {
            decimal += (2:u8).pow(count);
        }

        count += 1;
    }

    decimal
}

fn decimal_to_binary(x: &u8) -> String {

    if *x == 0 {
        return String::new();
    }

    let quotient = x / 2;

    let mut digit = (x % 2).to_string();
    let next = decimal_to_binary(&quotient);

    digit.insert_str(0, next.as_str());

    digit
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
