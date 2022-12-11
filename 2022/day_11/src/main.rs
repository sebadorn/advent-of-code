use std::env;
use std::fs;


#[derive(Debug)]
enum MonkeyOperation {
    Add,
    Multiply,
    None,
    Square,
}


#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<i64>,
    num_inspections: i64,
    operation: MonkeyOperation,
    op_value: i64,
    test_value: i64,
    test_true_target: usize,
    test_false_target: usize,
}


impl Monkey {

    fn do_turn(&mut self, worry_reduction: &i64, worry_simplifier: &i64) -> Vec<(usize, i64)> {
        let mut list: Vec<(usize, i64)> = vec![];

        for item in &self.items {
            self.num_inspections += 1;

            let mut worry = self.get_new_worry_level(item);

            if *worry_reduction > 0 {
                worry = (worry as f64 / *worry_reduction as f64).floor() as i64;
            }

            worry = worry % *worry_simplifier;

            let next_monkey = self.get_monkey_target(&worry);

            list.push((next_monkey, worry));
        }

        self.items.clear();

        list
    }

    fn get_monkey_target(&self, worry: &i64) -> usize {
        match *worry % self.test_value {
            0 => self.test_true_target,
            _ => self.test_false_target,
        }
    }

    fn get_new_worry_level(&self, worry: &i64) -> i64 {
        match self.operation {
            MonkeyOperation::Add => *worry + self.op_value,
            MonkeyOperation::Multiply => *worry * self.op_value,
            MonkeyOperation::Square => *worry * *worry,
            _ => panic!("Unknown operation"),
        }
    }

}


fn get_last_value(line: &str) -> i64 {
    let last_pos = line.rfind(' ').unwrap() + 1;
    let last_value = &line[last_pos..];

    if last_value == "old" {
        return 0
    }

    last_value.parse::<i64>()
        .expect(&format!("Failed to parse: {}", last_value))
}


fn get_operation(line: &str) -> MonkeyOperation {
    if line.contains(" + ") {
        return MonkeyOperation::Add;
    }
    else if line.contains(" * old") {
        return MonkeyOperation::Square;
    }
    else if line.contains(" * ") {
        return MonkeyOperation::Multiply;
    }

    MonkeyOperation::None
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut num_rounds = 20;
    let mut worry_reduction = 3;

    if args.contains(&"no_worry_reduction".to_owned()) {
        num_rounds = 10_000;
        worry_reduction = 0;
    }

    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let input = content.split("\n\n");
    let mut monkeys: Vec<Monkey> = vec![];

    for section in input {
        let lines: Vec<&str> = section.lines().collect();

        let items: Vec<i64> = lines[1][18..]
            .split(", ")
            .map(|x| x.parse::<i64>()
            .unwrap())
            .collect();

        let monkey = Monkey {
            id: monkeys.len(),
            items: items,
            num_inspections: 0,
            operation: get_operation(&lines[2]),
            op_value: get_last_value(&lines[2]),
            test_value: get_last_value(&lines[3]),
            test_true_target: get_last_value(&lines[4]) as usize,
            test_false_target: get_last_value(&lines[5]) as usize,
        };

        monkeys.push(monkey);
    }

    let mut worry_simplifier = 1;

    for monkey in &monkeys {
        worry_simplifier *= monkey.test_value;
    }

    for _round in 0..num_rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let item_targets: Vec<(usize, i64)> = monkey.do_turn(&worry_reduction, &worry_simplifier);

            for target in item_targets {
                monkeys[target.0].items.push(target.1);
            }
        }
    }

    monkeys.sort_by(|a, b| b.num_inspections.cmp(&a.num_inspections));

    for i in 0..monkeys.len() {
        let monkey = &monkeys[i];
        println!("Monkey {} inspected items {} times.", monkey.id, monkey.num_inspections);
    }

    let monkey_business = monkeys[0].num_inspections * monkeys[1].num_inspections;
    println!("The level of monkey business is {}.", monkey_business);
}
