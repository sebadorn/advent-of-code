use std::fs;


fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read file.");
    let lines: Vec<&str> = content.trim().split("\n").collect();
    let mut map: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let mut row = Vec::new();

        for c in line.chars() {
            row.push(if c == '@' { 1 } else { 0 });
        }

        map.push(row);
    }

    let height = map.len();
    let width = map[0].len();
    let mut counter = 0;

    for (y, line) in map.iter().enumerate() {
        for (x, item) in line.iter().enumerate() {
            if *item != 1 {
                continue;
            }

            let mut neighbour_rolls = 0;

            // Starting at top, going clockwise
            neighbour_rolls += if y > 0 { map[y - 1][x] } else { 0 };
            neighbour_rolls += if y > 0 && x + 1 < width { map[y - 1][x + 1] } else { 0 };
            neighbour_rolls += if x + 1 < width { map[y][x + 1] } else { 0 };
            neighbour_rolls += if y + 1 < height && x + 1 < width { map[y + 1][x + 1] } else { 0 };
            neighbour_rolls += if y + 1 < height { map[y + 1][x] } else { 0 };
            neighbour_rolls += if y + 1 < height && x > 0 { map[y + 1][x - 1] } else { 0 };
            neighbour_rolls += if x > 0 { map[y][x - 1] } else { 0 };
            neighbour_rolls += if y > 0 && x > 0 { map[y - 1][x - 1] } else { 0 };

            if neighbour_rolls < 4 {
                counter += 1;
            }
        }
    }

    println!("(Part 1) Accessible paper rolls: {}", counter);
}
