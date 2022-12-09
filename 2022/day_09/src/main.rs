use std::fs;


fn follow(tail: &mut (i32, i32), head: &(i32, i32)) {
    let diff_x = head.0 - tail.0;
    let diff_y = head.1 - tail.1;

    // Touching
    if diff_x.abs() <= 1 && diff_y.abs() <= 1 {
        return;
    }

    // Move only up/down
    if diff_x == 0 {
        let sign_y = if diff_y > 1 { 1 } else { -1 };
        tail.1 += sign_y;
    }
    // Move only left/right
    else if diff_y == 0 {
        let sign_x = if diff_x > 1 { 1 } else { -1 };
        tail.0 += sign_x;
    }
    // Diagonally
    else {
        let sign_x = if diff_x > 0 { 1 } else { -1 };
        let sign_y = if diff_y > 0 { 1 } else { -1 };
        tail.0 += sign_x;
        tail.1 += sign_y;
    }
}


fn get_unique(list: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    // Make the list of visited coordinates unique
    // visits.dedup_by() did not work correctly,
    // probably because it was not sorted and I am
    // not sure what to sort by.
    let mut unique: Vec<(i32, i32)> = vec![];

    for coord in list.into_iter() {
        let found = unique.iter().find(|a| a.0 == coord.0 && a.1 == coord.1);

        if found.is_none() {
            unique.push(*coord);
        }
    }

    unique
}


fn move_head(direction: &str, head: &mut (i32, i32)) {
    match direction {
        "U" => head.1 += 1,
        "D" => head.1 -= 1,
        "R" => head.0 += 1,
        "L" => head.0 -= 1,
        _ => (),
    }
}


fn simulate_part1(content: &String) {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visits: Vec<(i32, i32)> = vec![tail];

    for line in content.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let direction = parts[0];
        let steps = parts[1].parse::<i32>().unwrap();

        for _i in 1..=steps {
            move_head(&direction, &mut head);
            follow(&mut tail, &head);
            visits.push(tail);
        }
    }

    let unique = get_unique(&visits);
    println!("Part 1: The tail has visited {} places.", unique.len());
}


fn simulate_part2(content: &String) {
    let mut head = (0, 0);
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); 9];
    let mut visits: Vec<(i32, i32)> = vec![knots[8]];

    for line in content.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let direction = parts[0];
        let steps = parts[1].parse::<i32>().unwrap();

        for _step in 1..=steps {
            move_head(&direction, &mut head);
            follow(&mut knots[0], &head);

            for i in 1..knots.len() {
                let mut knot_now = knots[i];
                let knot_before = knots[i - 1];
                follow(&mut knot_now, &knot_before);
                knots[i] = knot_now;
            }

            visits.push(*knots.last().unwrap());
        }
    }

    let unique = get_unique(&visits);
    println!("Part 2: The tail has visited {} places.", unique.len());
}


fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt.");

    simulate_part1(&content);
    simulate_part2(&content);
}
