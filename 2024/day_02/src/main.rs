use std::fs;


fn is_safe( report: &Vec<i32>, ignore: usize ) -> bool {
    let start: usize = if ignore == 0 { 2 } else { 1 };
    let mut old_dir: i8 = 0;
    let mut old_level = report[start - 1];

    for i in start..report.len() {
        if ignore == i {
            continue;
        }

        let level = report[i];
        let diff = old_level - level;
        let diff_abs = diff.abs();

        if diff_abs < 1 || diff_abs > 3 {
            return false;
        }

        let dir: i8 = if diff < 0 { -1 } else { 1 };

        if old_dir == 0 {
            old_dir = dir;
        }
        else if old_dir != dir {
            return false;
        }

        old_level = level;
    }

    true
}


fn is_safe_dampened( report: &Vec<i32> ) -> bool {
    for i in 0..report.len() {
        if is_safe( report, i ) {
            return true;
        }
    }

    false
}


fn main() {
    let contents = fs::read_to_string( "input.txt" )
        .expect( "Failed to read file." );
    let lines: Vec<&str> = contents.split( "\n" ).collect();

    let mut safe_reports: i32 = 0;
    let mut safe_if_dampened: i32 = 0;

    for ( _index, line ) in lines.iter().enumerate() {
        if line.len() == 0 {
            continue;
        }

        let levels: Vec<i32> = line.split( " " )
            .map( |x| x.parse::<i32>().unwrap() )
            .collect();

        if is_safe( &levels, usize::MAX ) {
            safe_reports += 1;
            safe_if_dampened += 1;
        }
        else if is_safe_dampened( &levels ) {
            safe_if_dampened += 1;
        }
    }

    println!( "(Part 1) Safe reports: {}", safe_reports );
    println!( "(Part 2) Safe reports if dampened: {}", safe_if_dampened );
}
