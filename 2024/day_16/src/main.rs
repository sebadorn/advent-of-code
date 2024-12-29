use std::collections::{HashMap, HashSet};
use std::fs;


type Pos = (i32, i32);


#[allow(dead_code)]
fn print_map( map: &HashSet<Pos>, start: &Pos, end: &Pos ) {
    let height = start.1 + 2;
    let width = end.0 + 2;

    for y in 0..height {
        for x in 0..width {
            let pos = ( x, y );
            let mut c = if map.contains( &pos ) { '.' } else { '#' };

            if pos == *start {
                c = 'S';
            }
            else if pos == *end {
                c = 'E';
            }

            print!( "{}", c );
        }

        print!( "{}", '\n' );
    }
}


fn remove_dead_ends( map: &HashSet<Pos>, start: &Pos, end: &Pos ) -> HashSet<Pos> {
    let old_size = map.len();
    let mut clone = map.clone();

    loop {
        let mut new = HashSet::new();
        new.insert( *start );
        new.insert( *end );

        for pos in &clone {
            if pos == start || pos == end {
                continue;
            }

            let neighbours = vec![
                ( pos.0, pos.1 - 1 ),
                ( pos.0 + 1, pos.1 ),
                ( pos.0, pos.1 + 1 ),
                ( pos.0 - 1, pos.1 ),
            ];

            let mut sum = 0;

            for n in neighbours {
                if clone.contains( &n ) {
                    sum += 1;
                }
            }

            if sum > 1 {
                new.insert( *pos );
            }
        }

        let diff = clone.len() - new.len();

        if diff == 0 {
            println!( "Removed {} dead end spaces.", old_size - new.len() );

            return new;
        }

        clone = new;
    }
}


fn move_cost( new_dir: &Pos, dir: &Pos ) -> i32 {
    if new_dir.0 == dir.0 && new_dir.1 == dir.1 {
        return 1;
    }

    if ( dir.0 != 0 && new_dir.0 == -dir.0 ) || ( dir.1 != 0 && new_dir.1 == -dir.1 ) {
        return 2001;
    }

    1001
}


fn set_path_values(
    score: i32,
    last: &Pos,
    dir: &Pos,
    end: &Pos,
    mut map: &mut HashMap<Pos, i32>,
) {
    let dirs = vec![( 0, -1 ), ( 1, 0 ), ( -1, 0 ), ( 0, 1 )];

    for new_dir in dirs {
        let next = ( last.0 + new_dir.0, last.1 + new_dir.1 );

        if !map.contains_key( &next ) {
            continue;
        }

        let mut new_score = score;
        new_score += move_cost( &new_dir, &dir );

        if map[&next] < 0 {
            map.insert( next, new_score );
        }
        else if map[&next] > new_score {
            map.insert( next, new_score );
        }
        else {
            continue;
        }

        set_path_values( new_score, &next, &new_dir, &end, &mut map );
    }
}


fn main() {
    let contents = fs::read_to_string( "input.txt" )
        .expect( "Failed to read file." );
    let lines: Vec<&str> = contents.split_terminator( "\n" ).collect();

    let mut map = HashSet::new();
    let mut costs = HashMap::new();
    let mut start = ( 0, 0 );
    let mut end = ( 0, 0 );
    let mut y = 0;

    for line in lines {
        let mut x = 0;

        for c in line.chars() {
            let pos = ( x, y );

            match c {
                'S' => {
                    start.0 = x;
                    start.1 = y;
                    map.insert( pos );
                },
                'E' => {
                    end.0 = x;
                    end.1 = y;
                    map.insert( pos );
                },
                '.' => {
                    map.insert( pos );
                },
                _ => (),
            }

            x += 1;
        }

        y += 1;
    }

    map = remove_dead_ends( &map, &start, &end );

    for pos in &map {
        costs.insert( *pos, -1 );
    }

    // print_map( &map, &start, &end );

    set_path_values( 0, &start, &( 1, 0 ), &end, &mut costs );
    println!( "(Part 1) The path with the lowest score costs: {}", costs[&end] );
}
