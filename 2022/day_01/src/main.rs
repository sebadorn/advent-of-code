#![allow(unused_variables)]

use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Expected a list of numbers");
    let elves: Vec<&str> = contents.split("\n\n").collect();

    let mut list_calories: Vec<u32> = Vec::new();

    for (index, elf) in elves.iter().enumerate() {
        let calories: Vec<&str> = elf.split("\n").collect();
        let mut total_calories = 0;

        for calorie in calories.iter() {
            let value = calorie.parse::<u32>();

            total_calories += match value {
                Err(err) => 0,
                Ok(cal) => cal,
            }
        }

        list_calories.push(total_calories);
    }

    list_calories.sort();
    list_calories.reverse();

    let top_three_sum = list_calories[0] + list_calories[1] + list_calories[2];

    println!(
        "The most an elf carries is {} calories.",
        list_calories[0]
    );
    println!(
        "The top three elves carry {} calories in total.",
        top_three_sum
    );
}
