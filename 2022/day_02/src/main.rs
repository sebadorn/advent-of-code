#![allow(unused_mut)]
#![allow(unused_variables)]

use std::fs;

enum HandValue {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum ResultValue {
    Lost = 0,
    Draw = 3,
    Win = 6,
}


fn get_score_part1(play_opponent: &str, play_self: &str) -> i32 {
    // Rock
    if play_opponent == "A" {
        return match play_self {
            // Rock
            "X" => HandValue::Rock as i32 + ResultValue::Draw as i32,
            // Paper
            "Y" => HandValue::Paper as i32 + ResultValue::Win as i32,
            // Scissors
            "Z" => HandValue::Scissors as i32 + ResultValue::Lost as i32,
            value => panic!("Unexpected value: {:?}", value),
        }
    }
    // Paper
    else if play_opponent == "B" {
        return match play_self {
            // Rock
            "X" => HandValue::Rock as i32 + ResultValue::Lost as i32,
            // Paper
            "Y" => HandValue::Paper as i32 + ResultValue::Draw as i32,
            // Scissors
            "Z" => HandValue::Scissors as i32 + ResultValue::Win as i32,
            value => panic!("Unexpected value: {:?}", value),
        }
    }
    // Scissors
    else if play_opponent == "C" {
        return match play_self {
            // Rock
            "X" => HandValue::Rock as i32 + ResultValue::Win as i32,
            // Paper
            "Y" => HandValue::Paper as i32 + ResultValue::Lost as i32,
            // Scissors
            "Z" => HandValue::Scissors as i32 + ResultValue::Draw as i32,
            value => panic!("Unexpected value: {:?}", value),
        }
    }

    panic!("Unexpected situation.")
}


fn get_score_part2(play_opponent: &str, end: &str) -> i32 {
    // Rock
    if play_opponent == "A" {
        return match end {
            // Lose
            "X" => HandValue::Scissors as i32 + ResultValue::Lost as i32,
            // Draw
            "Y" => HandValue::Rock as i32 + ResultValue::Draw as i32,
            // Win
            "Z" => HandValue::Paper as i32 + ResultValue::Win as i32,
            value => panic!("Unexpected value: {:?}", value),
        }
    }
    // Paper
    else if play_opponent == "B" {
        return match end {
            // Lose
            "X" => HandValue::Rock as i32 + ResultValue::Lost as i32,
            // Draw
            "Y" => HandValue::Paper as i32 + ResultValue::Draw as i32,
            // Win
            "Z" => HandValue::Scissors as i32 + ResultValue::Win as i32,
            value => panic!("Unexpected value: {:?}", value),
        }
    }
    // Scissors
    else if play_opponent == "C" {
        return match end {
            // Lose
            "X" => HandValue::Paper as i32 + ResultValue::Lost as i32,
            // Draw
            "Y" => HandValue::Scissors as i32 + ResultValue::Draw as i32,
            // Win
            "Z" => HandValue::Rock as i32 + ResultValue::Win as i32,
            value => panic!("Unexpected value: {:?}", value),
        }
    }

    panic!("Unexpected situation.")
}


fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt.");

    let mut score_part1 = 0;
    let mut score_part2 = 0;

    for line in content.lines() {
        let play_opponent = line.get(0..1).unwrap();
        let column2 = line.get(2..3).unwrap();

        score_part1 += get_score_part1(play_opponent, column2);
        score_part2 += get_score_part2(play_opponent, column2);
    }

    println!("Final score (part 1): {}", score_part1);
    println!("Final score (part 2): {}", score_part2);
}
