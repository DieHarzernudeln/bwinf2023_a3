use crate::utils::read_data;

mod astar;
mod utils;

fn main() {
    let (size_x, size_y, floors, start, target) = read_data();
    println!("start: {:?}; target: {:?}", start, target);

    let (cost, path) = astar::astar(floors, start, target, size_x, size_y);
    println!("cost: {:?}; path: {:?}", cost, path)
}
