use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

pub fn daythree() {
    let file = File::open("input3.txt");
    let mut contents = String::new();
    let _ = file
        .expect("Error while reading")
        .read_to_string(&mut contents);

    let re = Regex::new(r"(mul\(\d+\,\d+\))|(do\(\))|(don\'t\(\))").unwrap();
    let vec1: Vec<&str> = re
        .find_iter(&contents.as_str())
        .filter_map(|x| Some(x.as_str()))
        .collect();
    let mut result = 0;
    let mut active = true;
    for item in vec1 {
        match item {
            "do()" => {
                active = true;
            }
            "don't()" => {
                active = false;
            }
            _ => {
                if active {
                    let item = &item[4..item.len() - 1];
                    let vec2: Vec<&str> = item.split(',').collect();
                    result += vec2[0].parse::<i32>().unwrap() * vec2[1].parse::<i32>().unwrap();
                }
            }
        }
    }
    println!("{result}");
}
