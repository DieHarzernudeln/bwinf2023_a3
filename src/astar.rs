use std::collections::HashMap;
use crate::utils::{Node, PVector};

pub fn astar(floors: [Vec<Vec<u8>>; 2], start: PVector, target: PVector, size_x: u32, size_y: u32) -> (u32, Vec<PVector>) {
    let mut nodes: HashMap<PVector, Node> = HashMap::new();

    let mut known_nodes: Vec<PVector> = vec![];
    let mut finished_nodes: Vec<PVector> = vec![];

    known_nodes.push(start);
    nodes.insert(start, Node::new(0, 0, PVector::zero()));

    while known_nodes.len() > 0 {
        let current = *known_nodes.iter().min_by_key(|p| nodes[p].g + nodes[p].h).unwrap();

        if current == target {
            let mut path: Vec<PVector> = vec![];
            let mut current = target;
            while current != start {
                path.push(current);
                current = nodes[&current].parent;
            }
            path.push(start);
            path.reverse();
            return (nodes[&target].g, path);
        }

        known_nodes.retain(|&p| p != current);
        finished_nodes.push(current);

        let neighbors = get_neighbors(current, &floors, size_x, size_y);

        for neighbor in neighbors.keys() {
            let dist = *neighbors.get(neighbor).unwrap();

            if finished_nodes.contains(neighbor) {
                if nodes[neighbor].g <= nodes[&current].g + dist {
                    continue;
                }
            }

            let g = nodes[&current].g + dist;
            let h = neighbor.dist(target);

            if !known_nodes.contains(&neighbor) {
                known_nodes.push(*neighbor);
            } else if g >= nodes[&neighbor].g {
                continue;
            }

            nodes.insert(*neighbor, Node::new(g, h, current));
        }
    }

    return (0, vec![]);
}


fn get_neighbors(current: PVector, floors: &[Vec<Vec<u8>>; 2], size_x: u32, size_y: u32) -> HashMap<PVector, u32> {
    let mut neighbors: HashMap<PVector, u32> = HashMap::new();


    if current.x > 0 {
        neighbors.insert(PVector::new(current.x - 1, current.y, current.z), 1);
    }
    if current.x < size_x - 1 {
        neighbors.insert(PVector::new(current.x + 1, current.y, current.z), 1);
    }
    if current.y > 0 {
        neighbors.insert(PVector::new(current.x, current.y - 1, current.z), 1);
    }
    if current.y < size_y - 1 {
        neighbors.insert(PVector::new(current.x, current.y + 1, current.z), 1);
    }
    if current.z == 1 {
        neighbors.insert(PVector::new(current.x, current.y, current.z - 1), 3);
    }
    if current.z == 0 {
        neighbors.insert(PVector::new(current.x, current.y, current.z + 1), 3);
    }

    return neighbors.iter().filter(|n| is_walkable((*n).0, floors)).map(|(k, v)| (*k, *v)).collect();
}

fn is_walkable(current: &PVector, floors: &[Vec<Vec<u8>>; 2]) -> bool {
    return floors[current.z as usize][current.y as usize][current.x as usize] != 0;
}
