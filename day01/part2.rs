use std::fs;

fn main() {
    let input_file = fs::read_to_string("input.txt")
        .expect("Could not read 'input.txt'");

    let mut sum : i32;
    let mut prev : i32 = 99999;
    let mut bigger : i32 = 0;

    let lines : Vec<&str> = input_file.lines().collect();

    for i in 0..(lines.len()-2) {
        sum = 0;
        for j in 0..3 {
            let n = lines[i+j].parse::<i32>().unwrap();
            // print!("n={} ", n);
            sum += n;
        }
        // println!("| sum={} prev={}", sum, prev);
        if sum > prev {
            bigger += 1;
        }
        prev = sum;
    }

    println!("Found {} increases!", bigger);
}
