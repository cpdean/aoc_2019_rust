use std::fs;

//type Result<T> = ::std::result::Result<T, dyn std::error::Error>;

#[derive(Debug, PartialEq)]
pub enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    Mult(ParameterMode, ParameterMode, ParameterMode),
    Add(ParameterMode, ParameterMode, ParameterMode),
    TakeInput,
    ReturnInput,
    Halt,
}

pub enum InstructionClass {
    Mult,
    Add,
    TakeInput,
    ReturnInput,
    Halt,
}

pub fn parse_opcode(instruction: i32) -> Opcode {
    use Opcode::*;
    use ParameterMode::*;
    let inst_class = match instruction % 10 {
        1 => InstructionClass::Add,
        2 => InstructionClass::Mult,
        3 => InstructionClass::TakeInput,
        4 => InstructionClass::ReturnInput,
        9 => InstructionClass::Halt, // should be 99 but oh well
        x => {
            panic!("got a {:?}, from {:?}", x, instruction);
        }
    };
    match inst_class {
        InstructionClass::Add => {
            let first_param = if instruction / 100 % 10 == 0 {
                Position
            } else {
                Immediate
            };
            let second_param = if instruction / 1000 % 10 == 0 {
                Position
            } else {
                Immediate
            };
            let third_param = if instruction / 10000 % 10 == 0 {
                Position
            } else {
                Immediate
            };
            Add(first_param, second_param, third_param)
        }
        InstructionClass::Mult => {
            let first_param = if instruction / 100 % 10 == 0 {
                Position
            } else {
                Immediate
            };
            let second_param = if instruction / 1000 % 10 == 0 {
                Position
            } else {
                Immediate
            };
            let third_param = if instruction / 10000 % 10 == 0 {
                Position
            } else {
                Immediate
            };
            Mult(first_param, second_param, third_param)
        }
        InstructionClass::Halt => Halt, // should be 99 but oh well
        InstructionClass::TakeInput => TakeInput,
        InstructionClass::ReturnInput => ReturnInput,
    }
}

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day05.txt")?;
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
    let instructions = input_state.clone();
    let mut stdin_stdout = vec![1];
    let _final_state = run_program(instructions, &mut stdin_stdout);
    dbg!(stdin_stdout);
    /*
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
    */
    Ok(())
}

fn wrap_pos(try_pos: usize, length: usize) -> usize {
    if try_pos > length {
        try_pos - length
    } else {
        try_pos
    }
}
fn get_val(position: usize, _mode: ParameterMode, program: &Vec<i32>) -> i32 {
    let a = wrap_pos(position, program.len() - 1);
    match _mode {
        ParameterMode::Immediate => program[a],
        ParameterMode::Position => program[program[a] as usize],
    }
}

pub fn step_forward(
    position: usize,
    mut program: Vec<i32>,
    stdin_stdout: &mut Vec<i32>,
) -> (usize, Vec<i32>) {
    use Opcode::*;
    let instruction = parse_opcode(program[position]);
    let (new_pos, new_state) = match instruction {
        Add(arg1, arg2, _arg3) => {
            dbg!(&arg1);
            dbg!(&arg2);
            dbg!(&_arg3);
            let (left, right, destination_pos) = (
                get_val(position + 1, arg1, &program),
                get_val(position + 2, arg2, &program),
                get_val(position + 3, ParameterMode::Immediate, &program) as usize,
            );
            dbg!(&left);
            dbg!(&right);
            program[destination_pos] = left + right;
            dbg!(program[destination_pos]);
            (position + 4, program)
        }
        Mult(arg1, arg2, _arg3) => {
            let (left, right, destination_pos) = (
                get_val(position + 1, arg1, &program),
                get_val(position + 2, arg2, &program),
                get_val(position + 3, ParameterMode::Immediate, &program) as usize,
            );
            program[destination_pos] = left * right;
            (position + 4, program)
        }
        TakeInput => {
            let the_data = stdin_stdout[0];
            dbg!(the_data);
            let address = program[wrap_pos(position + 1, program.len() - 1) as usize] as usize;
            dbg!(address);
            dbg!(program[address]);
            program[address] = the_data;
            dbg!(program[address]);
            (position + 2, program)
        }
        ReturnInput => {
            let address = program[wrap_pos(position + 1, program.len() - 1) as usize] as usize;
            let the_data = program[address];
            stdin_stdout[0] = the_data;
            (position + 2, program)
        }
        Halt => {
            println!("got a stop");
            (position + 4, program)
        }
    };
    (wrap_pos(new_pos, new_state.len()), new_state)
}

pub fn run_program(mut program: Vec<i32>, mut stdin_stdout: &mut Vec<i32>) -> Vec<i32> {
    let mut counter = 0;
    let mut position = 0;
    loop {
        let peek_instr = program[position as usize];
        dbg!(counter);
        dbg!(peek_instr);
        if peek_instr == 99 {
            return program;
        } else if counter > 1000 {
            panic!("infinite loop?");
        } else {
            let (i, s) = step_forward(position, program, &mut stdin_stdout);
            position = i;
            program = s;
            counter += 1;
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
        let (next_position, next_program) = step_forward(position, program, &mut vec![0]);
        assert_eq!(next_position, 4);
        assert_eq!(
            next_program,
            vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn it_works2() {
        let program = vec![1101, 100, -1, 4, 0];
        let position = 0;
        let (next_position, next_program) = step_forward(position, program, &mut vec![0]);
        assert_eq!(next_position, 4);
        assert_eq!(next_program, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn step_2() {
        let program = vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let position = 4;
        let (next_position, next_program) = step_forward(position, program, &mut vec![0]);
        assert_eq!(next_position, 8);
        assert_eq!(
            next_program,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_run() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let next_program = run_program(program, &mut vec![0]);
        assert_eq!(
            next_program,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_halt_parse1() {
        let i = parse_opcode(99);
        assert_eq!(i, Opcode::Halt);
    }

    #[test]
    fn test_add_parse1() {
        let i = parse_opcode(1);
        assert_eq!(
            i,
            Opcode::Add(
                ParameterMode::Position,
                ParameterMode::Position,
                ParameterMode::Position
            )
        );
    }

    #[test]
    fn test_mult_parse1() {
        let i = parse_opcode(2);
        assert_eq!(
            i,
            Opcode::Mult(
                ParameterMode::Position,
                ParameterMode::Position,
                ParameterMode::Position
            )
        );
    }

    #[test]
    fn test_mult_parse2_example() {
        let i = parse_opcode(1002);
        assert_eq!(
            i,
            Opcode::Mult(
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position
            )
        );
    }

    #[test]
    fn test_mult_parse2_example2() {
        // this is actually broken, you can't have an instr param be immediate....
        let i = parse_opcode(11002);
        assert_eq!(
            i,
            Opcode::Mult(
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Immediate
            )
        );
    }

    #[test]
    fn test_mult_parse2_example3() {
        let i = parse_opcode(1102);
        assert_eq!(
            i,
            Opcode::Mult(
                ParameterMode::Immediate,
                ParameterMode::Immediate,
                ParameterMode::Position
            )
        );
    }
}
