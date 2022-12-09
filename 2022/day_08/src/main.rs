use std::fs;


fn get_score(coords: (usize, usize), grid: &Vec<Vec<u32>>) -> i32 {
    let height = grid[coords.0][coords.1];

    let mut index_y = coords.0 as i32;
    let mut index_x = coords.1 as i32;
    let size = grid.len() as i32;

    let mut score_left = 0;
    let mut score_right = 0;
    let mut score_top = 0;
    let mut score_bottom = 0;

    // Check to the left
    while index_x > 0 {
        index_x -= 1;
        score_left += 1;
        let h = grid[coords.0][index_x as usize];

        if h >= height {
            break;
        }
    }

    index_x = coords.1 as i32;

    // Check to the right
    while index_x < size - 1 {
        index_x += 1;
        score_right += 1;
        let h = grid[coords.0][index_x as usize];

        if h >= height {
            break;
        }
    }

    // Check to the top
    while index_y > 0 {
        index_y -= 1;
        score_top += 1;
        let h = grid[index_y as usize][coords.1];

        if h >= height {
            break;
        }
    }

    index_y = coords.0 as i32;

    // Check to the bottom
    while index_y < size - 1 {
        index_y += 1;
        score_bottom += 1;
        let h = grid[index_y as usize][coords.1];

        if h >= height {
            break;
        }
    }

    // answer 132 is too low

    score_left * score_right * score_top * score_bottom
}

fn is_visible(coords: (usize, usize), grid: &Vec<Vec<u32>>) -> bool {
    let height = grid[coords.0][coords.1];

    let mut index_y = coords.0 as i32;
    let mut index_x = coords.1 as i32;
    let size = grid.len() as i32;

    let mut is_visible_left = true;
    let mut is_visible_right = true;
    let mut is_visible_top = true;
    let mut is_visible_bottom = true;

    // Check to the left
    while index_x > 0 {
        index_x -= 1;
        let h = grid[coords.0][index_x as usize];

        if h >= height {
            is_visible_left = false;
            break;
        }
    }

    index_x = coords.1 as i32;

    // Check to the right
    while index_x < size - 1 {
        index_x += 1;
        let h = grid[coords.0][index_x as usize];

        if h >= height {
            is_visible_right = false;
            break;
        }
    }

    // Check to the top
    while index_y > 0 {
        index_y -= 1;
        let h = grid[index_y as usize][coords.1];

        if h >= height {
            is_visible_top = false;
            break;
        }
    }

    index_y = coords.0 as i32;

    // Check to the bottom
    while index_y < size - 1 {
        index_y += 1;
        let h = grid[index_y as usize][coords.1];

        if h >= height {
            is_visible_bottom = false;
            break;
        }
    }

    return is_visible_left || is_visible_right || is_visible_top || is_visible_bottom;
}


fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt.");

    // Populate a height grid
    let mut grid: Vec<Vec<u32>> = Vec::new();

    for line in content.lines() {
        let mut row: Vec<u32> = Vec::new();

        for tree in line.chars() {
            let height = tree.to_digit(10).unwrap();
            row.push(height);
        }

        grid.push(row);
    }

    // Count visible trees
    let size = grid.len();
    let mut sum = ( size - 1 ) * 4; // Outer trees

    // Inside trees
    for y in 1..size - 1 {
        for x in 1..size - 1 {
            if is_visible((x, y), &grid) {
                sum += 1;
            }
        }
    }

    println!("{} trees are visible from outside.", sum);

    // Scenic scores
    let mut highest_score = 0;

    for y in 1..size - 1 {
        for x in 1..size - 1 {
            let score = get_score((x, y), &grid);

            if score > highest_score {
                highest_score = score;
            }
        }
    }

    println!("The best scenic score is {}.", highest_score);
}
