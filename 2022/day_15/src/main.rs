use regex::Regex;
use std::fs;
use std::time::Instant;


#[derive(Debug)]
struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
    m_dist: i32,
}


impl Sensor {

    fn manhatten_dist(a: &(i32, i32), b: &(i32, i32)) -> i32 {
        let x = (a.0 - b.0).abs();
        let y = (a.1 - b.1).abs();

        x + y
    }

    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(": ").collect();
        let re_left = Regex::new(r"Sensor at x=([-0-9]+), y=([-0-9]+)").unwrap();
        let re_right = Regex::new(r"closest beacon is at x=([-0-9]+), y=([-0-9]+)").unwrap();

        let cap_left = re_left.captures(parts[0]).unwrap();
        let x = cap_left.get(1).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        let y = cap_left.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());

        let cap_right = re_right.captures(parts[1]).unwrap();
        let b_x = cap_right.get(1).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        let b_y = cap_right.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());

        Self {
            pos: (x, y),
            beacon: (b_x, b_y),
            m_dist: Self::manhatten_dist(&(x, y), &(b_x, b_y)),
        }
    }

    fn get_outside_border_points(&self) -> Vec<(i32, i32)> {
        let mut list: Vec<(i32, i32)> = vec![];

        for i in 0..=self.m_dist {
            list.push((self.pos.0 + i, self.pos.1 - self.m_dist - 1 + i));
            list.push((self.pos.0 + self.m_dist + 1 - i, self.pos.1 - i));
            list.push((self.pos.0 - i, self.pos.1 + self.m_dist + 1 - i));
            list.push((self.pos.0 - self.m_dist - 1 + i, self.pos.1 - i));
        }

        list
    }

    fn is_in_area(&self, pos: &(i32, i32)) -> bool {
        self.m_dist >= Self::manhatten_dist(&self.pos, pos)
    }

}


fn main() {
    let now = Instant::now();

    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt.");

    let mut seen: Vec<i32> = vec![];
    let mut sensors: Vec<Sensor> = vec![];
    let line_check = 2_000_000;

    for line in content.lines() {
        let sensor = Sensor::new(line);

        // No need to check any further if straight
        // up/down to the line is not in the area.
        if !sensor.is_in_area(&(sensor.pos.0, line_check)) {
            sensors.push(sensor);
            continue;
        }

        let range_reduction = (sensor.pos.1 - line_check).abs();
        let from_left = sensor.pos.0 - sensor.m_dist + range_reduction;
        let to_right = sensor.pos.0 + sensor.m_dist - range_reduction;

        for x in from_left..=to_right {
            let pos = (x, line_check);

            if sensor.beacon == pos {
                continue;
            }

            if sensor.is_in_area(&pos) {
                seen.push(x);
            }
        }

        sensors.push(sensor);
    }

    // Testing showed that it is a lot faster to just add duplicates,
    // afterwards sort the list, and then remove the duplicates, than to...
    // ... check in the loop if the list already contains the value.
    // ... use a HashSet.
    seen.sort();
    seen.dedup();

    println!(
        "{} positions cannot contain an undetected beacon on line {}.",
        seen.len(), line_check
    );


    let mut beacon: Option<(i32, i32)> = None;

    for sensor in &sensors {
        let border = sensor.get_outside_border_points();

        for pos in border.into_iter() {
            let mut is_free = true;

            for sensor_2 in &sensors {
                if sensor.pos == sensor_2.pos {
                    continue;
                }

                if sensor_2.is_in_area(&pos) {
                    is_free = false;
                    break;
                }
            }

            if is_free {
                beacon = Some(pos);
            }
        }

        if beacon != None {
            break;
        }
    }

    let beacon_pos = beacon.unwrap();
    let tf = beacon_pos.0 as i64 * 4_000_000 + beacon_pos.1 as i64;
    println!("The beacon tuning frequency is {}.", tf);


    let elapsed = now.elapsed();
    println!("Execution took {} ms.", elapsed.as_millis());
}
