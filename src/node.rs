use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Pos {
    x: isize,
    y: isize
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Pos {
        Pos {
            x: x,
            y: y
        }
    }

    pub fn mask_copy(&self, x: isize, y: isize) -> Pos {
        Pos {
            x: self.x + x,
            y: self.y + y
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pos: Pos,
    left: Option<Pos>,
    right: Option<Pos>,
    up: Option<Pos>,
    down: Option<Pos>,
}

impl Node {
    pub fn new(pos: Pos) -> Node {
        Node {
            pos: pos,
            left: None,
            right: None,
            up: None,
            down: None,
        }
    }

    fn get_adj_pos(&self) -> [Pos; 4] {
        [
            self.pos.mask_copy(-1, 0), 
            self.pos.mask_copy(1, 0),
            self.pos.mask_copy(0, -1), // inverted because down is up
            self.pos.mask_copy(0, 1)
        ]
    }

    pub fn get_adj_nodes(&mut self, board_hashmap: &HashMap<Pos, bool>) {
        let adj_pos = self.get_adj_pos();
        let mut count = 0;

        for pos in adj_pos {
            println!("{:#?}, {}", pos, count);
            match board_hashmap.get(&pos) {
                Some(v) => {
                    if v == &true {
                        match count {
                            0 => self.left = Some(pos),
                            1 => self.right = Some(pos),
                            2 => self.up = Some(pos),
                            _ => self.down = Some(pos)
                        }
                    }
                },
                None => {}
            }
            count += 1;
        }
    }
}