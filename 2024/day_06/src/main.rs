use std::collections::HashSet;
use std::fs;


#[derive(Clone, PartialEq)]
enum MapField {
    Free,      // .
    Blocked,   // #
    Guard,     // ^ (starting face)
    GuardPath, // X
}

type TheMap = Vec<Vec<MapField>>;


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Default for Direction {
    fn default() -> Self { Direction::Up }
}


#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}


#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Guard {
    pos: Position,
    face: Direction,
}

impl Guard {
    fn new() -> Self {
        Default::default()
    }
}


fn get_next_position( next_pos: &Position, face: &Direction ) -> Position {
    let mut pos = next_pos.clone();

    match face {
        Direction::Down => pos.y += 1,
        Direction::Left => {
            if pos.x == 0 {
                pos.x = usize::MAX;
            }
            else {
                pos.x -= 1;
            }
        },
        Direction::Right => pos.x += 1,
        Direction::Up => {
            if pos.y == 0 {
                pos.y = usize::MAX;
            }
            else {
                pos.y -= 1;
            }
        },
    }

    pos
}


fn rotate_direction( current_dir: &Direction ) -> Direction {
    match current_dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}


fn simulate_step( map: &mut TheMap, guard: &mut Guard, map_size: &Position ) -> bool {
    let next_pos = get_next_position( &guard.pos, &guard.face );

    if !is_guard_on_map( &next_pos, &map_size ) {
        return false;
    }

    let next_field = &map[next_pos.y][next_pos.x];

    match next_field {
        MapField::Blocked => {
            guard.face = rotate_direction( &guard.face );
        },
        _ => {
            guard.pos = next_pos;
            map[guard.pos.y][guard.pos.x] = MapField::GuardPath;
        },
    }

    true
}


fn is_guard_on_map( guard_pos: &Position, map_size: &Position ) -> bool {
    // Guard position will be set to usize::MAX if outside the map in the negative.
    guard_pos.x < map_size.x &&
    guard_pos.y < map_size.y
}


fn count_visited( map: &TheMap ) -> i32 {
    let mut count = 0;

    for y in 0..map.len() {
        let row = &map[0];

        for x in 0..row.len() {
            let field = &map[y][x];

            count += match field {
                MapField::Guard => 1,
                MapField::GuardPath => 1,
                _ => 0,
            }
        }
    }

    count
}


fn main() {
    let contents = fs::read_to_string( "input.txt" )
        .expect( "Failed to read file." );
    let lines: Vec<&str> = contents.split_terminator( "\n" ).collect();

    let mut map: TheMap = Vec::new();
    let mut guard = Guard::new();

    for ( y, line ) in lines.into_iter().enumerate() {
        let mut row: Vec<MapField> = Vec::new();

        for ( x, c ) in line.chars().enumerate() {
            let field = match c {
                '.' => MapField::Free,
                '^' => MapField::Guard,
                '#' => MapField::Blocked,
                'X' => MapField::GuardPath,
                _ => panic!( "Unexpected field: {}", c ),
            };

            if field == MapField::Guard {
                guard.pos.x = x;
                guard.pos.y = y;
            }

            row.push( field );
        }

        map.push( row );
    }

    let map_start = map.clone();
    let guard_start = guard.clone();

    let map_size = Position {
        x: map.len(),
        y: map[0].len(),
    };

    let mut route = HashSet::new();

    loop {
        if !simulate_step( &mut map, &mut guard, &map_size ) {
            break;
        }

        route.insert( guard.pos.clone() );
    }

    println!( "(Part 1) The guard has visited {} positions.", count_visited( &map ) );


    let mut options = 0;
    let num_pos = route.len();
    let mut path = HashSet::new();

    for ( i, pos ) in route.into_iter().enumerate() {
        print!( "\rChecking {number:0>4}/{total}.", number = i, total = num_pos );

        // Reset guard position
        guard = guard_start.clone();
        // Reset map
        map = map_start.clone();
        // Place the obstruction
        map[pos.y][pos.x] = MapField::Blocked;

        path.clear();
        path.insert( guard.clone() );

        loop {
            // Not a loop, guard left map
            if !simulate_step( &mut map, &mut guard, &map_size ) {
                break;
            }

            // Loop found, guard back at start
            if path.contains( &guard ) {
                options += 1;
                break;
            }

            path.insert( guard.clone() );
        }
    }

    println!( "\r(Part 2) There are {} options for obstructions to create loops.", options );
}
