use std::fs;


fn get_number_from_line_part1(line: &str) -> u32 {
    let mut has_first = false;
    let mut first: char = '0';
    let mut last: char = '0';

    for char in line.chars() {
        if char.is_digit(10) {
            if !has_first {
                first = char;
                has_first = true;
            }

            last = char;
        }
    }

    let num_str = format!("{}{}", first, last);

    return num_str.parse::<u32>().unwrap();
}


fn get_number_from_line_part2(line: &str) -> u32 {
    let mut numbers: Vec<(usize, usize)> = Vec::new();
    let needles = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for (value, needle) in needles.iter().enumerate() {
        let mut line_copy = line;
        let mut offset = 0;

        loop {
            let result = line_copy.find(needle);

            if result.is_none() {
                break;
            }

            let pos = result.unwrap();
            numbers.push((offset + pos, value));

            let new_start = pos + needle.len();
            line_copy = &line_copy[new_start..];

            offset += new_start;
        }
    }

    for (index, char) in line.chars().enumerate() {
        if char.is_digit(10) {
            numbers.push((index, char.to_digit(10).unwrap() as usize));
        }
    }

    numbers.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let num_str = format!("{}{}", numbers.first().unwrap().1, numbers.last().unwrap().1);

    return num_str.parse::<u32>().unwrap();
}


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file.");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;

    for (_index, line) in lines.iter().enumerate() {
        if line.len() == 0 {
            continue;
        }

        let num1 = get_number_from_line_part1(line);
        sum1 += num1;

        let num2 = get_number_from_line_part2(line);
        sum2 += num2;
    }

    println!("(Part 1) The sum is: {}", sum1);
    println!("(Part 2) The sum is: {}", sum2);
}
