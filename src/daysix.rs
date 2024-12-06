use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    usize,
};

pub fn daysix(s: String) {
    let (map, mut obs, curr, x, y) = partone(s.clone());
    parttwo(map, &mut obs, curr, x, y);
    #[derive(Eq, Hash, PartialEq, Copy, Clone)]
    enum Facing {
        Up,
        Down,
        Left,
        Right,
    }
    fn partone(
        s: String,
    ) -> (
        HashMap<(usize, usize), Facing>,
        HashSet<(usize, usize)>,
        (usize, usize),
        usize,
        usize,
    ) {
        let mut charvec: Vec<Vec<char>> = Vec::new();
        for line in s.lines() {
            charvec.push(line.chars().collect());
        }
        let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
        let mut visited: HashMap<(usize, usize), Facing> = HashMap::new();
        let mut currentpos: (usize, usize) = (0, 0);
        let mut starting: (usize, usize) = (0, 0);
        let mut direction: Facing = Facing::Up;
        for i in 0..charvec.len() {
            for j in 0..charvec[0].len() {
                match charvec[i][j] {
                    '#' => {
                        obstacles.insert((i, j));
                    }
                    '^' => {
                        visited.insert((i, j), Facing::Up);
                        currentpos = (i, j);
                        starting = (i, j);
                    }
                    _ => (),
                }
            }
        }
        while currentpos.0 > 0
            && currentpos.0 < charvec.len()
            && currentpos.1 > 0
            && currentpos.1 < charvec[0].len()
        {
            let nextpos: (usize, usize);
            match direction {
                Facing::Up => {
                    nextpos = (currentpos.0 - 1, currentpos.1);
                }
                Facing::Down => {
                    nextpos = (currentpos.0 + 1, currentpos.1);
                }
                Facing::Left => {
                    nextpos = (currentpos.0, currentpos.1 - 1);
                }
                Facing::Right => {
                    nextpos = (currentpos.0, currentpos.1 + 1);
                }
            }
            if obstacles.contains(&nextpos) {
                match direction {
                    Facing::Up => {
                        direction = Facing::Right;
                    }
                    Facing::Down => {
                        direction = Facing::Left;
                    }
                    Facing::Left => {
                        direction = Facing::Up;
                    }
                    Facing::Right => {
                        direction = Facing::Down;
                    }
                }
            } else {
                if !visited.contains_key(&nextpos) {
                    visited.insert(nextpos, direction);
                } else {
                    if visited[&nextpos] == direction {
                        break;
                    }
                }
                currentpos = nextpos;
            }
        }
        println!("{}", visited.len());
        return (
            visited,
            obstacles,
            starting,
            charvec.len(),
            charvec[0].len(),
        );
    }
    fn parttwo(
        map: HashMap<(usize, usize), Facing>,
        obstacles: &mut HashSet<(usize, usize)>,
        starting: (usize, usize),
        x: usize,
        y: usize,
    ) {
        let mut result = 0;
        for item in map {
            if item.0 != starting {
                obstacles.insert(item.0);
                let mut currentpos = starting;
                let mut visited: HashMap<(usize, usize), Facing> = HashMap::new();
                let mut direction: Facing = Facing::Up;
                while currentpos.0 > 0 && currentpos.0 < x && currentpos.1 > 0 && currentpos.1 < y {
                    let nextpos: (usize, usize);
                    match direction {
                        Facing::Up => {
                            nextpos = (currentpos.0 - 1, currentpos.1);
                        }
                        Facing::Down => {
                            nextpos = (currentpos.0 + 1, currentpos.1);
                        }
                        Facing::Left => {
                            nextpos = (currentpos.0, currentpos.1 - 1);
                        }
                        Facing::Right => {
                            nextpos = (currentpos.0, currentpos.1 + 1);
                        }
                    }
                    if obstacles.contains(&nextpos) {
                        match direction {
                            Facing::Up => {
                                direction = Facing::Right;
                            }
                            Facing::Down => {
                                direction = Facing::Left;
                            }
                            Facing::Left => {
                                direction = Facing::Up;
                            }
                            Facing::Right => {
                                direction = Facing::Down;
                            }
                        }
                    } else {
                        if !visited.contains_key(&nextpos) {
                            visited.insert(nextpos, direction);
                        } else {
                            if visited[&nextpos] == direction {
                                result += 1;
                                break;
                            }
                        }
                        currentpos = nextpos;
                    }
                }
                obstacles.remove(&item.0);
            }
        }
        println!("{}", result);
    }
}
