use std::fs;


fn count_occurences( a: &i32, col2: &Vec<i32>, start: &mut usize ) -> ( i32, usize ) {
	let mut occurences: i32 = 0;

	let end: usize = col2.len();
	let mut new_start: usize = *start;

	for i in *start..end {
		let b = col2[i];

		if b == *a {
			occurences += 1;
		}
		else if b > *a {
			new_start = i;
			break;
		}
	}

	( occurences, new_start )
}


fn main() {
	let contents = fs::read_to_string( "input.txt" )
		.expect( "Failed to read file." );
	let lines: Vec<&str> = contents.split( "\n" ).collect();

	let mut col1: Vec<i32> = Vec::new();
	let mut col2: Vec<i32> = Vec::new();

	for ( _index, line ) in lines.iter().enumerate() {
		if line.len() == 0 {
			continue;
		}

		let parts: Vec<&str> = line.split( "   " ).collect();
		col1.push( parts[0].parse::<i32>().unwrap() );
		col2.push( parts[1].parse::<i32>().unwrap() );
	}

	col1.sort();
	col2.sort();

	let mut dist_sum: i32 = 0;
	let mut occ_sum: i32 = 0;
	let mut start: usize = 0;

	for i in 0..col1.len() {
		let a = col1[i];
		let b = col2[i];

		dist_sum += ( a - b ).abs();

		// NOTE: Could be further optimized by also keeping
		// a map of already checked values from col1 and their
		// number of occurences.
		let occurences = count_occurences( &a, &col2, &mut start );
		start = occurences.1;

		occ_sum += a * occurences.0;
	}

	println!( "(Part 1) The distance score is: {}", dist_sum );
	println!( "(Part 2) The similarity score is: {}", occ_sum );
}
