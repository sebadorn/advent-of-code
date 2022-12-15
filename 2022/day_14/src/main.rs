use std::fs;


#[derive(Clone)]
#[derive(PartialEq)]
enum Material {
    Air,
    Rock,
    Sand,
}


fn to_coords(part: &str) -> (i32, i32) {
    let as_str: Vec<&str> = part.split(',').collect();
    let x = as_str[0].parse::<i32>().unwrap();
    let y = as_str[1].parse::<i32>().unwrap();

    (x, y)
}


struct Simulator {
    grid: Vec<Vec<Material>>,
    width: usize,
    height: usize,
    min_coords: (usize, usize),
    max_coords: (usize, usize),
}


impl Simulator {

    fn new(min: (i32, i32), max: (i32, i32)) -> Self {
        let width = (max.0 - min.0) as usize + 1;
        let height = (max.1 - min.1) as usize + 1;

        Self {
            grid: vec![vec![Material::Air; width]; height],
            width: width,
            height: height,
            min_coords: (min.0 as usize, min.1 as usize),
            max_coords: (max.0 as usize, max.1 as usize),
        }
    }

    fn add_line(&mut self, from: &(i32, i32), to: &(i32, i32)) {
        let min_x = std::cmp::min(from.0, to.0) as usize - self.min_coords.0;
        let max_x = std::cmp::max(from.0, to.0) as usize - self.min_coords.0;

        let min_y = std::cmp::min(from.1, to.1) as usize - self.min_coords.1;
        let max_y = std::cmp::max(from.1, to.1) as usize - self.min_coords.1;

        let const_x = from.0 as usize - self.min_coords.0;
        let const_y = from.1 as usize - self.min_coords.1;

        for x in min_x..=max_x {
            self.grid[const_y][x] = Material::Rock;
        }

        for y in min_y..=max_y {
            self.grid[y][const_x] = Material::Rock;
        }
    }

    fn move_sand_to(&mut self, source: &(usize, usize)) -> bool {
        let x = source.0;
        let mut y = source.1;

        loop {
            if
                x >= self.width ||
                y >= self.height
            {
                return false;
            }

            let mat = &self.grid[y][x];

            if mat == &Material::Rock || mat == &Material::Sand {
                // Cannot go any further left, falling off
                if x == 0 {
                    return false;
                }

                let mat_left = &self.grid[y][x - 1];

                if mat_left == &Material::Rock || mat_left == &Material::Sand {
                    // Cannot go any further right, falling off
                    if x == self.width - 1 {
                        return false;
                    }

                    let mat_right = &self.grid[y][x + 1];

                    if mat_right == &Material::Rock || mat_right == &Material::Sand {
                        self.grid[y - 1][x] = Material::Sand;
                        break;
                    }
                    else {
                        return self.move_sand_to(&(x + 1, y));
                    }
                }
                else {
                    return self.move_sand_to(&(x - 1, y));
                }
            }
            else {
                y += 1;
            }
        }

        true
    }

    fn simulate_1(&mut self, mut source: (usize, usize)) -> i32 {
        source.0 -= self.min_coords.0;
        source.1 -= self.min_coords.1;

        let mut sand = 0;

        loop {
            sand += 1;

            match self.move_sand_to(&source) {
                true => (),
                false => break,
            }
        }

        sand
    }

    fn simulate_2(&mut self, mut source: (usize, usize)) -> i32 {
        source.0 -= self.min_coords.0;
        source.1 -= self.min_coords.1;

        let mut sand = 0;

        loop {
            sand += 1;

            match self.move_sand_to(&source) {
                true => {
                    let mat = &self.grid[source.1][source.0];

                    if mat == &Material::Sand {
                        println!("Sand is now blocking the source.");
                        break;
                    }
                },
                false => break,
            }
        }

        sand
    }

}


fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt.");

    let mut min = (-1, 0);
    let mut max = (-1, -1);

    for line in content.lines() {
        let parts = line.split(" -> ");

        for part in parts {
            let coords = to_coords(part);

            min.0 = if min.0 == -1 { coords.0 } else { std::cmp::min(min.0, coords.0) };
            min.1 = if min.1 == -1 { coords.1 } else { std::cmp::min(min.1, coords.1) };

            max.0 = std::cmp::max(max.0, coords.0);
            max.1 = std::cmp::max(max.1, coords.1);
        }
    }

    let mut sim_1 = Simulator::new(min, max);
    let mut sim_2 = Simulator::new((0, 0), (1000, max.1 + 2));

    for line in content.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();

        for i in 0..parts.len() {
            if i >= parts.len() - 1 {
                break;
            }

            let from = to_coords(parts[i]);
            let to = to_coords(parts[i + 1]);
            sim_1.add_line(&from, &to);
            sim_2.add_line(&from, &to);
        }
    }

    // Bottom line
    sim_2.add_line(
        &(sim_2.min_coords.0 as i32, sim_2.max_coords.1 as i32),
        &(sim_2.max_coords.0 as i32, sim_2.max_coords.1 as i32)
    );

    let sand_1 = sim_1.simulate_1((500, 0));
    println!("{} units of sand fell before it began falling into the abyss.", sand_1 - 1);

    let sand_2 = sim_2.simulate_2((500, 0));
    println!("{} units of sand fell before it blocked the source.", sand_2);
}
