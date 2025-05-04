use crate::node::{Node, Pos};
use std::collections::{VecDeque, HashSet, HashMap};

pub struct Bfs {
    pub nodes: Vec<Node>,
    pub queue: VecDeque<Pos>,
    pub visited: HashSet<Pos>,
    pub came_from: HashMap<Pos, Pos>,
    pub goal: Pos,
}

impl Bfs {
    pub fn new(nodes_arr: Vec<Node>, start: Pos, goal: Pos) -> Bfs {
        Bfs {
            nodes: nodes_arr,
            queue: VecDeque::from([start.clone()]),
            visited: HashSet::from([start]),
            came_from: HashMap::new(),
            goal,
        }
    }

    pub fn run(&mut self, graph_hash: &HashMap<Pos, usize>) ->  Option<Vec<Pos>> {
        while let Some(current_pos) = self.queue.pop_front() {
            if current_pos == self.goal {
                return Some(self.reconstruct_path());
            }

            if let Some(&idx) = graph_hash.get(&current_pos) {
                let node = &self.nodes[idx - 1];

                let neighbors = [
                    node.left.clone(),
                    node.right.clone(),
                    node.up.clone(),
                    node.down.clone(),
                ];

                for neighbor_opt in neighbors.iter() {
                    if let Some(neighbor) = neighbor_opt {
                        if !self.visited.contains(neighbor) {
                            self.visited.insert(neighbor.clone());
                            self.queue.push_back(neighbor.clone());
                            self.came_from.insert(neighbor.clone(), current_pos.clone());
                        }
                    }
                }
            }
        }

        None
    }

    pub fn reconstruct_path(&self) -> Vec<Pos> {
        let mut path = Vec::new();
        let mut current = self.goal;

        while let Some(prev) = self.came_from.get(&current) {
            path.push(current);
            current = prev.clone();
        }
        path.push(current);
        path.reverse();
        path
    }
}