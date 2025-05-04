use std::fs::read_to_string;

fn get_raw_board(filename: &str) -> Vec<Vec<bool>> {
    let mut result: Vec<Vec<bool>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let mut chars = line.chars();
        let mut char_arr: Vec<bool> = Vec::new();

        for n in 0..(line.chars().count()/2) {
            char_arr.push(
                match chars.next().unwrap_or(' ') {
                    ' ' => true,
                    _ => false
                }
            );
            let _ = chars.next();
        }

        result.push(char_arr);
    }
    result
}

fn main() {
    let lines = get_raw_board("input/maze.txt");
    println!("{}", lines[1][2]);
}
