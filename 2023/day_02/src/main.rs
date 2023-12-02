use std::fs;


struct GameSubset {
    r: i16,
    g: i16,
    b: i16,
}


struct Game {
    id: i16,
    subsets: Vec<GameSubset>,
}


impl Game {

    fn from_line(line: &str) -> Self {
        let parts: Vec<&str> = line.split(": ").collect();
        let id: i16 = parts[0][5..].parse::<i16>().unwrap();

        let draws: Vec<&str> = parts[1].split("; ").collect();
        let mut subsets: Vec<GameSubset> = Vec::new();

        for (_index, draw) in draws.iter().enumerate() {
            let balls: Vec<&str> = draw.split(", ").collect();
            let mut num_r = 0;
            let mut num_g = 0;
            let mut num_b = 0;

            for (_index, result) in balls.iter().enumerate() {
                let parts: Vec<&str> = result.split(" ").collect();
                let num_balls = parts[0].parse::<i16>().unwrap();

                match parts[1] {
                    "red" => num_r = num_balls,
                    "green" => num_g = num_balls,
                    "blue" => num_b = num_balls,
                    _ => (),
                }
            }

            subsets.push(GameSubset { r: num_r, g: num_g, b: num_b });
        }

        Self {
            id,
            subsets,
        }
    }

    fn is_possible(&mut self, limit: &GameSubset) -> bool {
        for (_index, subset) in self.subsets.iter().enumerate() {
            if !Self::is_subset_possible(&subset, &limit) {
                return false;
            }
        }

        return true;
    }

    fn is_subset_possible(subset: &GameSubset, limit: &GameSubset) -> bool {
        return subset.r <= limit.r &&
            subset.g <= limit.g &&
            subset.b <= limit.b;
    }

    fn get_power(&mut self) -> u32 {
        let minimum = self.get_required_minimum();

        return minimum.r as u32 * minimum.g as u32 * minimum.b as u32;
    }

    fn get_required_minimum(&mut self) -> GameSubset {
        let mut minimum = GameSubset { r: 0, g: 0, b: 0 };

        for (_index, subset) in self.subsets.iter().enumerate() {
            minimum.r = if subset.r > minimum.r { subset.r } else { minimum.r };
            minimum.g = if subset.g > minimum.g { subset.g } else { minimum.g };
            minimum.b = if subset.b > minimum.b { subset.b } else { minimum.b };
        }

        return minimum;
    }

}


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read file.");
    let lines: Vec<&str> = contents.split("\n").collect();

    let limit = GameSubset { r: 12, g: 13, b: 14 };
    let mut id_sum = 0;
    let mut power_sum = 0;

    for (_index, line) in lines.iter().enumerate() {
        if !line.starts_with("Game ") {
            continue
        }

        let mut game = Game::from_line(line);

        if game.is_possible(&limit) {
            id_sum += game.id;
        }

        power_sum += game.get_power();
    }

    println!("(Part 1) The ID sum of possible games is: {}", id_sum);
    println!("(Part 2) The power sum of all games is: {}", power_sum);
}
