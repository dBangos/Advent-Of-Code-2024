use std::fs::File;
use std::io::prelude::*;
pub fn daytwo() {
    let file = File::open("input2.txt");
    let mut contents = String::new();
    let _ = file
        .expect("Error while reading")
        .read_to_string(&mut contents);
    let mut result = 0;
    for line in contents.lines().map(|line| {
        line.split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect()
    }) {
        if vec_check(line, true) {
            result += 1;
        }
    }
    println!("{result}");
    fn vec_check(input: Vec<i32>, dampener: bool) -> bool {
        let increasing = input[0] < input[1];
        for index in 0..input.len() - 1 {
            let diff = input[index] - input[index + 1];
            if increasing {
                if diff < -3 || diff > -1 {
                    if dampener {
                        let mut input1 = input.clone();
                        let mut input2 = input.clone();
                        input1.remove(index);
                        input2.remove(index + 1);
                        return vec_check(input1, false) || vec_check(input2, false);
                    } else {
                        return false;
                    }
                }
            } else {
                if diff > 3 || diff < 1 {
                    if dampener {
                        let mut input1 = input.clone();
                        let mut input2 = input.clone();
                        input1.remove(index);
                        input2.remove(index + 1);
                        return vec_check(input1, false) || vec_check(input2, false);
                    } else {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}
