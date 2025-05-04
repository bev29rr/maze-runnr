mod node;
mod search;

use node::{Pos, Node};
use search::{Bfs};
use std::{collections::HashMap, fs::read_to_string, os::raw};

fn get_raw_board(filename: &str) -> (Vec<Vec<bool>>, HashMap<Pos, bool>) {
    let mut result: Vec<Vec<bool>> = Vec::new();
    let mut board_hash: HashMap<Pos, bool> = HashMap::new();

    let mut y_count = 0;

    for line in read_to_string(filename).unwrap().lines() {
        let mut chars = line.chars();
        let mut char_arr: Vec<bool> = Vec::new();

        for n in 0..(line.chars().count()/2) {
            char_arr.push(
                match chars.next().unwrap_or('_') {
                    ' ' => {
                        board_hash.insert(Pos::new(n as isize, y_count), true);
                        true
                    },
                    _ => {
                        board_hash.insert(Pos::new(n as isize, y_count), false);
                        false
                    }
                }
            );
            let _ = chars.next();
        }

        result.push(char_arr);
        y_count += 1;
    }

    (result, board_hash)
}

fn to_graph(raw_board: Vec<Vec<bool>>, raw_board_hash: HashMap<Pos, bool>) -> (Vec<Node>, HashMap<Pos, usize>) {
    let mut pos_hash = HashMap::new();
    let mut node_arr: Vec<Node> = Vec::new();

    let mut y_count = 0;

    for row in raw_board {
        let mut x_count = 0;
        for tile in row {
            match tile {
                true => {
                    let node_pos = Pos::new(x_count, y_count);
                    let mut node = Node::new(node_pos.clone());
                    node.get_adj_nodes(&raw_board_hash);
                    node_arr.push(node);
                    pos_hash.insert(node_pos, node_arr.len());
                },
                false => {}
            }
            x_count += 1;
        }
        y_count += 1;
    }

    (node_arr, pos_hash)
}

fn main() {
    println!("Generating bool board...");
    let (raw_board, raw_board_hash) = get_raw_board("input/maze.txt");
    println!("Done!");
    
    println!("Generating graph...");
    let (graph, graph_hash) = to_graph(raw_board, raw_board_hash);
    println!("Done!");
    
    let mut bfs_search = Bfs::new(
        graph, 
        Pos::new(1,1),
        Pos::new(255,255)
    );
    println!("Performing Bfs Search...");
    let run_status = bfs_search.run(&graph_hash);
    println!("Success!");
    if let Some(pos_vec) = run_status {
        println!("{:?}", pos_vec);
    }
}
