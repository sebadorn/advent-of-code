use std::collections::HashMap;
use std::fs;


#[derive(Clone, Debug)]
struct Equation {
    result: i64,
    values: Vec<i64>,
}

impl Equation {
    fn new( result: i64, values: Vec<i64> ) -> Self {
        Equation { result, values }
    }

    fn check( &self, operators: &Vec<Operator> ) -> bool {
        let mut acc = self.values[0];

        for i in 0..operators.len() {
            let op = &operators[i];

            match op {
                Operator::Add => acc += self.values[i + 1],
                Operator::Mul => acc *= self.values[i + 1],
                Operator::Con => {
                    let as_text = format!( "{}{}", acc, self.values[i + 1] );
                    acc = as_text.parse::<i64>().unwrap();
                },
            }
        }

        acc == self.result
    }
}


#[derive(Clone, Debug, Hash, PartialEq)]
enum Operator {
    Add,
    Mul,
    Con,
}


fn check_equation( eq: &Equation, combinations: &HashMap<usize, Vec<Vec<Operator>>> ) -> i64 {
    let operators = combinations.get( &eq.values.len() ).unwrap();

    for operator in operators {
        if eq.check( &operator ) {
            return eq.result;
        }
    }

    0
}


fn gen_combinations( options: &Vec<Operator>, len: usize ) -> Vec<Vec<Operator>> {
    if len == 0 {
        return (*options.into_iter()
            .map( |o| vec![o.clone()] )
            .collect::<Vec<Vec<Operator>>>()).to_vec();
    }

    let sub_combinations = gen_combinations( options, len - 1 );
    let mut list = Vec::new();

    for comb in sub_combinations {
        for option in options {
            let mut new_comb = comb.clone();
            new_comb.push( option.clone() );
            list.push( new_comb );
        }
    }

    list
}


fn main() {
    let contents = fs::read_to_string( "input.txt" )
        .expect( "Failed to read file." );
    let lines: Vec<&str> = contents.split_terminator( "\n" ).collect();

    let combs_2 = vec![Operator::Add, Operator::Mul];
    let combs_3 = vec![Operator::Add, Operator::Mul, Operator::Con];
    let mut combs_2_map: HashMap<usize, Vec<Vec<Operator>>> = HashMap::new();
    let mut combs_3_map: HashMap<usize, Vec<Vec<Operator>>> = HashMap::new();

    let mut equations: Vec<Equation> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split( ": " ).collect();
        let result = parts[0].parse::<i64>().unwrap();
        let values = parts[1].split_terminator( " " ).map( |v| v.parse::<i64>().unwrap() ).collect();

        let eq = Equation::new( result, values );
        let len = eq.values.len();

        if combs_2_map.get( &len ).is_none() {
            combs_2_map.insert( len, gen_combinations( &combs_2, len - 2 ) );
            combs_3_map.insert( len, gen_combinations( &combs_3, len - 2 ) );
        }

        equations.push( eq );
    }

    let mut calibration_2 = 0;
    let mut calibration_3 = 0;

    for eq in equations {
        calibration_2 += check_equation( &eq, &combs_2_map );
        calibration_3 += check_equation( &eq, &combs_3_map );
    }

    println!( "(Part 1) Calibration result: {}", calibration_2 );
    println!( "(Part 2) Calibration result: {}", calibration_3 );
}
