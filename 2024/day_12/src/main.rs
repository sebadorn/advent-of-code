use std::collections::{HashMap, HashSet};
use std::fs;


#[derive(Clone, Copy)]
struct Plot {
    region_id: u32, // 0: not assigned yet
    pos: (usize, usize),
    plant: char,
    fences: u32,
}

struct Farm {
    map: Vec<Vec<Plot>>,
    width: usize,
    height: usize,
}

impl Farm {
    fn new( map: Vec<Vec<Plot>> ) -> Self {
        Self {
            map: map.clone(),
            width: map[0].len(),
            height: map.len(),
        }
    }
}

struct Region<'a> {
    plots: Vec<&'a Plot>,
}

impl Region<'_> {
    fn count_perimeters( &self ) -> u32 {
        let mut sum = 0;

        for i in 0..self.plots.len() {
            sum += self.plots[i].fences;
        }

        sum
    }

    fn calculate_sides( &self ) -> u32 {
        let mut top_left = (0, 0);
        let mut bottom_right = (0, 0);

        let mut set: HashSet<(usize, usize)> = HashSet::new();

        for i in 0..self.plots.len() {
            let plot = self.plots[i];

            set.insert( plot.pos );

            if i == 0 {
                top_left.0 = plot.pos.0;
                top_left.1 = plot.pos.1;

                bottom_right.0 = plot.pos.0;
                bottom_right.1 = plot.pos.1;

                continue;
            }

            top_left.0 = usize::min( top_left.0, plot.pos.0 );
            top_left.1 = usize::min( top_left.1, plot.pos.1 );

            bottom_right.0 = usize::max( bottom_right.0, plot.pos.0 );
            bottom_right.1 = usize::max( bottom_right.1, plot.pos.1 );
        }

        // Include that position when used as for-in-range loops.
        bottom_right.0 += 1;
        bottom_right.1 += 1;

        let mut corners = 0;

        for y in top_left.1..bottom_right.1 {
            for x in top_left.0..bottom_right.0 {
                let pos = (x, y);

                let left = if x > 0 { set.contains( &( x - 1, y ) ) } else { false };
                let right = set.contains( &( x + 1, y ) );
                let top = if y > 0 { set.contains( &( x, y - 1 ) ) } else { false };
                let bottom = set.contains( &( x, y + 1 ) );

                // Plot
                if set.contains( &pos ) {
                    if !left {
                        let tl = if x > 0 && y > 0 { set.contains( &( x - 1, y - 1 ) ) } else { false };
                        let bl = if x > 0 { set.contains( &( x - 1, y + 1 ) ) } else { false };

                        if !top && !tl { corners += 1; }
                        if !bottom && !bl { corners += 1; };
                    }

                    if !right {
                        let tr = if y > 0 { set.contains( &( x + 1, y - 1 ) ) } else { false };
                        let br = set.contains( &( x + 1, y + 1 ) );

                        if !top && !tr { corners += 1; }
                        if !bottom && !br { corners += 1; }
                    }
                }
                // Gap
                else {
                    if left && top { corners += 1; }
                    if left && bottom { corners += 1; };
                    if right && top { corners += 1; }
                    if right && bottom { corners += 1; }
                }
            }
        }

        corners
    }

    fn calculate_price( &self ) -> u32 {
        let area: u32 = self.plots.len().try_into().unwrap();
        let perimeter = self.count_perimeters();

        area * perimeter
    }

    fn calculate_price_discounted( &self ) -> u32 {
        let area: u32 = self.plots.len().try_into().unwrap();
        let sides = self.calculate_sides();

        area * sides
    }
}


/// Returns true if this neighbour belongs to same region, false otherwise.
fn check_neighbours( farm: &mut Farm, pos: (usize, usize), plant: char, region_id: &u32 ) -> bool {
    {
        let plot = &mut farm.map[pos.1][pos.0];

        if plot.plant != plant {
            return false;
        }

        // Same plant type, but region has already been set. Nothing else to do.
        if plot.region_id > 0 {
            return true;
        }

        plot.region_id = *region_id;
    }

    // check right
    if pos.0 < farm.width - 1 {
        let right = ( pos.0 + 1, pos.1 );

        if !check_neighbours( farm, right, plant, region_id ) {
            farm.map[pos.1][pos.0].fences += 1;
        }
    }
    else {
        farm.map[pos.1][pos.0].fences += 1;
    }

    // check left
    if pos.0 > 0 {
        let left = ( pos.0 - 1, pos.1 );

        if !check_neighbours( farm, left, plant, region_id ) {
            farm.map[pos.1][pos.0].fences += 1;
        }
    }
    else {
        farm.map[pos.1][pos.0].fences += 1;
    }

    // check up
    if pos.1 > 0 {
        let up = ( pos.0, pos.1 - 1 );

        if !check_neighbours( farm, up, plant, region_id ) {
            farm.map[pos.1][pos.0].fences += 1;
        }
    }
    else {
        farm.map[pos.1][pos.0].fences += 1;
    }

    // check down
    if pos.1 < farm.height - 1 {
        let down = ( pos.0, pos.1 + 1 );

        if !check_neighbours( farm, down, plant, region_id ) {
            farm.map[pos.1][pos.0].fences += 1;
        }
    }
    else {
        farm.map[pos.1][pos.0].fences += 1;
    }

    true
}


fn assign_regions( farm: &mut Farm ) -> HashMap<u32, Region> {
    let mut region_id_counter = 0;

    for y in 0..farm.height {
        for x in 0..farm.width {
            let plot = &farm.map[y][x];

            if plot.region_id == 0 {
                region_id_counter += 1;
            }

            check_neighbours( farm, ( x, y ), plot.plant, &region_id_counter );
        }
    }

    let mut regions = HashMap::new();

    for y in 0..farm.height {
        for x in 0..farm.width {
            let plot = &farm.map[y][x];

            if !regions.contains_key( &plot.region_id ) {
                regions.insert(
                    plot.region_id,
                    Region { plots: vec![plot] }
                );
            }
            else {
                regions.get_mut( &plot.region_id ).unwrap().plots.push( plot );
            }
        }
    }

    regions
}


fn main() {
    let contents = fs::read_to_string( "input.txt" )
        .expect( "Failed to read file." );
    let lines: Vec<&str> = contents.split_terminator( "\n" ).collect();

    let mut farm_data = Vec::new();

    for line in lines {
        let mut row = Vec::new();

        for plant in line.chars() {
            let pos = ( row.len(), farm_data.len() );
            row.push( Plot { region_id: 0, pos: pos, plant, fences: 0 } );
        }

        farm_data.push( row );
    }

    let mut farm = Farm::new( farm_data );
    let regions = assign_regions( &mut farm );

    let mut total = 0;
    let mut with_discount = 0;

    for region in regions.values() {
        total += region.calculate_price();
        with_discount += region.calculate_price_discounted();
    }

    println!( "(Part 1) The total price is: {}", total );
    println!( "(Part 2) Discounted the price is: {}", with_discount );
}
