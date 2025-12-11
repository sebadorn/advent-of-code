use std::fs;


fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read file.");
    let lines: Vec<&str> = content.trim().split("\n").collect();

    let mut sum_2 = 0;

    for line in lines {
        let batteries: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut high_one = (0, 0);
        let mut high_two = (0, 0);

        for (index, value) in batteries.iter().enumerate() {
            if high_one.0 < *value {
                high_one.0 = *value;
                high_one.1 = index;
            }
        }

        // Found at end, search to the left
        if high_one.1 == batteries.len() - 1 {
            for (index, value) in batteries[0..high_one.1].iter().enumerate().rev() {
                if high_two.0 < *value {
                    high_two.0 = *value;
                    high_two.1 = index;
                }
            }
        }
        else {
            for (index, value) in batteries[(high_one.1 + 1)..].iter().enumerate() {
                if high_two.0 < *value {
                    high_two.0 = *value;
                    high_two.1 = index;
                }
            }

            high_two.1 += high_one.1 + 1;
        }

        let first = if high_one.1 < high_two.1 { high_one.0 } else { high_two.0 };
        let second = if high_one.1 > high_two.1 { high_one.0 } else { high_two.0 };

        sum_2 += first * 10 + second;
    }

    println!("(Part 1) Best 2-joltage output: {}", sum_2);
}
