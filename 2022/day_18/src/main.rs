use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;


#[derive(Debug)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}


impl Cube {

    fn from_str(line: &str) -> Self {
        let parts: Vec<&str> = line.split(',').collect();

        Self {
            x: parts[0].parse::<i32>().unwrap(),
            y: parts[1].parse::<i32>().unwrap(),
            z: parts[2].parse::<i32>().unwrap(),
        }
    }

    fn count_free_sides(&self, grid: &HashMap<String, Cube>) -> i32 {
        let side_keys = [
            (self.x, self.y + 1, self.z),
            (self.x, self.y - 1, self.z),
            (self.x + 1, self.y, self.z),
            (self.x - 1, self.y, self.z),
            (self.x, self.y, self.z + 1),
            (self.x, self.y, self.z - 1),
        ];

        let mut free = 0;

        for pos in side_keys {
            let key = format!("{},{},{}", pos.0, pos.1, pos.2);

            if !grid.contains_key(key.as_str()) {
                free += 1;
            }
        }

        free
    }

    fn count_free_outside(&self, air: &HashSet<String>) -> i32 {
        let sides = [
            (0, 1, 0),
            (0, -1, 0),
            (1, 0, 0),
            (-1, 0, 0),
            (0, 0, 1),
            (0, 0, -1),
        ];

        let mut free = 0;

        for dir in sides {
            let pos = (self.x + dir.0, self.y + dir.1, self.z + dir.2);
            let key = format!("{},{},{}", pos.0, pos.1, pos.2);

            if air.contains(&key) {
                free += 1;
            }
        }

        free
    }

}


fn fill_air_grid(
    air: &mut HashSet<String>, grid: &HashMap<String, Cube>,
    min: i32, max: i32
) {
    if min >= max {
        return;
    }

    for x in min..=max {
        for y in min..=max {
            for z in min..=max {
                let key = format!("{},{},{}", x, y, z);

                if grid.contains_key(&key) {
                    continue;
                }

                let sides = [
                    (0, 1, 0),
                    (0, -1, 0),
                    (1, 0, 0),
                    (-1, 0, 0),
                    (0, 0, 1),
                    (0, 0, -1),
                ];

                // Check for connections to other outside air
                for dir in sides {
                    let pos_key = format!("{},{},{}", x + dir.0, y + dir.1, z + dir.2);

                    if air.contains(&pos_key) {
                        air.insert(key);
                        break;
                    }
                }
            }
        }
    }

    fill_air_grid(air, &grid, min + 1, max - 1);
}


fn main() {
    let now = Instant::now();
    let content = std::fs::read_to_string("input.txt").unwrap();

    let mut grid: HashMap<String, Cube> = HashMap::new();
    let mut min = (i32::MAX, i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN, i32::MIN);

    for line in content.lines() {
        let cube = Cube::from_str(line);

        min.0 = cmp::min(min.0, cube.x);
        min.1 = cmp::min(min.1, cube.y);
        min.2 = cmp::min(min.2, cube.z);

        max.0 = cmp::max(max.0, cube.x);
        max.1 = cmp::max(max.1, cube.y);
        max.2 = cmp::max(max.2, cube.z);

        grid.insert(line.to_string(), cube);
    }

    println!("min: {:?}, max: {:?}", min, max);

    // Use padded cube
    let air_min = cmp::min(min.0, cmp::min(min.1, min.2)) - 1;
    let air_max = cmp::max(max.0, cmp::max(max.1, max.2)) + 1;
    let mut air: HashSet<String> = HashSet::new();

    // Fill outside area first
    for x in air_min..=air_max {
        for y in air_min..=air_max {
            for z in air_min..=air_max {
                let key = format!("{},{},{}", x, y, z);

                if grid.contains_key(&key) {
                    continue;
                }

                if
                    (x <= air_min || x >= air_max) &&
                    (y <= air_min || y >= air_max) &&
                    (z <= air_min || z >= air_max)
                {
                    air.insert(key);
                }
            }
        }
    }

    fill_air_grid(&mut air, &grid, air_min, air_max);

    let mut free_sides = 0;
    let mut free_outside = 0;

    for cube in grid.values() {
        free_sides += cube.count_free_sides(&grid);
        free_outside += cube.count_free_outside(&air);
    }

    println!("The surface area for the {} cubes is {}.", grid.len(), free_sides);
    println!("The surface area excluding trapped droplets is {}.", free_outside);

    println!("Execution took {} ms.", now.elapsed().as_millis());
}
