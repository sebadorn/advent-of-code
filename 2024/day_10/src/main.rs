use std::fs;


struct TopoMap {
    data: Vec<Vec<i8>>,
    width: usize,
    height: usize,
}

struct Start {
    x: usize,
    y: usize,
    score: usize,
    rating: usize,
    goals: Vec<(usize, usize)>,
}

impl Start {
    fn new( x: usize, y: usize ) -> Self {
        Self {
            x, y,
            score: 0,
            rating: 0,
            goals: Vec::new(),
        }
    }
}


fn find_starting_points( map: &TopoMap ) -> Vec<Start> {
    let mut starts = Vec::new();

    for y in 0..map.height {
        let row = &map.data[y];

        for x in 0..row.len() {
            let height = row[x];

            if height == 0 {
                starts.push( Start::new( x, y ) );
            }
        }
    }

    starts
}


fn check_path( start: &mut Start, pos: (usize, usize), height: i8, map: &TopoMap ) {
    if height == 9 {
        start.rating += 1;

        if !start.goals.contains( &pos ) {
            start.goals.push( pos );
            start.score += 1;
        }

        return;
    }

    // check left
    if pos.0 > 0 {
        let left = map.data[pos.1][pos.0 - 1];

        if left == height + 1 {
            check_path( start, ( pos.0 - 1, pos.1 ), left, &map );
        }
    }

    // check right
    if pos.0 < map.width - 1 {
        let right = map.data[pos.1][pos.0 + 1];

        if right == height + 1 {
            check_path( start, ( pos.0 + 1, pos.1 ), right, &map );
        }
    }

    // check up
    if pos.1 > 0 {
        let up = map.data[pos.1 - 1][pos.0];

        if up == height + 1 {
            check_path( start, ( pos.0, pos.1 - 1 ), up, &map );
        }
    }

    // check down
    if pos.1 < map.height - 1 {
        let down = map.data[pos.1 + 1][pos.0];

        if down == height + 1 {
            check_path( start, ( pos.0, pos.1 + 1 ), down, &map );
        }
    }
}


fn main() {
    let contents = fs::read_to_string( "input.txt" )
        .expect( "Failed to read file." );
    let lines: Vec<&str> = contents.split_terminator( "\n" ).collect();

    let mut map_data: Vec<Vec<i8>> = Vec::new();

    for line in lines {
        let mut row = Vec::new();

        for c in line.chars() {
            let height = c.to_digit( 10 ).unwrap().try_into().unwrap();
            row.push( height );
        }

        map_data.push( row );
    }

    let map = TopoMap {
        data: map_data.clone(),
        width: map_data[0].len(),
        height: map_data.len(),
    };

    let starts = find_starting_points( &map );
    let mut score_sum = 0;
    let mut rating_sum = 0;

    for mut start in starts {
        let pos = ( start.x, start.y );
        check_path( &mut start, pos, 0, &map );
        score_sum += start.score;
        rating_sum += start.rating;
    }

    println!( "(Part 1) Score sum: {}", score_sum );
    println!( "(Part 2) Rating sum: {}", rating_sum );
}
