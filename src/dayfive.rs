use std::collections::HashSet;

pub fn dayfive(s: String) {
    partone(s.clone());
    //parttwo(s);
    pub fn partone(s: String) {
        let splitstr: Vec<&str> = s.split("\n\n").collect();
        let mut set: HashSet<(i32, i32)> = HashSet::new();
        for item in splitstr[0].lines() {
            let vec: Vec<i32> = item
                .split('|')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|x| x.parse().unwrap())
                .collect();
            set.insert((vec[0], vec[1]));
        }
        let mut result: i32 = 0;
        for item in splitstr[1].lines() {
            //println!("{}", item);
            let vec: Vec<i32> = item
                .split(',')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|x| x.parse().unwrap())
                .collect();
            let mut correct: bool = true;
            for i in 0..vec.len() - 1 {
                for j in (i + 1)..vec.len() {
                    if set.contains(&(vec[j], vec[i])) {
                        correct = false;
                        break;
                    }
                }
            }
            if correct {
                result += vec[vec.len() / 2];
            }
        }
        println!("{}", result);
    }
    pub fn parttwo(s: String) {
        let splitstr: Vec<&str> = s.split("\n\n").collect();
        let mut set: HashSet<(i32, i32)> = HashSet::new();
        for item in splitstr[0].lines() {
            let vec: Vec<i32> = item
                .split('|')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|x| x.parse().unwrap())
                .collect();
            set.insert((vec[0], vec[1]));
        }
        let mut result: i32 = 0;
        for item in splitstr[1].lines() {
            //println!("{}", item);
            let mut vec: Vec<i32> = item
                .split(',')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|x| x.parse().unwrap())
                .collect();
            let mut correct: bool = true;
            for i in 0..vec.len() - 1 {
                for j in (i + 1)..vec.len() {
                    if set.contains(&(vec[j], vec[i])) {
                        correct = false;
                        break;
                    }
                }
            }
            if !correct {
                for i in (0..vec.len() - 1).rev() {
                    for j in (i + 1..vec.len()).rev() {
                        if set.contains(&(vec[j], vec[i])) {
                            let temp = vec[j];
                            vec[j] = vec[i];
                            vec[i] = temp;
                        }
                    }
                }
                result += vec[vec.len() / 2];
            }
        }
        println!("{}", result);
    }
}
