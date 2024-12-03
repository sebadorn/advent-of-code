use std::fs;
use regex::Regex;


fn get_all_pairs( source: &str ) -> Vec<(i64,i64)> {
    let re = Regex::new( r"mul\((?<x>[0-9]+),(?<y>[0-9]+)\)" ).unwrap();

    let pairs: Vec<(i64,i64)> = re.captures_iter( &source ).map( |c| {
        let x: i64 = c.name( "x" ).unwrap()
            .as_str().parse::<i64>()
            .expect( "Failed to parse match" );
        let y: i64 = c.name( "y" ).unwrap()
            .as_str().parse::<i64>()
            .expect( "Failed to parse match" );

        ( x, y )
    } ).collect();

    pairs
}


fn sum( pairs: &Vec<(i64,i64)> ) -> i64 {
    let mut sum = 0;

    for pair in pairs {
        sum += pair.0 * pair.1;
    }

    sum
}


fn main() {
    let contents = fs::read_to_string( "input.txt" )
        .expect( "Failed to read file." );

    let pairs_part1 = get_all_pairs( &contents );
    println!( "(Part 1) The sum of the mul(x,y) operations is: {}", sum( &pairs_part1 ) );


    let mut pairs_part2: Vec<(i64,i64)> = Vec::new();
    let mut haystack: &str = &contents;

    while haystack.len() > 7 {
        let end_match = haystack.find( "don't()" );
        let end = if end_match.is_some() { end_match.unwrap() } else { haystack.len() };

        let section = &haystack[..end];
        pairs_part2.append( &mut get_all_pairs( &section ) );

        haystack = &haystack[end..];
        let search_next_start = haystack.find( "do()" );

        if !search_next_start.is_some() {
            break;
        }

        let start = search_next_start.unwrap();
        haystack = &haystack[start..];
    }

    println!( "(Part 2) The sum of enabled mul(x,y): {}", sum( &pairs_part2 ) );
}
