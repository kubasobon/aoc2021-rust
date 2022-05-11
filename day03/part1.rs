use std::fs;

fn binary_to_decimal(b : &[char]) -> i32 {
     let b_str : String = b.iter().collect();
     return i32::from_str_radix(&b_str, 2)
         .expect("Could not parse as binary");
}

fn main() {
    let input_file = fs::read_to_string("input.txt").
        expect("Could not load 'input.txt'");

    let mut gamma_b : [char; 5] = ['0'; 5]; // most common
    let mut epsilon_b : [char; 5] = ['0'; 5]; // least common
    let mut ones : [i32; 5] = [0; 5]; // ones per column

    for line in input_file.lines() {
        for (i, ch) in line.trim().chars().enumerate() {
            if ch == '1' {
                ones[i] += 1;
            }
        }
    }
    let row_count : i32 = input_file.lines().count() as i32;

    for (i, total) in ones.iter().enumerate() {
        if *total > row_count/2 {
            gamma_b[i] = '1';
            epsilon_b[i] = '0';
        } else {
            gamma_b[i] = '0';
            epsilon_b[i] = '1';
        }
    }

    let gamma_decimal = binary_to_decimal(&gamma_b);
    let epsilon_decimal = binary_to_decimal(&epsilon_b);
    println!("Gamma: {}\nEpsilon: {}\nGamma x Epsilon: {}",
        gamma_decimal, epsilon_decimal, gamma_decimal * epsilon_decimal);
}
