use std::collections::{HashMap};
use std::io::{BufRead, BufReader};

pub(crate) fn read_data() -> (u32, u32, [Vec<Vec<u8>>; 2], PVector, PVector) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let file_path = input.trim();
    let file = BufReader::new(std::fs::File::open(file_path).unwrap());

    let mut lines = file.lines();

    let first_line = lines.next().unwrap().unwrap();
    let split: Vec<&str> = first_line.split(" ").collect();
    let size_y = split[0].parse::<u32>().unwrap();
    let size_x = split[1].parse::<u32>().unwrap();

    let mut start: PVector = PVector::zero();
    let mut target: PVector = PVector::zero();

    let mut floors: [Vec<Vec<u8>>; 2] = [Vec::new(), Vec::new()];

    for f in 0..2 {
        for i in 0..size_y {
            let linestr = lines.next().unwrap().unwrap();
            let mut line: Vec<u8> = Vec::new();

            for (j, ch) in linestr.chars().enumerate() {
                let id = char_to_int(ch);

                line.push(id);
                if id == 2 {
                    start = PVector::new(j as u32, i, f);
                } else if id == 3 {
                    target = PVector::new(j as u32, i, f);
                }
            }
            floors[f as usize].push(line);
        }

        lines.next();
    }

    return (size_x, size_y, floors, start, target);
}

fn char_to_int(ch: char) -> u8 {
    let mut char_to_int: HashMap<char, u8> = HashMap::new();

    char_to_int.insert('#', 0);
    char_to_int.insert('.', 1);
    char_to_int.insert('A', 2);
    char_to_int.insert('B', 3);

    return *char_to_int.get(&ch).unwrap();
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct PVector {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}


impl PVector {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        PVector {
            x,
            y,
            z,
        }
    }

    pub fn zero() -> Self {
        PVector {
            x: 0,
            y: 0,
            z: 0,
        }
    }

    pub fn dist(&self, pos: PVector) -> u32 {
        return self.x.abs_diff(pos.x) + self.y.abs_diff(pos.y) + self.z.abs_diff(pos.z) * 3;
    }
}

#[derive(Debug)]
pub struct Node<> {
    pub g: u32,
    pub h: u32,
    pub parent: PVector,
}

impl Node {
    pub fn new(g: u32, h: u32, parent: PVector) -> Self {
        Node {
            g,
            h,
            parent,
        }
    }
}