use std::fmt;
use std::fs;

struct Board {
    board: Vec::<i32>,
    marked: Vec::<bool>,
}

impl Board {
    pub fn new() -> Self {
        let m: Vec::<bool> = vec![false; 25];
        Self {
            board: Vec::new(),
            marked: m,
        }
    }

    fn mark(&mut self, num: i32) {
        if !self.board.contains(&num) {
            return;
        }
        for (i, n) in self.board.iter().enumerate() {
            if n == &num {
                self.marked[i] = true;
            }
        }
    }

    fn has_bingo(&self) -> bool {

    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = String::new();
        for (i, num) in self.board.iter().enumerate() {
            if i > 0 && i % 5 == 0 {
                disp.push_str("\n");
            }
            disp.push_str(format!("{:>3}", num).as_str());
            disp.push_str(if self.marked[i] { "*" } else { " " });
        }
        write!(f, "{}", disp)
    }
}


fn main() {
    let input_file = fs::read_to_string("input.txt")
        .expect("Could not load 'input.txt'");
    let lines: Vec<&str> = input_file.lines().collect();
    let numbers: Vec<i32> = lines[0].split(",").map(|si| si.parse::<i32>().unwrap()).collect();
    let mut boards : Vec::<Board> = Vec::new();
    println!("Loaded numbers: {:?}\n", numbers);

    let mut new_board = Board::new();
    for l in lines[2..].iter() {
        if l.trim() == "" {
            boards.push(new_board);
            new_board = Board::new();
        }
        for num_str in l.trim().split_whitespace() {
            let num = num_str.parse::<i32>().unwrap();
            new_board.board.push(num);
        }
    }
    boards.push(new_board);

    println!("Collected {} boards", boards.len());

    while {
        println!("---------------------------------------");
        for b in boards.iter_mut() {
            println!("{}\n", b);
        }
    }
}
