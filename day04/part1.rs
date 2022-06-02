use std::fmt;
use std::fs;

struct Board {
    board: Vec::<i32>,
    marked: Vec::<bool>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: Vec::new(),
            marked: vec![false; 25],
        }
    }

    fn mark(&mut self, given_num : &i32) {
        for (i, num) in self.board.iter().enumerate() {
            if num == given_num {
                self.marked[i] = true;
                break;
            }
        }
    }
}

fn check_row(b : &Board) -> bool {
    for row in 0..5 {
        let mut win = true;
        for cell in (row*5)..(row*5)+5 {
            if !b.marked[cell] {
                win = false;
                break;
            }
        }
        if win {
            return true;
        }
    }
    return false;
}

fn check_col(b : &Board) -> bool {
    for col in 0..5 {
        let mut win = true;
        for row in 0..5 {
            let cell = col + row*5;
            if !b.marked[cell] {
                win = false;
                break;
            }
        }
        if win {
            return true;
        }
    }
    return false;
}

fn sum_unmarked(b : &Board) -> i32 {
    let mut sum : i32 = 0;
    for (i, num) in b.board.iter().enumerate() {
        if !b.marked[i] {
            sum += num;
        }
    }
    return sum;
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut disp = String::new();
        for (i, num) in self.board.iter().enumerate() {
            if i > 0 && i % 5 == 0 {
                disp.push_str("\n");
            }
            let is_marked = if self.marked[i] { "*" } else { " " };
            disp.push_str(format!("{:>3}{}", num, is_marked).as_str());
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
    for b in boards.iter() {
        println!("{}\n", b);
    }

    'number_loop: for given_num in numbers.iter() {
        for b in boards.iter_mut() {
            b.mark(given_num);
            if check_row(&b) || check_col(&b) {
                println!("BINGO!\n{}\n", b);
                let sum = sum_unmarked(&b);
                println!("Sum of unmarked numbers: {}", sum);
                println!("{} x {}: {}", sum, given_num, sum * given_num);
                break 'number_loop;
            }
        }
    }
}
