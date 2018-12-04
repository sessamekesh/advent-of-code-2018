fn extract_value(token: &str) -> i32 {
    let coefficient = match token.get(0..1).expect("Token does not have a parity value") {
        "+" => 1i32,
        "-" => -1i32,
        other_val => panic!(format!("{} is not expected value of + or -", other_val)),
    };

    let maybe_number = token.get(1..).expect("No numeric portion of the token found")
        .trim_right()
        .parse::<i32>().expect("Cannot parse numeric portion into an i32");

    coefficient * maybe_number
}

fn part_one(input: &str) {
    let tokens = input.split("\n");

    let mut accum: i32 = 0i32;
    for token in tokens {
        accum += extract_value(token);
    }

    println!("Part one result: {}", accum)
}

fn part_two(input: &str) {
    use std::collections::HashSet;

    let mut previous_frequencies = HashSet::new();
    previous_frequencies.insert(0i32);

    let mut accum: i32 = 0i32;
    let mut result: Option<i32> = None;

    let tokens = input.split("\n").collect::<Vec<_>>();
    'outerloop: loop {
        for token in &tokens {
            accum += extract_value(token);
            if previous_frequencies.contains(&accum) {
                result = Some(accum);
                break 'outerloop;
            }
            previous_frequencies.insert(accum);
        }
    }

    result.map(|rsl| println!("Part two result: {}", rsl)).expect("No value found for part two! Perhaps there was a bug?");
}

fn main() {
    use std::fs::File;
    use std::io::prelude::*;
    
    // Gather input
    let mut in_file = File::open("01_input.txt").expect("Input file in_file.txt not found");
    let mut raw_input = String::new();
    in_file.read_to_string(&mut raw_input).expect("Failed to read file in_file.txt");
    
    // Trim input (trailing newline character)
    let trim_length = raw_input.trim_right().len();
    raw_input.truncate(trim_length);

    let input = raw_input.get(0..).expect("Apparently the raw input was empty");
    part_one(input);
    part_two(input);
}