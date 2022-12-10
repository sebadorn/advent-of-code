use std::fs;


fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt.");

    let mut cycle = 1;
    let mut x_by_cycle: [i32; 241] = [1; 241];

    for line in content.lines() {
        println!("{} -> {}", cycle, line);

        if line == "noop" {
            x_by_cycle[cycle] = x_by_cycle[cycle - 1];
            cycle += 1;
            continue;
        }

        let value = &line[5..];
        let current_x = x_by_cycle[cycle - 1];
        x_by_cycle[cycle] = current_x;
        cycle += 1;
        x_by_cycle[cycle] = current_x + value.parse::<i32>().unwrap();
        cycle += 1;
    }


    let mut signal_strength_sum = 0;

    for i in (20..=220).step_by(40) {
        let signal = i as i32 * x_by_cycle[i - 1];
        signal_strength_sum += signal;
        println!("During cycle {} the signal strength is {}.", i, signal);
    }

    println!("The signal strength sum is {}.", signal_strength_sum);


    let mut image = String::new();

    for (cycle, x) in x_by_cycle.into_iter().enumerate() {
        if cycle == x_by_cycle.len() - 1 {
            break;
        }

        let row_index = cycle as i32 % 40;

        if cycle > 1 && row_index == 0 {
            image.push('\n');
        }

        if x == row_index || x - 1 == row_index || x + 1 == row_index {
            image.push('#');
        }
        else {
            image.push('.');
        }
    }

    println!("\nThe CTR image reads:\n{}", image);
}
