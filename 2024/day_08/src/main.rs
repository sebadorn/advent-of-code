use std::collections::{HashMap, HashSet};
use std::fs;


type Location = (i32, i32);
type CityMap = Vec<Vec<char>>;
type AntennaMap = HashMap<char, Vec<Location>>;


fn build_antenna_list( map: &CityMap ) -> AntennaMap {
    let mut antenna = HashMap::new();

    for ( y, row ) in map.iter().enumerate() {
        for ( x, c ) in row.iter().enumerate() {
            if *c == '.' {
                continue;
            }

            let loc: Location = (
                x.try_into().unwrap(),
                y.try_into().unwrap()
            );
            let entry = antenna.get_mut( c );

            if entry.is_none() {
                antenna.insert( *c, vec![loc] );
            }
            else {
                let list = entry.unwrap();
                list.push( loc );
            }
        }
    }

    antenna
}


fn find_antinode_locations( antenna: &AntennaMap, map_limit: &Location ) -> (HashSet<Location>, HashSet<Location>) {
    let mut locations_part1 = HashSet::new();
    let mut locations_part2 = HashSet::new();

    for ( _key, list ) in antenna.iter() {
        for ( i, l1 ) in list.into_iter().enumerate() {
            let sub_list = &list[( i + 1 )..];

            for l2 in sub_list {
                let ( result_part1, result_part2 ) = check_for_antinodes( l1, l2, &map_limit );

                for loc in result_part1 {
                    locations_part1.insert( loc );
                }

                for loc in result_part2 {
                    locations_part2.insert( loc );
                }
            }
        }
    }

    ( locations_part1, locations_part2 )
}


fn check_for_antinodes( l1: &Location, l2: &Location, map_limit: &Location ) -> (Vec<Location>, Vec<Location>) {
    let dir = ( l1.0 - l2.0, l1.1 - l2.1 );
    let node1 = ( l1.0 + dir.0, l1.1 + dir.1 );
    let node2 = ( l2.0 - dir.0, l2.1 - dir.1 );

    let mut nodes_part1 = Vec::new();
    let mut nodes_part2 = vec![*l1, *l2];

    if is_in_map( &node1, &map_limit ) {
        nodes_part1.push( node1 );
        nodes_part2.push( node1 );
    }

    if is_in_map( &node2, &map_limit ) {
        nodes_part1.push( node2 );
        nodes_part2.push( node2 );
    }

    let mut next = node1.clone();

    loop {
        next = ( next.0 + dir.0, next.1 + dir.1 );

        if is_in_map( &next, &map_limit ) {
            nodes_part2.push( next );
        }
        else {
            break;
        }
    }

    next = node2.clone();

    loop {
        next = ( next.0 - dir.0, next.1 - dir.1 );

        if is_in_map( &next, &map_limit ) {
            nodes_part2.push( next );
        }
        else {
            break;
        }
    }

    ( nodes_part1, nodes_part2 )
}


fn is_in_map( node: &Location, map_limit: &Location ) -> bool {
    node.0 >= 0 && node.1 >= 0 && node.0 < map_limit.0 && node.1 < map_limit.1
}


fn main() {
    let contents = fs::read_to_string( "input.txt" )
        .expect( "Failed to read file." );
    let lines: Vec<&str> = contents.split_terminator( "\n" ).collect();

    let mut map: CityMap = Vec::new();

    for line in lines {
        let row: Vec<char> = line.chars().collect();
        map.push( row );
    }

    let map_limit = (
        map.len().try_into().unwrap(),
        map[0].len().try_into().unwrap()
    );
    let antenna = build_antenna_list( &map );
    let ( antinodes_part1, antinodes_part2 ) = find_antinode_locations( &antenna, &map_limit );

    println!( "(Part 1) Unique locations with antinodes: {}", antinodes_part1.len() );
    println!( "(Part 2) Unique locations with antinodes: {}", antinodes_part2.len() );
}
