use std::fs;

const CHAR_LEN : usize = 5;

fn binary_to_decimal(s : &str) -> i32 {
     return i32::from_str_radix(s, 2)
         .expect("Could not parse as binary");
}

fn filter_vec(mut v : Vec<&str>, filter : fn(f64, f64) -> char) -> Vec<&str> {
    for i in 0..CHAR_LEN {
        let mut ones : f64 = 0.0;
        for num in v.iter() {
            if num.chars().nth(i).unwrap() == '1' {
                ones += 1.0;
            }
        }

        let filter_ch : char = filter(ones, v.len() as f64);
        v = v.into_iter().filter(|s| s.chars().nth(i).unwrap() == filter_ch).collect();
        if v.len() == 1 {
            break;
        }
    }

    return v;
}

fn ones_more_common(ones : f64, l : f64) -> char {
    if ones < l / 2.0 { '0' } else { '1' }
}

fn ones_less_common(ones : f64, l : f64) -> char {
    if ones >= l / 2.0 { '0' } else { '1' }
}

fn main() {
    let input_file = fs::read_to_string("input.txt").
        expect("Could not load 'input.txt'");

    let mut oxygen : Vec<&str> = input_file.lines().collect(); // most common filter
    oxygen = filter_vec(oxygen, ones_more_common);
    let oxygen_int = binary_to_decimal(oxygen.get(0).unwrap());

    let mut co2 : Vec<&str> = input_file.lines().collect(); // least common filter
    co2 = filter_vec(co2, ones_less_common);
    let co2_int = binary_to_decimal(co2.get(0).unwrap());

    println!("Oxygen: {}", oxygen_int);
    println!("CO2: {}", co2_int);
    println!("Oxygen x CO2: {}", oxygen_int * co2_int);
}
