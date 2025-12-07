use std::fs;


fn is_invalid(num: usize) -> (bool, bool) {
    let mut part1 = false;
    let mut part2 = false;

    if num < 10 {
        return (false, false);
    }

    let length = f64::log10(num as f64).floor() as usize + 1;

    if length % 2 == 0 {
        let d = (10 as usize).pow((length / 2).try_into().unwrap());
        let last_half = num % d;
        let first_half = (num - last_half) / d;

        part1 = first_half == last_half;

        if part1 {
            return (true, true);
        }
    }

    let num_str = num.to_string();

    for i in 0..(length / 2) {
        let search = num_str[0..(i + 1)].to_string();

        // Single digit
        if i == 0 {
            if search.repeat(length) == num_str {
                part2 = true;
                break;
            }
        }
        // Even length of search string
        else if i % 2 != 0 {
            if search.repeat(length / 2) == num_str {
                part2 = true;
                break;
            }
        }
        // Odd length of search string
        else if i % 2 == 0 {
            let max_rep = length / (i + 1);

            for repeat in 2..(max_rep + 1) {
                if search.repeat(repeat) == num_str {
                    part2 = true;
                    break;
                }
            }
        }

        if part2 {
            break;
        }
    }

    (part1, part2)
}


fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read file.");
    let entries: Vec<&str> = content.trim().split(",").collect();

    let mut invalid_sum_part1 = 0;
    let mut invalid_sum_part2 = 0;

    for entry in entries {
        let ids: Vec<&str> = entry.split("-").collect();
        let first_id = ids[0].parse::<usize>().unwrap();
        let last_id = ids[1].parse::<usize>().unwrap();

        println!("Checking entry: {}", entry);

        for i in first_id..(last_id + 1) {
            let invalids = is_invalid(i);

            if invalids.0 {
                invalid_sum_part1 += i;
            }

            if invalids.1 {
                invalid_sum_part2 += i;
            }
        }
    }

    println!("(Part 1) Sum of all invalid entries: {}", invalid_sum_part1);
    println!("(Part 2) Sum of all invalid entries: {}", invalid_sum_part2);
}
