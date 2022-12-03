#![allow(unused_mut)]
#![allow(unused_variables)]

use std::fs;


fn get_badge(rucksack_1: &str, rucksack_2: &str, rucksack_3: &str) -> char {
    let mut all: Vec<&str> = vec![rucksack_1, rucksack_2, rucksack_3];
    all.sort_by(|a, b| a.len().cmp(&b.len()));

    let rucksack_least = all[0];
    let mut badge = '-';

    for item_type in rucksack_least.chars() {
        if
            all[1].contains(item_type) &&
            all[2].contains(item_type)
        {
            badge = item_type;
            break;
        }
    }

    if badge == '-' {
        panic!("Badge not found!");
    }

    badge
}


fn get_prio(item_type: char) -> i32 {
    // a to z would be 97 to 122 as i32.
    if item_type.is_lowercase() {
        return item_type as i32 - 96;
    }

    // A to Z would be 65 to 90 as i32.
    return item_type as i32 - 38;
}


fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt.");

    let mut prio_sum = 0;
    let mut group_sum = 0;
    let lines: Vec<&str> = content.lines().collect();

    for (line_num, line) in lines.iter().enumerate() {
        let num_items = line.len();
        let index_mid = num_items / 2;
        let compartment_1 = line.get(0..index_mid).unwrap();
        let compartment_2 = line.get(index_mid..).unwrap();

        // If this happens, there is a mistake in the input file.
        if compartment_1.len() != compartment_2.len() {
            panic!(
                "Compartments do not have the same size! -> {} and {}",
                compartment_1.len(), compartment_2.len()
            );
        }

        for item_type in compartment_1.chars() {
            if compartment_2.contains(item_type) {
                prio_sum += get_prio(item_type);
                break;
            }
        }

        // New group
        if line_num % 3 == 0 {
            let badge = get_badge(
                lines[line_num],
                lines[line_num + 1],
                lines[line_num + 2]
            );

            group_sum += get_prio(badge);
        }
    }

    println!("The sum of all duplicate item priorities is {}.", prio_sum);
    println!("The sum of all group badges is {}.", group_sum);
}
