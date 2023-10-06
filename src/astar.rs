use crate::utils::{Node, PVector};
use std::collections::HashMap;

pub fn astar(
    floors: [Vec<Vec<u8>>; 2],
    start: PVector,
    target: PVector,
    size_x: u32,
    size_y: u32,
) -> (u32, Vec<PVector>) {
    let mut nodes: HashMap<PVector, Node> = HashMap::new();

    let mut known_nodes: Vec<PVector> = vec![];
    let mut finished_nodes: Vec<PVector> = vec![];

    known_nodes.push(start);
    nodes.insert(start, Node::new(0, 0, PVector::zero()));

    while !known_nodes.is_empty() {
        let current = *known_nodes
            .iter()
            .min_by_key(|p| nodes[p].g + nodes[p].h)
            .unwrap();

        if current == target {
            return finalize(start, target, &nodes);
        }

        known_nodes.retain(|&p| p != current);
        finished_nodes.push(current);

        let neighbors = get_neighbors(current, &floors, size_x, size_y);
        check_neighbors(
            &neighbors,
            current,
            &mut nodes,
            &mut known_nodes,
            &finished_nodes,
            target,
        );
    }

    (0, vec![])
}

fn finalize(
    start: PVector,
    target: PVector,
    nodes: &HashMap<PVector, Node>,
) -> (u32, Vec<PVector>) {
    let mut path: Vec<PVector> = vec![];
    let mut current = target;
    while current != start {
        path.push(current);
        current = nodes[&current].parent;
    }
    path.push(start);
    path.reverse();
    (nodes[&target].g, path)
}

fn check_neighbors(
    neighbors: &HashMap<PVector, u32>,
    current: PVector,
    nodes: &mut HashMap<PVector, Node>,
    known_nodes: &mut Vec<PVector>,
    finished_nodes: &[PVector],
    target: PVector,
) {
    for neighbor in neighbors.keys() {
        let dist = *neighbors.get(neighbor).unwrap();

        if finished_nodes.contains(neighbor) && nodes[neighbor].g <= nodes[&current].g + dist {
            continue;
        }

        let g = nodes[&current].g + dist;
        let h = neighbor.dist(target);

        if !known_nodes.contains(neighbor) {
            known_nodes.push(*neighbor);
        } else if g >= nodes[&neighbor].g {
            continue;
        }

        nodes.insert(*neighbor, Node::new(g, h, current));
    }
}

fn get_neighbors(
    current: PVector,
    floors: &[Vec<Vec<u8>>; 2],
    size_x: u32,
    size_y: u32,
) -> HashMap<PVector, u32> {
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

    return neighbors
        .iter()
        .filter(|n| is_walkable(n.0, floors))
        .map(|(k, v)| (*k, *v))
        .collect();
}

fn is_walkable(current: &PVector, floors: &[Vec<Vec<u8>>; 2]) -> bool {
    floors[current.z as usize][current.y as usize][current.x as usize] != 0
}
