use std::collections::HashMap;
use std::fs;


struct ValueData {
    is_even: bool,
    split_left: u128,
    split_right: u128,
}

impl ValueData {
    fn new( value: u128 ) -> Self {
        let num_digits = if value == 0 { 1 } else { value.ilog10() + 1 };

        let mut value_data = Self {
            is_even: num_digits % 2 == 0,
            split_left: 0,
            split_right: 0,
        };

        if value > 9 && value_data.is_even {
            let tens = 10_u128.pow( num_digits / 2 );
            value_data.split_left = value / tens;
            value_data.split_right = value % tens;
        }

        value_data
    }
}


fn update_count( map: &mut HashMap<u128, u64>, value: &u128, count: u64 ) {
    if map.contains_key( &value ) {
        *map.get_mut( &value ).unwrap() += count;
    }
    else {
        map.insert( *value, count );
    }
}


fn blink( stones: &HashMap<u128, u64>, lookup: &mut HashMap<u128, ValueData> ) -> HashMap<u128, u64> {
    let mut updated: HashMap<u128, u64> = HashMap::new();

    for ( value, count ) in stones {
        if *count == 0 {
            continue;
        }

        if *value == 0 {
            update_count( &mut updated, &1, *count );
            continue;
        }

        if !lookup.contains_key( value ) {
            lookup.insert( *value, ValueData::new( *value ) );
        }

        let cached: &ValueData = &lookup[&value];

        if cached.is_even {
            let left = cached.split_left;
            let right = cached.split_right;

            update_count( &mut updated, &left, *count );
            update_count( &mut updated, &right, *count );

            continue;
        }

        let new_val = value * 2024;
        update_count( &mut updated, &new_val, *count );
    }

    updated
}


fn count_stones( stones: &HashMap<u128, u64> ) -> u64 {
    let mut num = 0;

    for count in stones.values() {
        num += count;
    }

    num
}


fn main() {
    let contents = fs::read_to_string( "input.txt" )
        .expect( "Failed to read file." );

    let input: Vec<u128> = contents.trim().split_terminator( " " )
        .map( |x| x.parse::<u128>().unwrap() )
        .collect();

    let mut stones: HashMap<u128, u64> = HashMap::new();
    let mut lookup: HashMap<u128, ValueData> = HashMap::new();

    for stone in &input {
        if stones.contains_key( &stone ) {
            *stones.get_mut( &stone ).unwrap() += 1;
        }
        else {
            stones.insert( *stone, 1 );
            lookup.insert( *stone, ValueData::new( *stone ) );
        }
    }

    for i in 0..75 {
        stones = blink( &stones, &mut lookup );

        if i == 24 {
            println!( "(Part 1) There are {} stones after 25 blinks.", count_stones( &stones ) );
        }
    }

    println!( "(Part 2) There are {} stones after 75 blinks.", count_stones( &stones ) );
}
