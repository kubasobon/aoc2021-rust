use std::fs;

fn main() {
    let input_file = fs::read_to_string("input.txt")
        .expect("Could not read 'input.txt'");

    let mut num : i32 = -1;
    let mut prev : i32 = 0;
    let mut bigger : i32 = 0;
    for line in input_file.lines() {
        if num == -1 {
            prev = line.parse::<i32>().unwrap();
            num = 0;
            continue;
        }

        num = line.parse::<i32>().unwrap();
        if num > prev {
            bigger += 1;
        }
        prev = num;
    }

    println!("Found {} increases!", bigger);
}
