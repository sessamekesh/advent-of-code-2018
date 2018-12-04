fn part_two(input: &str) {
    use std::collections::HashSet;

    // I happen to know that the input consists of all 26-length things, soooo that's nice
    let id_len = 26;

    let mut tokens = input.split("\n").collect::<Vec<_>>();

    // Also, the prompt gives a pretty huge hint into a potential solution of this that runs
    //  in O(id_len * id_ct) - run through, delete a different character, add to set, look for
    //  duplicates.
    let mut result: Option<_> = None;
    'outerloop: for remove_char in 0..id_len {
        let mut partials = HashSet::new();
        for token in &tokens {
            // There's got to be a better way to trim_right haha. Like a mapping function or something.
            let actual_token = token.trim_right();
            let head = actual_token.get(0..remove_char).expect("Could not get head");
            let tail = actual_token.get((remove_char+1)..).expect("Could not get tail");
            let combined_token = format!("{}{}", head, tail);
            if partials.contains(&combined_token) {
                result = Some(combined_token);
                break 'outerloop;
            }
            partials.insert(combined_token);
        }
    }

    result.map(|rsl| println!("Part two result: {}", rsl)).expect("No value found for part two! Perhaps there was a bug?");
}

fn part_one(input: &str) {
    let mut two_ct = 0u32;
    let mut three_ct = 0u32;
    for token in input.split("\n") {
        let mut cts: [i32; 26] = [0; 26];
        for letter in token.trim_right().chars() {
            let a = letter.to_digit(36).expect("Could not parse as digit") - 10u32;
            cts[a as usize] += 1;
        }
        let mut has_two: bool = false;
        let mut has_three: bool = false;
        for idx in 0..26 {
            has_two = has_two || cts[idx] == 2;
            has_three = has_three || cts[idx] == 3;
        }
        if has_two {
            two_ct += 1;
        }
        if has_three {
            three_ct += 1;
        }
    }
    println!("Part one result: {}", two_ct * three_ct);
}

fn main() {
    use std::fs::File;
    use std::io::prelude::*;
    
    // Gather input
    let mut in_file = File::open("02_input.txt").expect("Input file in_file.txt not found");
    let mut raw_input = String::new();
    in_file.read_to_string(&mut raw_input).expect("Failed to read file in_file.txt");
    
    // Trim input (trailing newline character)
    let trim_length = raw_input.trim_right().len();
    raw_input.truncate(trim_length);

    let input = raw_input.get(0..).expect("Apparently the raw input was empty");

    part_one(input);
    part_two(input);
}