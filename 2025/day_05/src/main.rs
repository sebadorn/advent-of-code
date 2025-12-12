use std::fs;


fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read file.");
    let lines: Vec<&str> = content.trim().split("\n").collect();

    let mut list_fresh = Vec::<(u64, u64)>::new();
    let mut list_avail = Vec::<u64>::new();
    let mut do_switch = false;

    for line in lines {
        if line.len() == 0 {
            do_switch = true;
            continue;
        }

        if !do_switch {
            let parts: Vec<&str> = line.split("-").collect();
            list_fresh.push((
                parts[0].parse::<u64>().unwrap(),
                parts[1].parse::<u64>().unwrap()
            ));
        }
        else {
            list_avail.push(line.parse::<u64>().unwrap());
        }
    }

    let mut sum_is_fresh = 0;

    for item in list_avail {
        for entry in &list_fresh {
            if item >= entry.0 && item <= entry.1 {
                sum_is_fresh += 1;
                break;
            }
        }
    }

    println!("(Part 1) Fresh ingredients: {}", sum_is_fresh);
}
