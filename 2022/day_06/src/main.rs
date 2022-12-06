#![allow(unused_variables)]

use std::fs;


fn find_marker(text: &String, num_diff: usize) -> usize {
    let mut index = 0;
    let mut window = &text[0..num_diff];

    loop {
        if is_distinct(&window) {
            break;
        }

        index += 1;
        let end = index + num_diff;
        window = &text[index..end];
    }

    index + num_diff
}


fn is_distinct(window: &str) -> bool {
    let mut last_chars: Vec<char> = Vec::new();

    for c in window.chars() {
        if last_chars.contains(&c) {
            return false;
        }

        last_chars.push(c);
    }

    true
}


fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt.");

    let index_packet = find_marker(&content, 4);
    println!("The start-of-packet marker was found at position {}.", index_packet);

    let index_message = find_marker(&content, 14);
    println!("The start-of-message marker was found at position {}.", index_message);
}
