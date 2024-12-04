use std::fs;


fn check_for_xmas( map: &Vec<Vec<char>>, i: usize, j: usize ) -> i32 {
    let mut sum = 0;
    let line = &map[i];

    let right_ok = j < line.len() - 3;
    let left_ok = j >= 3;
    let up_ok = i >= 3;
    let down_ok = i < map.len() - 3;

    // check right
    if right_ok {
        sum += if line[j + 1] == 'M' && line[j + 2] == 'A' && line[j + 3] == 'S' { 1 } else { 0 };

        // check right-up
        if up_ok {
            sum += if map[i - 1][j + 1] == 'M' && map[i - 2][j + 2] == 'A' && map[i - 3][j + 3] == 'S' { 1 } else { 0 };
        }

        // check right-down
        if down_ok {
            sum += if map[i + 1][j + 1] == 'M' && map[i + 2][j + 2] == 'A' && map[i + 3][j + 3] == 'S' { 1 } else { 0 };
        }
    }

    // check left
    if left_ok {
        sum += if line[j - 1] == 'M' && line[j - 2] == 'A' && line[j - 3] == 'S' { 1 } else { 0 };

        // check left-up
        if up_ok {
            sum += if map[i - 1][j - 1] == 'M' && map[i - 2][j - 2] == 'A' && map[i - 3][j - 3] == 'S' { 1 } else { 0 };
        }

        // check left-down
        if down_ok {
            sum += if map[i + 1][j - 1] == 'M' && map[i + 2][j - 2] == 'A' && map[i + 3][j - 3] == 'S' { 1 } else { 0 };
        }
    }

    // check up
    if up_ok {
        sum += if map[i - 1][j] == 'M' && map[i - 2][j] == 'A' && map[i - 3][j] == 'S' { 1 } else { 0 };
    }

    // check down
    if down_ok {
        sum += if map[i + 1][j] == 'M' && map[i + 2][j] == 'A' && map[i + 3][j] == 'S' { 1 } else { 0 };
    }

    sum
}


fn check_for_x_mas( map: &Vec<Vec<char>>, i: usize, j: usize ) -> i32 {
    if i == 0 || i >= map.len() - 1 || j == 0 || j >= map[0].len() - 1 {
        return 0;
    }

    let lt = map[i - 1][j - 1];
    let lb = map[i + 1][j - 1];
    let rt = map[i - 1][j + 1];
    let rb = map[i + 1][j + 1];

    let diag_1_ok = ( lt == 'M' && rb == 'S' ) || ( lt == 'S' && rb == 'M' );
    let diag_2_ok = ( rt == 'M' && lb == 'S' ) || ( rt == 'S' && lb == 'M' );

    if diag_1_ok && diag_2_ok { 1 } else { 0 }
}


fn main() {
    let contents = fs::read_to_string( "input.txt" )
        .expect( "Failed to read file." );
    let lines: Vec<&str> = contents.split( "\n" ).collect();

    let mut map: Vec<Vec<char>> = lines.iter().map( |l| l.chars().collect() ).collect();
    map.pop(); // remove the last empty line

    let mut xmas_sum = 0;
    let mut x_mas_sum = 0;

    for i in 0..map.len() {
        let line = &map[i];

        for j in 0..line.len() {
            let c: char = line[j];

            if c == 'X' {
                xmas_sum += check_for_xmas( &map, i, j );
            }
            else if c == 'A' {
                x_mas_sum += check_for_x_mas( &map, i, j );
            }
        }
    }

    println!( "(Part 1) XMAS count: {}", xmas_sum );
    println!( "(Part 2) X-MAS count: {}", x_mas_sum );
}
