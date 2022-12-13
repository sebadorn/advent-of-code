use std::fs;


fn find_way(
    grid: &Vec<Vec<i32>>, step_grid: &mut Vec<Vec<i32>>,
    start: &(usize, usize), goal: &(usize, usize)
) {
    if start.0 == goal.0 && start.1 == goal.1 {
        return;
    }

    let grid_h = grid.len();
    let grid_w = grid[0].len();

    let x = start.0;
    let y = start.1;

    let height_from = grid[y][x];
    let next_step = step_grid[y][x] + 1;

    // Go right
    if x < grid_w - 1 {
        let target_step = step_grid[y][x + 1];

        if target_step == -1 || target_step > next_step {
            let height_to = grid[y][x + 1];

            if height_from + 1 >= height_to {
                step_grid[y][x + 1] = next_step;
                find_way(grid, step_grid, &(x + 1, y), goal);
            }
        }
    }

    // Go left
    if x > 0 {
        let target_step = step_grid[y][x - 1];

        if target_step == -1 || target_step > next_step {
            let height_to = grid[y][x - 1];

            if height_from + 1 >= height_to {
                step_grid[y][x - 1] = next_step;
                find_way(grid, step_grid, &(x - 1, y), goal);
            }
        }
    }

    // Go up
    if y > 0 {
        let target_step = step_grid[y - 1][x];

        if target_step == -1 || target_step > next_step {
            let height_to = grid[y - 1][x];

            if height_from + 1 >= height_to {
                step_grid[y - 1][x] = next_step;
                find_way(grid, step_grid, &(x, y - 1), goal);
            }
        }
    }

    // Go down
    if y < grid_h - 1 {
        let target_step = step_grid[y + 1][x];

        if target_step == -1 || target_step > next_step {
            let height_to = grid[y + 1][x];

            if height_from + 1 >= height_to {
                step_grid[y + 1][x] = next_step;
                find_way(grid, step_grid, &(x, y + 1), goal);
            }
        }
    }
}


fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt.");

    let mut grid: Vec<Vec<i32>> = vec![];
    let mut start: (usize, usize) = (0, 0);
    let mut goal: (usize, usize) = (0, 0);

    for (y, line) in content.lines().enumerate() {
        let mut grid_line: Vec<i32> = vec![];

        for (x, c) in line.chars().enumerate() {
            let height = match c {
                'S' => {
                    start = (x, y);
                    1
                },
                'E' => {
                    goal = (x, y);
                    26
                },
                c => c as i32 - 96,
            };

            grid_line.push(height);
        }

        grid.push(grid_line);
    }

    let height = grid.len();
    let width = grid[0].len();

    println!("The grid has a size of: {:?}", (width, height));
    println!("Starting point is at: {:?}", start);
    println!("Goal is at: {:?}", goal);

    let mut step_grid = vec![vec![-1; width]; height];
    step_grid[start.1][start.0] = 0;
    find_way(&grid, &mut step_grid, &start, &goal);

    println!("Shortest route to goal: {} steps", step_grid[goal.1][goal.0]);
    println!();


    println!("Searching for shortest starting point...");

    let mut shortest = -1;
    let mut checks = 0;

    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != 'a' && c != 'S' {
                continue;
            }

            checks += 1;

            let mut step_grid = vec![vec![-1; width]; height];
            step_grid[y][x] = 0;
            find_way(&grid, &mut step_grid, &(x, y), &goal);
            let num_steps = step_grid[goal.1][goal.0];

            // No path found
            if num_steps == -1 {
                continue;
            }

            if shortest == -1 || num_steps < shortest {
                shortest = num_steps;
            }
        }

        println!("  Finished line: {}", y);
    }

    println!("Checked {} starting points.", checks);
    println!("The shortest route from an \"a\" starting point is {} steps.", shortest);
}
