use std::fs;


fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read file.");
    let lines: Vec<&str> = content.trim().split("\n").collect();

    let mut pos_old = 50;
    let mut zero_counter_turn = 0;
    let mut zero_counter_click = 0;

    for (_index, line) in lines.iter().enumerate() {
        let dir = line.chars().nth(0).unwrap();
        let steps = line[1..].parse::<i32>().unwrap();
        let sign = if dir == 'L' { -1 } else { 1 };

        let full_rotations = steps / 100;
        let rest = steps - full_rotations * 100;
        let mut pos_new = pos_old + rest * sign;

        zero_counter_click += full_rotations;

        if pos_old != 0 && pos_new == 0 {
            zero_counter_click += 1;
        }

        if pos_new < 0 {
            pos_new += 100;

            if pos_old != 0 {
                zero_counter_click += 1;
            }
        }

        if pos_new > 99 {
            pos_new -= 100;

            if pos_old != 0 {
                zero_counter_click += 1;
            }
        }

        if pos_new == 0 {
            zero_counter_turn += 1;
        }

        pos_old = pos_new;
    }

    println!("(Part 1) The dial ended on zero {} times.", zero_counter_turn);
    println!("(Part 2) The dial moved across zero {} times.", zero_counter_click);
}
