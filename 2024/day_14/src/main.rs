use std::collections::HashSet;
use std::fs;


struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Robot {
    fn from_line( line: &str ) -> Self {
        let parts: Vec<&str> = line.split_terminator( " " ).collect();
        let pos_str: Vec<&str> = parts[0][2..].split_terminator( "," ).collect();
        let vel_str: Vec<&str> = parts[1][2..].split_terminator( "," ).collect();

        Robot {
            pos: ( pos_str[0].parse().unwrap(), pos_str[1].parse().unwrap() ),
            vel: ( vel_str[0].parse().unwrap(), vel_str[1].parse().unwrap() ),
        }
    }

    fn step( &mut self, map_width: i32, map_height: i32 ) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;

        while self.pos.0 < 0 {
            self.pos.0 += map_width;
        }

        while self.pos.0 >= map_width {
            self.pos.0 -= map_width;
        }

        while self.pos.1 < 0 {
            self.pos.1 += map_height;
        }

        while self.pos.1 >= map_height {
            self.pos.1 -= map_height;
        }
    }
}


fn count_robots( robots: &Vec<Robot>, start: (i32, i32), size: (i32, i32 ) ) -> i32 {
    let mut sum = 0;

    for robot in robots {
        if robot.pos.0 < start.0 || robot.pos.0 >= start.0 + size.0 {
            continue;
        }

        if robot.pos.1 < start.1 || robot.pos.1 >= start.1 + size.1 {
            continue;
        }

        sum += 1;
    }

    sum
}


fn check_for_tree( robots: &Vec<Robot>, width: &i32, height: &i32 ) -> bool {
    let mut map = HashSet::new();

    for robot in robots {
        map.insert( robot.pos );
    }

    let center = ( width / 2, height / 2 );

    if !map.contains( &center ) {
        return false;
    }

    // Check if there is a cluster of robots (5x5 square) in the center.
    // Requires the tree image to be centered and filled, which it is.
    for y in ( center.1 - 2 )..( center.1 + 2 ) {
        for x in ( center.0 - 2 )..( center.0 + 2 ) {
            let pos = ( x, y );

            if !map.contains( &pos ) {
                return false;
            }
        }
    }

    true
}


fn main() {
    let contents = fs::read_to_string( "input.txt" )
        .expect( "Failed to read file." );
    let lines: Vec<&str> = contents.split_terminator( "\n" ).collect();

    let map_width = 101;
    let map_height = 103;

    let mut robots = Vec::new();

    for line in lines {
        robots.push( Robot::from_line( &line ) );
    }

    let mut possible_tree_at = -1;

    for i in 1..101 {
        for robot in &mut robots {
            robot.step( map_width, map_height );
        }

        if possible_tree_at < 0 && check_for_tree( &robots, &map_width, &map_height ) {
            possible_tree_at = i;
        }
    }

    let map_width_half = map_width / 2;
    let map_height_half = map_height / 2;
    let quadrant_size = ( map_width_half, map_height_half );

    let quadrant_lt = count_robots( &robots, ( 0, 0 ), quadrant_size );
    let quadrant_rt = count_robots( &robots, ( map_width_half + 1, 0 ), quadrant_size );
    let quadrant_lb = count_robots( &robots, ( 0, map_height_half + 1 ), quadrant_size );
    let quadrant_rb = count_robots( &robots, ( map_width_half + 1, map_height_half + 1 ), quadrant_size );

    println!(
        "(Part 1) Safety factor: {} * {} * {} * {} = {}",
        quadrant_lt, quadrant_rt, quadrant_lb, quadrant_rb,
        quadrant_lt * quadrant_rt * quadrant_lb * quadrant_rb
    );

    let mut i = 101;

    while possible_tree_at < 0 {
        for robot in &mut robots {
            robot.step( map_width, map_height );
        }

        if check_for_tree( &robots, &map_width, &map_height ) {
            possible_tree_at = i;
        }

        i += 1;
    }

    println!( "(Part 2) Possible tree shown after {} seconds.", possible_tree_at );
}
