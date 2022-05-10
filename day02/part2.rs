use std::fs;

fn main() {
    let input_file = fs::read_to_string("input.txt")
        .expect("Could not read 'input.txt'");
    let mut aim = 0;
    let mut horiz = 0;
    let mut depth = 0;

    for line in input_file.lines() {
        let mut line_split = line.trim().split_whitespace();
        let command = line_split.next().unwrap();
        let amount_str = line_split.next().unwrap();

        let amount = amount_str.parse::<i32>().expect("Could not parse amount");
        match command {
            "forward" => {
                horiz += amount;
                depth += amount * aim;
            },
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => panic!("Command {} not supported", command),
        }
    }

    println!("Horizontal Positon: {}", horiz);
    println!("Depth: {}", depth);
    println!("Horizontal Position x Depth: {}", horiz * depth);
}
