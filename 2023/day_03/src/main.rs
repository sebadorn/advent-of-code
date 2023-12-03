use std::fs;


fn check_left(x: &usize, y: &usize, lines: &Vec<&str>) -> bool {
    if *x == 0 {
        return false;
    }

    return is_symbol(*x - 1, *y, lines);
}


fn check_top_left(x: &usize, y: &usize, lines: &Vec<&str>) -> bool {
    if *x == 0 || *y == 0 {
        return false;
    }

    return is_symbol(*x - 1, *y - 1, lines);
}


fn check_top_right(x: usize, y: &usize, lines: &Vec<&str>) -> bool {
    if *y == 0 || x + 1 >= lines[*y].len() {
        return false;
    }

    return is_symbol(x + 1, *y - 1, lines);
}


fn check_right(x: usize, y: &usize, lines: &Vec<&str>) -> bool {
    if x + 1 >= lines[*y].len() {
        return false;
    }

    return is_symbol(x + 1, *y, lines);
}


fn check_top(x: &usize, y: &usize, w: &usize, lines: &Vec<&str>) -> bool {
    if *y == 0 {
        return false;
    }

    for i in *x..(*x + *w) {
        if is_symbol(i, *y - 1, lines) {
            return true;
        }
    }

    return false;
}


fn check_bottom(x: &usize, y: &usize, w: &usize, lines: &Vec<&str>) -> bool {
    if *y + 1 >= lines.len() {
        return false;
    }

    for i in *x..(*x + *w) {
        if is_symbol(i, *y + 1, lines) {
            return true;
        }
    }

    return false;
}


fn check_bottom_left(x: &usize, y: &usize, lines: &Vec<&str>) -> bool {
    if *x == 0 || *y + 1 >= lines.len() {
        return false;
    }

    return is_symbol(*x - 1, *y + 1, lines);
}


fn check_bottom_right(x: usize, y: &usize, lines: &Vec<&str>) -> bool {
    if *y + 1 >= lines.len() || x + 1 >= lines[*y + 1].len() {
        return false;
    }

    return is_symbol(x + 1, *y + 1, lines);
}


fn close_to_symbol(x: usize, y: &usize, w: &usize, lines: &Vec<&str>) -> bool {
    let x_end = x + *w - 1;

    return check_left(&x, y, lines) ||
        check_right(x_end, y, lines) ||
        check_top(&x, y, w, lines) ||
        check_bottom(&x, y, w, lines) ||
        check_top_left(&x, y, lines) ||
        check_top_right(x_end, y, lines) ||
        check_bottom_left(&x, y, lines) ||
        check_bottom_right(x_end, y, lines);
}


fn is_symbol(x: usize, y: usize, lines: &Vec<&str>) -> bool {
    // Should only be used with ASCII
    let ch = lines[y].bytes().nth(x).unwrap() as char;

    return ch != '.' && !ch.is_digit(10);
}


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file.");
    let lines: Vec<&str> = contents.trim().split("\n").collect();
    let mut sum = 0;

    for (y, line) in lines.iter().enumerate() {
        let mut number = String::new();

        for (x, ch) in line.chars().enumerate() {
            let is_digit = ch.is_digit(10);
            let is_end_of_line = x + 1 == line.len();

            if is_digit {
                number.push(ch);
            }

            if (!is_digit && number.len() > 0) || (is_digit && is_end_of_line) {
                let mut x_start = x - number.len();

                if is_digit && is_end_of_line {
                    x_start += 1;
                }

                if close_to_symbol(x_start, &y, &number.len(), &lines) {
                    sum += number.parse::<i32>().unwrap();
                }

                number.clear();
            }
        }
    }

    println!("(Part 1) The sum of valid parts is: {}", sum);
}
