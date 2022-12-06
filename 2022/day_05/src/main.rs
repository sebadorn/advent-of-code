use std::env;
use std::fs;


fn create_empty_stacks(lines: &Vec<&str>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let first_line = lines[0].trim();
    let mut digits_str: String = String::from("");

    for character in first_line.chars() {
        if character.to_digit(10).is_none() {
            if digits_str.len() > 0 {
                stacks.push(Vec::new());
            }

            digits_str.clear();
        }
        else {
            digits_str.push(character);
        }
    }

    stacks.push(Vec::new());

    stacks
}


fn fill_stacks(lines: &Vec<&str>, stacks: &mut Vec<Vec<char>>) {
    let stacks_len = stacks.len();
    println!("There are {} stacks.", stacks_len);

    for (index, line) in lines.into_iter().enumerate() {
        if index == 0 {
            continue;
        }

        let mut stack_index = 0;
        let mut stack = &mut stacks[stack_index];
        let mut open = false;
        let mut num_spaces = 0;

        println!("{}", line);

        for character in line.chars() {
            if stack_index < stacks_len {
                stack = &mut stacks[stack_index];
            }

            match character {
                '[' => {
                    open = true;
                    num_spaces = 0;
                },
                ']' => {
                    open = false;
                    num_spaces = 0;
                },
                ' ' => {
                    num_spaces += 1;

                    if num_spaces == 4 {
                        num_spaces = 0;
                        stack_index += 1;
                    }
                },
                _ => {
                    num_spaces = 0;

                    if open {
                        stack.push(character);
                        stack_index += 1;
                    }
                },
            }
        }
    }
}


fn move_crates(
    commands_str: &str,
    stacks: &mut Vec<Vec<char>>,
    mode: &CrateMover
) {
    println!("Moving crates...");

    for line in commands_str.lines() {
        let parts: Vec<&str> = line.split(' ').collect();

        if parts[0] != "move" {
            println!("Line not starting with \"move\".");
            break;
        }

        let quantity = parts[1].parse::<usize>().unwrap();
        let index_from = parts[3].parse::<usize>().unwrap() - 1;
        let index_to = parts[5].parse::<usize>().unwrap() - 1;

        let stack_from = &mut stacks[index_from];
        let mut tmp_stack: Vec<char> = Vec::new();

        match *mode {
            CrateMover::ModeSingle => {
                for _i in 0..quantity {
                    let item = stack_from.pop();

                    if item.is_none() {
                        println!("{:?}", stack_from);
                        panic!("Item is None! Stack is already empty!");
                    }

                    tmp_stack.push(item.unwrap());
                }
            },
            CrateMover::ModeMulti => {
                let split_index = stack_from.len() - quantity;
                tmp_stack = stack_from.split_off(split_index);
            }
        }

        let stack_to = &mut stacks[index_to];
        stack_to.append(&mut tmp_stack);
    }

    println!("Crates have been moved.");
}


#[derive(PartialEq)]
enum CrateMover {
    ModeSingle = 1,
    ModeMulti = 2,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut mode = CrateMover::ModeSingle;

    if args.contains(&"cm9001".to_owned()) {
        mode = CrateMover::ModeMulti;
    }

    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt.");

    let split_index = content.find("move").unwrap();
    let (mut stacks_str, mut commands_str) = content.split_at(split_index);
    stacks_str = stacks_str.trim();
    commands_str = commands_str.trim();

    let lines: Vec<&str> = stacks_str.lines().rev().collect();
    let mut stacks: Vec<Vec<char>> = create_empty_stacks(&lines);

    fill_stacks(&lines, &mut stacks);
    move_crates(&commands_str, &mut stacks, &mode);

    let mut top_row = String::from("");

    for stack in stacks.into_iter() {
        let character = stack.last().unwrap();
        top_row.push(*character);
    }

    println!("The top row spells: {}", top_row);
}
