use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
pub fn dayone() {
    let file = File::open("input1.txt");
    let mut contents = String::new();
    let _ = file
        .expect("Error while reading")
        .read_to_string(&mut contents);
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();
    for line in contents.split("\n") {
        let splitline: Vec<&str> = line.split(" ").collect();
        if splitline[0] != "" {
            vec1.push(splitline[0].parse().unwrap());
            vec2.push(splitline[3].parse().unwrap());
        }
    }
    vec1.sort();
    vec2.sort();
    let mut distance: i32 = 0;
    for i in 0..vec1.len() {
        distance += (vec1[i] - vec2[i]).abs();
    }
    println!("{distance}");
    let mut similarity = 0;
    let mut frequency: HashMap<i32, i32> = HashMap::new();
    for item in vec2 {
        if let Some(existing_val) = frequency.get_mut(&item) {
            *existing_val += 1;
        } else {
            frequency.insert(item, 1);
        }
    }
    for item in vec1 {
        if let Some(freq) = frequency.get(&item) {
            similarity += item * freq;
        }
    }
    println!("{similarity}");
}
