#![allow(unused_mut)]
#![allow(unused_variables)]

use std::fs;


struct Section {
    start: i32,
    end: i32,
}


impl Section {

    fn new(section: &str) -> Self {
        let parts: Vec<&str> = section.split('-').collect();
        let start: i32 = parts[0].parse().unwrap();
        let end: i32 = parts[1].parse().unwrap();

        Self {
            start: start,
            end: end,
        }
    }

    fn contains(&self, other: &Section) -> bool {
        if self.start <= other.start && self.end >= other.end {
            return true;
        }

        false
    }

    fn overlaps(&self, other: &Section) -> bool {
        if
            (self.start <= other.start && self.end >= other.start) ||
            (self.start <= other.end && self.end >= other.end)
        {
            return true;
        }

        false
    }

}


fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt.");

    let mut sum_contained = 0;
    let mut sum_any_overlap = 0;

    for line in content.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        let section_1 = Section::new(parts[0]);
        let section_2 = Section::new(parts[1]);

        if
            section_1.contains(&section_2) ||
            section_2.contains(&section_1)
        {
            sum_contained += 1;
            sum_any_overlap += 1;
        }
        else if
            section_1.overlaps(&section_2) ||
            section_2.overlaps(&section_1)
        {
            sum_any_overlap += 1;
        }
    }

    println!("In {} pairs is one contained in the other.", sum_contained);
    println!("In {} pairs have overlapping ranges.", sum_any_overlap);
}
