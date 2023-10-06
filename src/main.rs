use crate::utils::read_data;
use std::env;

mod astar;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut fixed_path = String::new();

    if args.len() >= 2 {
        fixed_path = args[1].clone();
    }
    let (size_x, size_y, floors, start, target) = read_data(fixed_path);
    //println!("start: {:?}; target: {:?}", start, target);

    let (cost, path) = astar::astar(floors, start, target, size_x, size_y);
    println!("cost: {:?}; path: {:?}", cost, path)
}
