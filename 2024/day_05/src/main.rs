use std::collections::HashMap;
use std::fs;


fn check_update( update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>> ) -> i32 {
    // No need to check first page as it will always come
    // before all other pages no matter the rule.
    for i in 1..update.len() {
        let page = update[i];
        let rule = rules.get( &page );

        if rule.is_none() {
            continue;
        }

        if !check_rule( &update, rule.unwrap(), i ) {
            return 0;
        }
    }

    update[update.len() / 2]
}


fn check_rule( update: &Vec<i32>, rule: &Vec<i32>, start: usize ) -> bool {
    for i in 0..start {
        let value = update[i];

        if rule.contains( &value ) {
            return false;
        }
    }

    true
}


fn fix_update( update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>> ) -> i32 {
    let mut copy = update.clone();
    let num_pages = update.len();
    let mut index_no_rules = num_pages;

    for i in ( 0..num_pages ).rev() {
        let page = copy[i];
        let rule = rules.get( &page );

        // If there are no rules for this page, put
        // it at the end as this cannot be wrong.
        if rule.is_none() {
            if i < num_pages - 1 {
                copy.remove( i );
                copy.push( page );
            }

            index_no_rules -= 1;
        }
    }

    let mut fixed_something: bool;

    loop {
        fixed_something = false;

        for i in ( 1..index_no_rules ).rev() {
            let page = copy[i];
            let rule = rules.get( &page ).unwrap();

            if check_rule( &copy, rule, i ) {
                continue;
            }

            let mut new_copy: Vec<i32> = vec![page];
            copy.remove( i );
            new_copy.extend_from_slice( &copy );
            copy = new_copy;

            fixed_something = true;
            break;
        }

        if !fixed_something {
            break;
        }
    }

    copy[num_pages / 2]
}


fn main() {
    let binding = fs::read_to_string( "input.txt" )
        .expect( "Failed to read file." );
    let contents: Vec<&str> = binding.split_terminator( "\n\n" ).collect();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in contents[0].split_terminator( "\n" ) {
        let parts: Vec<&str> = line.split( "|" ).collect();
        let a = parts[0].parse::<i32>().unwrap();
        let b = parts[1].parse::<i32>().unwrap();

        if !rules.contains_key( &a ) {
            rules.insert( a, vec![b] );
        }
        else {
            let values = rules.get_mut( &a ).unwrap();
            values.push( b );
        }
    }

    let updates: Vec<Vec<i32>> = contents[1].split_terminator( "\n" )
        .map( |l| {
            l.split( "," )
                .map( |page| page.parse::<i32>().unwrap() )
                .collect()
        } )
        .collect();

    let mut updates_okay = 0;
    let mut updates_value_after_fix = 0;
    let mut fixed = 0;

    for update in updates {
        let check_value = check_update( &update, &rules );
        updates_okay += check_value;

        if check_value == 0 {
            updates_value_after_fix += fix_update( &update, &rules );
            fixed += 1;
        }
    }

    println!( "(Part 1) Updates value for those in right order: {}", updates_okay );
    println!( "(Part 2) Updates value for those fixed ({}): {}", fixed, updates_value_after_fix );
}
