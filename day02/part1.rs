use std::fs;

fn main() {
    let input_file = fs::read_to_string("input.txt")
        .expect("Could not read 'input.txt'");
    let mut horiz = 0;
    let mut depth = 0;

    for line in input_file.lines() {
        let mut line_split = line.trim().split_whitespace();
        let command = line_split.next().unwrap();
        let amount_str = line_split.next().unwrap();

        let amount = amount_str.parse::<i32>().expect("Could not parse amount");
        match command {
            "forward" => horiz += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => panic!("Command {} not supported", command),
        }
    }

    println!("Horizontal Positon: {}", horiz);
    println!("Depth: {}", depth);
    println!("Horizontal Position x Depth: {}", horiz * depth);
}
