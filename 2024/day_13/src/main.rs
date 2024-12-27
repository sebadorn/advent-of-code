use std::fs;


struct Machine {
    btn_a: (i128, i128),
    btn_b: (i128, i128),
    prize: (i128, i128),
}

impl Machine {
    fn solve_x( &self, modifier: i128 ) -> i128 {
        let a = self.btn_a.0;
        let b = self.btn_b.0;
        let c = self.prize.0 + modifier;
        let d = self.btn_a.1;
        let e = self.btn_b.1;
        let f = self.prize.1 + modifier;

        let det = d * b - e * a;

        if det == 0 {
            return -1;
        }

        let denom = b * f - c * e;

        if denom % det != 0 {
            return -1;
        }

        denom / det
    }

    fn solve_y( &self, modifier: i128 ) -> i128 {
        let a = self.btn_a.0;
        let b = self.btn_b.0;
        let c = self.prize.0 + modifier;
        let d = self.btn_a.1;
        let e = self.btn_b.1;
        let f = self.prize.1 + modifier;

        let det = d * b - e * a;

        if det == 0 {
            return -1;
        }

        let denom = d * c - a * f;

        if denom % det != 0 {
            return -1;
        }

        denom / det
    }

    fn tokens_required( &self, modifier: i128, limit: i128 ) -> i128 {
        let x = self.solve_x( modifier );

        if x < 0 || ( limit > 0 && x > limit ) {
            return -1;
        }

        let y = self.solve_y( modifier );

        if y < 0 || ( limit > 0 && y > limit ) {
            return -1;
        }

        x * 3 + y
    }
}


fn main() {
    let contents = fs::read_to_string( "input.txt" )
        .expect( "Failed to read file." );
    let entries: Vec<&str> = contents.split_terminator( "\n\n" ).collect();

    let mut machines: Vec<Machine> = Vec::new();

    for entry in entries {
        let lines: Vec<&str> = entry.split_terminator( "\n" ).collect();
        let btn_a: Vec<&str> = lines[0][10..].split_terminator( ", " ).collect();
        let btn_b: Vec<&str> = lines[1][10..].split_terminator( ", " ).collect();
        let prize: Vec<&str> = lines[2][7..].split_terminator( ", " ).collect();

        let machine = Machine {
            btn_a: ( btn_a[0][2..].parse().unwrap(), btn_a[1][2..].parse().unwrap() ),
            btn_b: ( btn_b[0][2..].parse().unwrap(), btn_b[1][2..].parse().unwrap() ),
            prize: ( prize[0][2..].parse().unwrap(), prize[1][2..].parse().unwrap() ),
        };
        machines.push( machine );
    }

    let mut sum_prices = 0;
    let mut sum_tokens = 0;

    for machine in &machines {
        let tokens = machine.tokens_required( 0, 100 );

        if tokens > -1 {
            sum_prices += 1;
            sum_tokens += tokens;
        }
    }

    println!( "(Part 1) Number of prices: {} (for {} tokens)", sum_prices, sum_tokens );

    sum_prices = 0;
    sum_tokens = 0;

    for machine in &machines {
        let tokens = machine.tokens_required( 10_000_000_000_000, 0 );

        if tokens > -1 {
            sum_prices += 1;
            sum_tokens += tokens;
        }
    }

    println!( "(Part 2) Number of prices: {} (for {} tokens)", sum_prices, sum_tokens );
}
