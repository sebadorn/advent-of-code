use std::fs;


#[derive(Clone, PartialEq)]
enum Item {
    Space,
    Box,
    BoxLeft,
    BoxRight,
    Wall,
}


type Map = Vec<Vec<Item>>;


fn make_move( robot: &mut (i32, i32), m: &(i32, i32), map: &mut Map ) {
    let w = map[0].len() as i32;
    let h = map.len() as i32;

    let next = ( robot.0 + m.0, robot.1 + m.1 );

    if !is_in_map( &next, &w, &h ) {
        return;
    }

    let field = &map[next.1 as usize][next.0 as usize];

    if *field == Item::Wall {
        return;
    }

    if *field == Item::Space {
        *robot = next;
        return;
    }

    // Box at target position.
    let mut pos = next.clone();

    while is_in_map( &pos, &w, &h ) {
        pos.0 += m.0;
        pos.1 += m.1;

        let field = &map[pos.1 as usize][pos.0 as usize];

        // Cannot move in that direction.
        if *field == Item::Wall {
            break;
        }

        // Collect another box to maybe move.
        if *field == Item::Box {
            continue;
        }

        // Move all boxes in that direction.
        if *field == Item::Space {
            *robot = next;

            map[next.1 as usize][next.0 as usize] = Item::Space;
            map[pos.1 as usize][pos.0 as usize] = Item::Box;

            break;
        }
    }
}


fn is_in_map( pos: &(i32, i32), width: &i32, height: &i32 ) -> bool {
    pos.0 >= 0 && pos.0 < *width && pos.1 >= 0 && pos.1 < *height
}


fn extend_map( map: &Map ) -> Map {
    let w = map[0].len();
    let h = map.len();

    let mut new: Map = Vec::new();

    for y in 0..h {
        let mut row = Vec::new();

        for x in 0..w {
            let field = &map[y as usize][x as usize];

            if *field == Item::Wall || *field == Item::Space {
                row.push( field.clone() );
                row.push( field.clone() );
            }
            else if *field == Item::Box {
                row.push( Item::BoxLeft );
                row.push( Item::BoxRight );
            }
        }

        new.push( row );
    }

    new
}


fn calc_gps_score( map: &Map ) -> usize {
    let w = map[0].len();
    let h = map.len();

    let mut score = 0;

    for y in 0..h {
        for x in 0..w {
            let field = &map[y][x];

            if *field == Item::Box || *field == Item::BoxLeft {
                score += 100 * y + x;
            }
        }
    }

    score
}


fn main() {
    let contents = fs::read_to_string( "input.txt" )
        .expect( "Failed to read file." );
    let parts: Vec<&str> = contents.split_terminator( "\n\n" ).collect();
    let map_str: Vec<&str> = parts[0].split_terminator( "\n" ).collect();

    let mut map: Map = Vec::new();
    let mut robot = ( 0, 0 );
    let mut moves = Vec::new();

    for line in map_str {
        let mut row = Vec::new();

        for c in line.chars() {
            let item = match c {
                '#' => Item::Wall,
                'O' => Item::Box,
                _ => Item::Space,
            };

            if c == '@' {
                robot.0 = row.len() as i32;
                robot.1 = map.len() as i32;
            }

            row.push( item );
        }

        map.push( row );
    }

    let mut robot_big = robot.clone();
    let mut map_big = extend_map( &map );

    for c in parts[1].chars() {
        let m = match c {
            '>' => ( 1, 0 ),
            '^' => ( 0, -1 ),
            '<' => ( -1, 0 ),
            'v' => ( 0, 1 ),
            _ => ( 0, 0 ),
        };

        if m.0 != 0 || m.1 != 0 {
            moves.push( m );
        }
    }

    for m in &moves {
        make_move( &mut robot, &m, &mut map );
    }

    println!( "(Part 1) GPS score: {}", calc_gps_score( &map ) );

    // for m in &moves {
    //     make_move_big( &mut robot_big, &m, &mut map_big );
    // }

    // println!( "(Part 2) GPS score: {}", calc_gps_score( &map_big ) );
}
