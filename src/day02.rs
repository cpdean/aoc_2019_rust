use std::fs;

//type Result<T> = ::std::result::Result<T, dyn std::error::Error>;

pub fn fuel_cost(n: f64) -> f64 {
    (n / 3.0).trunc() - 2.0
}

pub fn true_fuel_cost(n: f64) -> f64 {
    let cost = fuel_cost(n);
    if cost <= 0.0 {
        return 0.0;
    }
    cost + true_fuel_cost(cost)
}

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day02.txt")?;
    let input_state: Vec<i32> = f
        .trim()
        .split(",")
        .map(|e| {
            let i: i32 = match e.parse() {
                Ok(x) => x,
                Err(error) => panic!("what is this <{}>", error),
            };
            i
        })
        .collect();
    /* prompt:
     * Once you have a working computer, the first step is to restore the gravity assist
     * program (your puzzle input) to the "1202 program alarm" state it had just before the
     * last computer caught fire. To do this, before running the program, replace position 1
     * with the value 12 and replace position 2 with the value 2. What value is left at
     * position 0 after the program halts?
     */
    let mut instructions = input_state.clone();
    println!("sum {}", input_state.iter().fold(0, |a, e| a + e));
    instructions[1] = 12;
    instructions[2] = 2;
    let final_state = run_program(instructions);
    println!("part 1: {}", final_state[0]);
    // part 2
    let target = 19690720;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut part2 = input_state.clone();
            part2[1] = noun;
            part2[2] = verb;
            let attempt = run_program(part2);
            if attempt[0] == target {
                println!("part2 100 * {} + {} = {}", noun, verb, 100 * noun + verb);
            }
        }
    }
    Ok(())
}

fn wrap_pos(try_pos: usize, length: usize) -> usize {
    if try_pos > length {
        try_pos - length
    } else {
        try_pos
    }
}

pub fn step_forward(position: usize, mut program: Vec<i32>) -> (usize, Vec<i32>) {
    let instruction = program[position];
    let new_state = match instruction {
        1 => {
            let arg1 = wrap_pos(position + 1, program.len() - 1);
            let arg2 = wrap_pos(position + 2, program.len() - 1);
            let arg3 = wrap_pos(position + 3, program.len() - 1);
            let (left_pos, right_pos, destination_pos) = (
                program[arg1] as usize,
                program[arg2] as usize,
                program[arg3] as usize,
            );
            let (left, right) = (program[left_pos], program[right_pos]);
            program[destination_pos] = left + right;
            program
        }
        2 => {
            let arg1 = wrap_pos(position + 1, program.len() - 1);
            let arg2 = wrap_pos(position + 2, program.len() - 1);
            let arg3 = wrap_pos(position + 3, program.len() - 1);
            let (left_pos, right_pos, destination_pos) = (
                program[arg1] as usize,
                program[arg2] as usize,
                program[arg3] as usize,
            );
            let (left, right) = (program[left_pos], program[right_pos]);
            program[destination_pos] = left * right;
            program
        }
        99 => {
            println!("got a stop");
            program
        }
        x => {
            panic!("got a {}", x);
        }
    };
    (wrap_pos(position + 4, new_state.len()), new_state)
}

pub fn run_program(mut program: Vec<i32>) -> Vec<i32> {
    let counter = 0;
    let mut position = 0;
    loop {
        let peek_instr = program[position as usize];
        if peek_instr == 99 {
            return program;
        } else if counter > 1000 {
            panic!("infinite loop?");
        } else {
            let (i, s) = step_forward(position, program);
            position = i;
            program = s;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let position = 0;
        let (next_position, next_program) = step_forward(position, program);
        assert_eq!(next_position, 4);
        assert_eq!(
            next_program,
            vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn step_2() {
        let program = vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let position = 4;
        let (next_position, next_program) = step_forward(position, program);
        assert_eq!(next_position, 8);
        assert_eq!(
            next_program,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_run() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let next_program = run_program(program);
        assert_eq!(
            next_program,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }
}
