use std::cmp::Ordering;
use std::fs;
use serde_json::Value;


fn compare_arrays(left: &Value, right: &Value) -> i32 {
    let left_array = left.as_array().unwrap();
    let right_array = right.as_array().unwrap();

    for (i, item_l) in left_array.into_iter().enumerate() {
        // Right side is smaller -> wrong order
        if i >= right_array.len() {
            return 1;
        }

        let item_r = &right_array[i];

        if item_l.is_number() {
            // Compare number and number
            if item_r.is_number() {
                match compare_numbers(&item_l, &item_r) {
                    0 => continue,
                    result => return result,
                };
            }
            // Convert left to array, then compare arrays
            else if item_r.is_array() {
                let value = serde_json::to_value(item_l).unwrap();
                let arr_1 = &Value::Array(vec![value]);

                match compare_arrays(arr_1, &item_r) {
                    0 => continue,
                    result => return result,
                };
            }
            else {
                panic!("Unexpected value for right item. Left item is number.");
            }
        }
        else if item_l.is_array() {
            // Convert right to array, then compare arrays
            if item_r.is_number() {
                let value = serde_json::to_value(item_r).unwrap();
                let arr_2 = &Value::Array(vec![value]);

                match compare_arrays(&item_l, arr_2) {
                    0 => continue,
                    result => return result,
                };
            }
            // Compare array and array
            else if item_r.is_array() {
                match compare_arrays(&item_l, &item_r) {
                    0 => continue,
                    result => return result,
                };
            }
            else {
                panic!("Unexpected value for right item. Left item is array.");
            }
        }
        else {
            panic!("Unexpected value for left item.");
        }
    }

    if left_array.len() == right_array.len() {
        return 0;
    }

    -1
}


fn compare_numbers(left: &Value, right: &Value) -> i32 {
    let a = left.as_i64().unwrap();
    let b = right.as_i64().unwrap();

    if a < b { return -1; }
    if a > b { return 1; }
    0
}


fn is_order_right(pack_1_str: &str, pack_2_str: &str) -> bool {
    let pack_1: Value = serde_json::from_str(pack_1_str).unwrap();
    let pack_2: Value = serde_json::from_str(pack_2_str).unwrap();

    match compare_arrays(&pack_1, &pack_2) {
        1 => false,
        -1 => true,
        res => panic!("Unexpected result: {}", res),
    }
}


fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt.");

    let mut lines: Vec<&str> = content.lines().collect();
    let mut is_in_right_order = 0;
    let mut indices_sum = 0;

    for i in (0..lines.len()).step_by(3) {
        if is_order_right(lines[i], lines[i + 1]) {
            is_in_right_order += 1;
            indices_sum += i / 3 + 1;
        }
    }

    println!("{} pairs are in the right order.", is_in_right_order);
    println!("The sum of their indices is {}.", indices_sum);


    // Remove empty lines
    lines.retain(|l| l.len() > 0);

    let divider_packet_1 = "[[2]]";
    let divider_packet_2 = "[[6]]";
    lines.push(divider_packet_1);
    lines.push(divider_packet_2);

    lines.sort_by(|a, b| {
        match is_order_right(a, b) {
            true => Ordering::Less,
            false => Ordering::Greater,
        }
    });

    let pos_div_1 = lines.iter().position(|&a| a == divider_packet_1).unwrap() + 1;
    let pos_div_2 = lines.iter().position(|&a| a == divider_packet_2).unwrap() + 1;

    println!(
        "The divider packages are at indices {} and {}. Decoder key: {}",
        pos_div_1, pos_div_2, pos_div_1 * pos_div_2
    );
}
