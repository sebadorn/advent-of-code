use std::fs;


struct Card {
    winning: Vec<usize>,
    chosen: Vec<usize>,
}


impl Card {

    fn line_to_number(line: &str) -> Vec<usize> {
        let binding = line.trim().replace("  ", " ");
        let strs: Vec<&str> = binding.split(" ").collect();

        let numbers: Vec<usize> = strs.iter()
            .map(|&a| a.parse::<usize>().unwrap())
            .collect();

        return numbers;
    }

    fn from_line(line: &str) -> Self {
        let parts: Vec<&str> = line.split(":")
            .collect::<Vec<&str>>()[1]
            .split(" | ")
            .collect();

        let winning = Self::line_to_number(parts[0]);
        let chosen = Self::line_to_number(parts[1]);

        Self {
            winning,
            chosen,
        }
    }

    fn get_points(&mut self) -> usize {
        let mut points = 0;

        for (_index, number) in self.chosen.iter().enumerate() {
            if self.winning.contains(number) {
                if points == 0 {
                    points = 1;
                }
                else {
                    points *= 2;
                }
            }
        }

        points
    }

}


fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Failed to read file.");
    let lines: Vec<&str> = content.trim().split("\n").collect();

    let mut points = 0;

    for (_card, line) in lines.iter().enumerate() {
        let mut card = Card::from_line(line);
        points += card.get_points();
    }

    println!("(Part 1) The cards have a sum of {} points.", points);
}
