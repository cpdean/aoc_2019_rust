/// day 7, chaining computers together
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
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
    Halt,
}

pub enum InstructionClass {
    Mult,
    Add,
    TakeInput,
    ReturnInput,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
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
        5 => InstructionClass::JumpIfTrue,
        6 => InstructionClass::JumpIfFalse,
        7 => InstructionClass::LessThan,
        8 => InstructionClass::Equals,
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
        InstructionClass::JumpIfTrue => {
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
            JumpIfTrue(first_param, second_param)
        }
        InstructionClass::JumpIfFalse => {
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
            JumpIfFalse(first_param, second_param)
        }
        InstructionClass::LessThan => {
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
            LessThan(first_param, second_param, third_param)
        }
        InstructionClass::Equals => {
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
            Equals(first_param, second_param, third_param)
        }
    }
}

pub fn input_combinations_part2() -> Vec<Vec<i32>> {
    vec![
        vec![5, 6, 7, 8, 9],
        vec![5, 6, 7, 9, 8],
        vec![5, 6, 8, 7, 9],
        vec![5, 6, 8, 9, 7],
        vec![5, 6, 9, 7, 8],
        vec![5, 6, 9, 8, 7],
        vec![5, 7, 6, 8, 9],
        vec![5, 7, 6, 9, 8],
        vec![5, 7, 8, 6, 9],
        vec![5, 7, 8, 9, 6],
        vec![5, 7, 9, 6, 8],
        vec![5, 7, 9, 8, 6],
        vec![5, 8, 6, 7, 9],
        vec![5, 8, 6, 9, 7],
        vec![5, 8, 7, 6, 9],
        vec![5, 8, 7, 9, 6],
        vec![5, 8, 9, 6, 7],
        vec![5, 8, 9, 7, 6],
        vec![5, 9, 6, 7, 8],
        vec![5, 9, 6, 8, 7],
        vec![5, 9, 7, 6, 8],
        vec![5, 9, 7, 8, 6],
        vec![5, 9, 8, 6, 7],
        vec![5, 9, 8, 7, 6],
        vec![6, 5, 7, 8, 9],
        vec![6, 5, 7, 9, 8],
        vec![6, 5, 8, 7, 9],
        vec![6, 5, 8, 9, 7],
        vec![6, 5, 9, 7, 8],
        vec![6, 5, 9, 8, 7],
        vec![6, 7, 5, 8, 9],
        vec![6, 7, 5, 9, 8],
        vec![6, 7, 8, 5, 9],
        vec![6, 7, 8, 9, 5],
        vec![6, 7, 9, 5, 8],
        vec![6, 7, 9, 8, 5],
        vec![6, 8, 5, 7, 9],
        vec![6, 8, 5, 9, 7],
        vec![6, 8, 7, 5, 9],
        vec![6, 8, 7, 9, 5],
        vec![6, 8, 9, 5, 7],
        vec![6, 8, 9, 7, 5],
        vec![6, 9, 5, 7, 8],
        vec![6, 9, 5, 8, 7],
        vec![6, 9, 7, 5, 8],
        vec![6, 9, 7, 8, 5],
        vec![6, 9, 8, 5, 7],
        vec![6, 9, 8, 7, 5],
        vec![7, 5, 6, 8, 9],
        vec![7, 5, 6, 9, 8],
        vec![7, 5, 8, 6, 9],
        vec![7, 5, 8, 9, 6],
        vec![7, 5, 9, 6, 8],
        vec![7, 5, 9, 8, 6],
        vec![7, 6, 5, 8, 9],
        vec![7, 6, 5, 9, 8],
        vec![7, 6, 8, 5, 9],
        vec![7, 6, 8, 9, 5],
        vec![7, 6, 9, 5, 8],
        vec![7, 6, 9, 8, 5],
        vec![7, 8, 5, 6, 9],
        vec![7, 8, 5, 9, 6],
        vec![7, 8, 6, 5, 9],
        vec![7, 8, 6, 9, 5],
        vec![7, 8, 9, 5, 6],
        vec![7, 8, 9, 6, 5],
        vec![7, 9, 5, 6, 8],
        vec![7, 9, 5, 8, 6],
        vec![7, 9, 6, 5, 8],
        vec![7, 9, 6, 8, 5],
        vec![7, 9, 8, 5, 6],
        vec![7, 9, 8, 6, 5],
        vec![8, 5, 6, 7, 9],
        vec![8, 5, 6, 9, 7],
        vec![8, 5, 7, 6, 9],
        vec![8, 5, 7, 9, 6],
        vec![8, 5, 9, 6, 7],
        vec![8, 5, 9, 7, 6],
        vec![8, 6, 5, 7, 9],
        vec![8, 6, 5, 9, 7],
        vec![8, 6, 7, 5, 9],
        vec![8, 6, 7, 9, 5],
        vec![8, 6, 9, 5, 7],
        vec![8, 6, 9, 7, 5],
        vec![8, 7, 5, 6, 9],
        vec![8, 7, 5, 9, 6],
        vec![8, 7, 6, 5, 9],
        vec![8, 7, 6, 9, 5],
        vec![8, 7, 9, 5, 6],
        vec![8, 7, 9, 6, 5],
        vec![8, 9, 5, 6, 7],
        vec![8, 9, 5, 7, 6],
        vec![8, 9, 6, 5, 7],
        vec![8, 9, 6, 7, 5],
        vec![8, 9, 7, 5, 6],
        vec![8, 9, 7, 6, 5],
        vec![9, 5, 6, 7, 8],
        vec![9, 5, 6, 8, 7],
        vec![9, 5, 7, 6, 8],
        vec![9, 5, 7, 8, 6],
        vec![9, 5, 8, 6, 7],
        vec![9, 5, 8, 7, 6],
        vec![9, 6, 5, 7, 8],
        vec![9, 6, 5, 8, 7],
        vec![9, 6, 7, 5, 8],
        vec![9, 6, 7, 8, 5],
        vec![9, 6, 8, 5, 7],
        vec![9, 6, 8, 7, 5],
        vec![9, 7, 5, 6, 8],
        vec![9, 7, 5, 8, 6],
        vec![9, 7, 6, 5, 8],
        vec![9, 7, 6, 8, 5],
        vec![9, 7, 8, 5, 6],
        vec![9, 7, 8, 6, 5],
        vec![9, 8, 5, 6, 7],
        vec![9, 8, 5, 7, 6],
        vec![9, 8, 6, 5, 7],
        vec![9, 8, 6, 7, 5],
        vec![9, 8, 7, 5, 6],
        vec![9, 8, 7, 6, 5],
    ]
}

pub fn input_combinations_part1() -> Vec<Vec<i32>> {
    /* I didn't want to figure out how to translate a permutation alg to rust because of its
     * memory issues. generated this source code from a python script lol:
     *
     *     from itertools import permutations
     *     print("vec![")
     *     for p in permutations(range(5), 5):
     *         print("vec!{},".format(list(p)))
     *     print("]")
     *
     */
    vec![
        vec![0, 1, 2, 3, 4],
        vec![0, 1, 2, 4, 3],
        vec![0, 1, 3, 2, 4],
        vec![0, 1, 3, 4, 2],
        vec![0, 1, 4, 2, 3],
        vec![0, 1, 4, 3, 2],
        vec![0, 2, 1, 3, 4],
        vec![0, 2, 1, 4, 3],
        vec![0, 2, 3, 1, 4],
        vec![0, 2, 3, 4, 1],
        vec![0, 2, 4, 1, 3],
        vec![0, 2, 4, 3, 1],
        vec![0, 3, 1, 2, 4],
        vec![0, 3, 1, 4, 2],
        vec![0, 3, 2, 1, 4],
        vec![0, 3, 2, 4, 1],
        vec![0, 3, 4, 1, 2],
        vec![0, 3, 4, 2, 1],
        vec![0, 4, 1, 2, 3],
        vec![0, 4, 1, 3, 2],
        vec![0, 4, 2, 1, 3],
        vec![0, 4, 2, 3, 1],
        vec![0, 4, 3, 1, 2],
        vec![0, 4, 3, 2, 1],
        vec![1, 0, 2, 3, 4],
        vec![1, 0, 2, 4, 3],
        vec![1, 0, 3, 2, 4],
        vec![1, 0, 3, 4, 2],
        vec![1, 0, 4, 2, 3],
        vec![1, 0, 4, 3, 2],
        vec![1, 2, 0, 3, 4],
        vec![1, 2, 0, 4, 3],
        vec![1, 2, 3, 0, 4],
        vec![1, 2, 3, 4, 0],
        vec![1, 2, 4, 0, 3],
        vec![1, 2, 4, 3, 0],
        vec![1, 3, 0, 2, 4],
        vec![1, 3, 0, 4, 2],
        vec![1, 3, 2, 0, 4],
        vec![1, 3, 2, 4, 0],
        vec![1, 3, 4, 0, 2],
        vec![1, 3, 4, 2, 0],
        vec![1, 4, 0, 2, 3],
        vec![1, 4, 0, 3, 2],
        vec![1, 4, 2, 0, 3],
        vec![1, 4, 2, 3, 0],
        vec![1, 4, 3, 0, 2],
        vec![1, 4, 3, 2, 0],
        vec![2, 0, 1, 3, 4],
        vec![2, 0, 1, 4, 3],
        vec![2, 0, 3, 1, 4],
        vec![2, 0, 3, 4, 1],
        vec![2, 0, 4, 1, 3],
        vec![2, 0, 4, 3, 1],
        vec![2, 1, 0, 3, 4],
        vec![2, 1, 0, 4, 3],
        vec![2, 1, 3, 0, 4],
        vec![2, 1, 3, 4, 0],
        vec![2, 1, 4, 0, 3],
        vec![2, 1, 4, 3, 0],
        vec![2, 3, 0, 1, 4],
        vec![2, 3, 0, 4, 1],
        vec![2, 3, 1, 0, 4],
        vec![2, 3, 1, 4, 0],
        vec![2, 3, 4, 0, 1],
        vec![2, 3, 4, 1, 0],
        vec![2, 4, 0, 1, 3],
        vec![2, 4, 0, 3, 1],
        vec![2, 4, 1, 0, 3],
        vec![2, 4, 1, 3, 0],
        vec![2, 4, 3, 0, 1],
        vec![2, 4, 3, 1, 0],
        vec![3, 0, 1, 2, 4],
        vec![3, 0, 1, 4, 2],
        vec![3, 0, 2, 1, 4],
        vec![3, 0, 2, 4, 1],
        vec![3, 0, 4, 1, 2],
        vec![3, 0, 4, 2, 1],
        vec![3, 1, 0, 2, 4],
        vec![3, 1, 0, 4, 2],
        vec![3, 1, 2, 0, 4],
        vec![3, 1, 2, 4, 0],
        vec![3, 1, 4, 0, 2],
        vec![3, 1, 4, 2, 0],
        vec![3, 2, 0, 1, 4],
        vec![3, 2, 0, 4, 1],
        vec![3, 2, 1, 0, 4],
        vec![3, 2, 1, 4, 0],
        vec![3, 2, 4, 0, 1],
        vec![3, 2, 4, 1, 0],
        vec![3, 4, 0, 1, 2],
        vec![3, 4, 0, 2, 1],
        vec![3, 4, 1, 0, 2],
        vec![3, 4, 1, 2, 0],
        vec![3, 4, 2, 0, 1],
        vec![3, 4, 2, 1, 0],
        vec![4, 0, 1, 2, 3],
        vec![4, 0, 1, 3, 2],
        vec![4, 0, 2, 1, 3],
        vec![4, 0, 2, 3, 1],
        vec![4, 0, 3, 1, 2],
        vec![4, 0, 3, 2, 1],
        vec![4, 1, 0, 2, 3],
        vec![4, 1, 0, 3, 2],
        vec![4, 1, 2, 0, 3],
        vec![4, 1, 2, 3, 0],
        vec![4, 1, 3, 0, 2],
        vec![4, 1, 3, 2, 0],
        vec![4, 2, 0, 1, 3],
        vec![4, 2, 0, 3, 1],
        vec![4, 2, 1, 0, 3],
        vec![4, 2, 1, 3, 0],
        vec![4, 2, 3, 0, 1],
        vec![4, 2, 3, 1, 0],
        vec![4, 3, 0, 1, 2],
        vec![4, 3, 0, 2, 1],
        vec![4, 3, 1, 0, 2],
        vec![4, 3, 1, 2, 0],
        vec![4, 3, 2, 0, 1],
        vec![4, 3, 2, 1, 0],
    ]
}

pub fn find_max_signal(amplifier_software: Vec<i32>) -> i32 {
    let mut m = 0;
    for input_config in input_combinations_part1() {
        let signal = get_amplifier_signal_part1(&amplifier_software, input_config);
        if m < signal {
            m = signal
        }
    }
    m
}

pub fn find_max_signal_part2(amplifier_software: Vec<i32>) -> i32 {
    let mut m = 0;
    for input_config in input_combinations_part2() {
        let signal = get_amplifier_signal_part2(&amplifier_software, input_config);
        if m < signal {
            m = signal
        }
    }
    m
}

pub fn get_amplifier_signal_part1(
    amplifier_software: &Vec<i32>,
    mut input_config: Vec<i32>,
) -> i32 {
    // works by chaining several computers together, feeding each one an input plus a second
    // number you get from the previous computer

    // A->B->C->D->E-> {i32}
    assert_eq!(input_config.len(), 5);

    let mut stdin = vec![];
    let mut stdout = vec![];
    // run amp A
    let instructions = amplifier_software.clone();
    stdin.push(input_config.remove(0));
    stdin.push(0);
    run_program(instructions, &mut stdin, &mut stdout);
    // run amp B
    let instructions = amplifier_software.clone();
    stdin.push(input_config.remove(0));
    assert_eq!(stdout.len(), 1);
    stdin.push(stdout.remove(0));
    run_program(instructions, &mut stdin, &mut stdout);
    // run amp C
    let instructions = amplifier_software.clone();
    stdin.push(input_config.remove(0));
    assert_eq!(stdout.len(), 1);
    stdin.push(stdout.remove(0));
    run_program(instructions, &mut stdin, &mut stdout);
    // run amp D
    let instructions = amplifier_software.clone();
    stdin.push(input_config.remove(0));
    assert_eq!(stdout.len(), 1);
    stdin.push(stdout.remove(0));
    run_program(instructions, &mut stdin, &mut stdout);
    // run amp E
    let instructions = amplifier_software.clone();
    stdin.push(input_config.remove(0));
    assert_eq!(stdout.len(), 1);
    stdin.push(stdout.remove(0));
    run_program(instructions, &mut stdin, &mut stdout);
    assert_eq!(stdout.len(), 1);
    stdout.remove(0)
}

pub fn get_amplifier_signal_part2(
    amplifier_software: &Vec<i32>,
    mut input_config: Vec<i32>,
) -> i32 {
    // now the computers must run in a cycle, processing input from the computer in front of it
    // this is much harder because I have to re-do how halting works, implement blocking, and write
    // what is going to essentially be a scheduler to switch control flow to the next computer in
    // the ring

    // A->B->C->D->E->-+(when halts...)--> {i32}
    // ^               |
    // +-------<-------+
    //
    // additionally I cannot re-use the stdin/stdout pipes, i have to make a pipe for each
    // connection

    let mut stdin_a = vec![];
    let mut stdin_b = vec![];
    let mut stdin_c = vec![];
    let mut stdin_d = vec![];
    let mut stdin_e = vec![];
    // load initial params
    stdin_a.push(input_config.remove(0));
    stdin_b.push(input_config.remove(0));
    stdin_c.push(input_config.remove(0));
    stdin_d.push(input_config.remove(0));
    stdin_e.push(input_config.remove(0));
    // run amp A
    let instructions = amplifier_software.clone();
    stdin_a.push(0);
    run_program(instructions, &mut stdin_a, &mut stdin_b);
    // run amp b
    let instructions = amplifier_software.clone();
    run_program(instructions, &mut stdin_b, &mut stdin_c);
    // run amp c
    let instructions = amplifier_software.clone();
    run_program(instructions, &mut stdin_c, &mut stdin_d);
    // run amp d
    let instructions = amplifier_software.clone();
    run_program(instructions, &mut stdin_d, &mut stdin_e);
    // run amp e
    let instructions = amplifier_software.clone();
    run_program(instructions, &mut stdin_e, &mut stdin_a);
    stdin_a.remove(0)
}

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day07.txt")?;
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
    let instructions = input_state.clone();
    let pt_1_max = find_max_signal(instructions);
    dbg!(pt_1_max);
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
    stdin: &mut Vec<i32>,
    stdout: &mut Vec<i32>,
) -> (usize, Vec<i32>) {
    use Opcode::*;
    let instruction = parse_opcode(program[position]);
    let (new_pos, new_state) = match instruction {
        Add(arg1, arg2, _arg3) => {
            let (left, right, destination_pos) = (
                get_val(position + 1, arg1, &program),
                get_val(position + 2, arg2, &program),
                get_val(position + 3, ParameterMode::Immediate, &program) as usize,
            );
            program[destination_pos] = left + right;
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
            let the_data = stdin.remove(0);
            let address = program[wrap_pos(position + 1, program.len() - 1) as usize] as usize;
            program[address] = the_data;
            (position + 2, program)
        }
        ReturnInput => {
            let address = program[wrap_pos(position + 1, program.len() - 1) as usize] as usize;
            let the_data = program[address];
            stdout.push(the_data);
            (position + 2, program)
        }
        JumpIfTrue(arg1, arg2) => {
            let (a, b) = (
                get_val(position + 1, arg1, &program),
                get_val(position + 2, arg2, &program),
            );
            if a != 0 {
                (b as usize, program)
            } else {
                (position + 3, program)
            }
        }
        JumpIfFalse(arg1, arg2) => {
            let (a, b) = (
                get_val(position + 1, arg1, &program),
                get_val(position + 2, arg2, &program),
            );
            if a == 0 {
                (b as usize, program)
            } else {
                (position + 3, program)
            }
        }
        LessThan(arg1, arg2, _arg3) => {
            let (left, right, destination_pos) = (
                get_val(position + 1, arg1, &program),
                get_val(position + 2, arg2, &program),
                get_val(position + 3, ParameterMode::Immediate, &program) as usize,
            );
            if left < right {
                program[destination_pos] = 1;
            } else {
                program[destination_pos] = 0;
            }
            (position + 4, program)
        }
        Equals(arg1, arg2, _arg3) => {
            let (left, right, destination_pos) = (
                get_val(position + 1, arg1, &program),
                get_val(position + 2, arg2, &program),
                get_val(position + 3, ParameterMode::Immediate, &program) as usize,
            );
            if left == right {
                program[destination_pos] = 1;
            } else {
                program[destination_pos] = 0;
            }
            (position + 4, program)
        }
        Halt => {
            println!("got a stop");
            (position + 4, program)
        }
    };
    (wrap_pos(new_pos, new_state.len()), new_state)
}

pub fn run_program(
    mut program: Vec<i32>,
    mut stdin: &mut Vec<i32>,
    mut stdout: &mut Vec<i32>,
) -> Vec<i32> {
    let mut counter = 0;
    let mut position = 0;
    loop {
        let peek_instr = program[position as usize];
        if peek_instr == 99 {
            return program;
        } else if counter > 1000 {
            panic!("infinite loop?");
        } else {
            let (i, s) = step_forward(position, program, &mut stdin, &mut stdout);
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
        let (next_position, next_program) =
            step_forward(position, program, &mut vec![0], &mut vec![0]);
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
        let (next_position, next_program) =
            step_forward(position, program, &mut vec![0], &mut vec![0]);
        assert_eq!(next_position, 4);
        assert_eq!(next_program, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn step_2() {
        let program = vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let position = 4;
        let (next_position, next_program) =
            step_forward(position, program, &mut vec![0], &mut vec![0]);
        assert_eq!(next_position, 8);
        assert_eq!(
            next_program,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_run() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let next_program = run_program(program, &mut vec![0], &mut vec![0]);
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

    #[test]
    fn test_input_consume_removes_item() {
        let f = fs::read_to_string("input/day05.txt").unwrap();
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
        let instructions = input_state.clone();
        let mut stdin = vec![1];
        let mut stdout = vec![];
        let _final_state = run_program(instructions, &mut stdin, &mut stdout);
        assert_eq!(stdin, vec![]);
        // !!!! super weird!!!!
        assert_eq!(stdout[stdout.len() - 1], 12440243);
    }

    #[test]
    fn test_input_consume_removes_item2() {
        let f = fs::read_to_string("input/day05.txt").unwrap();
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
        let instructions = input_state.clone();
        let mut stdin = vec![5];
        let mut stdout = vec![];
        let _final_state = run_program(instructions, &mut stdin, &mut stdout);
        assert_eq!(stdin, vec![]);
        assert_eq!(stdout, vec![15486302]);
    }

    #[test]
    fn test_amplifier_checker_example1() {
        let input_state: Vec<i32> = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let input_config: Vec<i32> = vec![4, 3, 2, 1, 0];
        let instructions = input_state.clone();
        let signal = get_amplifier_signal_part1(&instructions, input_config);
        assert_eq!(signal, 43210);
    }

    fn _test_amplifier_checker_example1_is_max() {
        // need to figure out combinatorics
        let input_state: Vec<i32> = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let input_config: Vec<i32> = vec![4, 4, 4, 4, 4];
        let instructions = input_state.clone();
        let signal = get_amplifier_signal_part1(&instructions, input_config);
        assert_eq!(signal, 43210);
    }

    #[test]
    fn test_amplifier_checker_example2() {
        let input_state: Vec<i32> = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let input_config: Vec<i32> = vec![0, 1, 2, 3, 4];
        let instructions = input_state.clone();
        let signal = get_amplifier_signal_part1(&instructions, input_config);
        assert_eq!(signal, 54321);
    }

    #[test]
    fn test_amplifier_checker_example2_probably_max() {
        let input_state: Vec<i32> = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let input_config: Vec<i32> = vec![0, 1, 2, 4, 3];
        let instructions = input_state.clone();
        let signal = get_amplifier_signal_part1(&instructions, input_config);
        assert_eq!(signal < 54321, true);
    }

    #[test]
    fn test_find_max_of_example2() {
        let input_state: Vec<i32> = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let instructions = input_state.clone();
        let signal = find_max_signal(instructions);
        assert_eq!(signal, 54321);
    }

    #[test]
    fn test_find_max_of_example3() {
        let input_state: Vec<i32> = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let instructions = input_state.clone();
        let signal = find_max_signal(instructions);
        assert_eq!(signal, 65210);
    }

    #[test]
    fn input_combo_works() {
        let i = input_combinations_part1();
        assert_eq!(i.len(), 120);
    }

    #[test]
    fn test_try_thing() {
        let input_state: Vec<i32> = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let input_config = vec![9, 8, 7, 6, 5];
        let instructions = input_state.clone();
        let signal = get_amplifier_signal_part2(&instructions, input_config);
        assert_eq!(signal, 65210);
    }
}
