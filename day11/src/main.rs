/// day 9, relative mode IntCode computer
use std::fs;

//type Result<T> = ::std::result::Result<T, dyn std::error::Error>;

#[derive(Debug, PartialEq)]
pub enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug, PartialEq)]
pub enum InterruptState {
    Running,
    Blocked,
    Halted,
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    Mult(ParameterMode, ParameterMode, ParameterMode),
    Add(ParameterMode, ParameterMode, ParameterMode),
    TakeInput(ParameterMode),
    ReturnInput(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
    AdjustRelativeBase(ParameterMode),
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
    AdjustRelativeBase,
    Halt,
}

fn param_mode_of_arg(instruction: i64, arg_number: usize) -> ParameterMode {
    use ParameterMode::*;
    let offset = match arg_number {
        1 => 100,
        2 => 1000,
        3 => 10000,
        x => panic!("does not support arg number '{}' yet", x),
    };
    if instruction / offset % 10 == 0 {
        Position
    } else if instruction / offset % 10 == 1 {
        Immediate
    } else if instruction / offset % 10 == 2 {
        Relative
    } else {
        panic!(
            "failed on instruction {}, parsing param mode of arg number '{}'",
            instruction, arg_number
        );
    }
}

pub fn parse_opcode(instruction: i64) -> Opcode {
    use Opcode::*;
    let inst_class = match instruction % 100 {
        1 => InstructionClass::Add,
        2 => InstructionClass::Mult,
        3 => InstructionClass::TakeInput,
        4 => InstructionClass::ReturnInput,
        5 => InstructionClass::JumpIfTrue,
        6 => InstructionClass::JumpIfFalse,
        7 => InstructionClass::LessThan,
        8 => InstructionClass::Equals,
        9 => InstructionClass::AdjustRelativeBase,
        99 => InstructionClass::Halt,
        x => {
            panic!("got a {:?}, from {:?}", x, instruction);
        }
    };
    match inst_class {
        InstructionClass::Add => {
            let first_param = param_mode_of_arg(instruction, 1);
            let second_param = param_mode_of_arg(instruction, 2);
            let third_param = param_mode_of_arg(instruction, 3);
            Add(first_param, second_param, third_param)
        }
        InstructionClass::Mult => {
            let first_param = param_mode_of_arg(instruction, 1);
            let second_param = param_mode_of_arg(instruction, 2);
            let third_param = param_mode_of_arg(instruction, 3);
            Mult(first_param, second_param, third_param)
        }
        InstructionClass::Halt => Halt, // should be 99 but oh well
        InstructionClass::TakeInput => {
            let first_param = param_mode_of_arg(instruction, 1);
            TakeInput(first_param)
        }
        InstructionClass::ReturnInput => {
            let first_param = param_mode_of_arg(instruction, 1);
            ReturnInput(first_param)
        }
        InstructionClass::JumpIfTrue => {
            let first_param = param_mode_of_arg(instruction, 1);
            let second_param = param_mode_of_arg(instruction, 2);
            JumpIfTrue(first_param, second_param)
        }
        InstructionClass::JumpIfFalse => {
            let first_param = param_mode_of_arg(instruction, 1);
            let second_param = param_mode_of_arg(instruction, 2);
            JumpIfFalse(first_param, second_param)
        }
        InstructionClass::LessThan => {
            let first_param = param_mode_of_arg(instruction, 1);
            let second_param = param_mode_of_arg(instruction, 2);
            let third_param = param_mode_of_arg(instruction, 3);
            LessThan(first_param, second_param, third_param)
        }
        InstructionClass::Equals => {
            let first_param = param_mode_of_arg(instruction, 1);
            let second_param = param_mode_of_arg(instruction, 2);
            let third_param = param_mode_of_arg(instruction, 3);
            Equals(first_param, second_param, third_param)
        }
        InstructionClass::AdjustRelativeBase => {
            let first_param = param_mode_of_arg(instruction, 1);
            AdjustRelativeBase(first_param)
        }
    }
}

pub fn input_combinations_part2() -> Vec<Vec<i64>> {
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

pub fn input_combinations_part1() -> Vec<Vec<i64>> {
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

pub fn find_max_signal(amplifier_software: Vec<i64>) -> i64 {
    let mut m = 0;
    for input_config in input_combinations_part1() {
        let signal = get_amplifier_signal_part1(&amplifier_software, input_config);
        if m < signal {
            m = signal
        }
    }
    m
}

pub fn find_max_signal_part2(amplifier_software: Vec<i64>) -> i64 {
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
    amplifier_software: &Vec<i64>,
    mut input_config: Vec<i64>,
) -> i64 {
    // works by chaining several computers together, feeding each one an input plus a second
    // number you get from the previous computer

    // A->B->C->D->E-> {i64}
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
    amplifier_software: &Vec<i64>,
    mut input_config: Vec<i64>,
) -> i64 {
    // now the computers must run in a cycle, processing input from the computer in front of it
    // this is much harder because I have to re-do how halting works, implement blocking, and write
    // what is going to essentially be a scheduler to switch control flow to the next computer in
    // the ring

    // A->B->C->D->E->-+(when halts...)--> {i64}
    // ^               |
    // +-------<-------+
    //
    // additionally I cannot re-use the stdin/stdout pipes, i have to make a pipe for each
    // connection
    use std::cell::RefCell;

    let mut stdin_a = vec![];
    let mut stdin_b = vec![];
    let mut stdin_c = vec![];
    let mut stdin_d = vec![];
    let mut stdin_e = vec![];
    // load initial params
    stdin_a.push(input_config.remove(0));
    stdin_a.push(0); // amp A has an extra thing added to it
    stdin_b.push(input_config.remove(0));
    stdin_c.push(input_config.remove(0));
    stdin_d.push(input_config.remove(0));
    stdin_e.push(input_config.remove(0));

    let stdin_a_ptr = RefCell::new(stdin_a);
    let stdin_b_ptr = RefCell::new(stdin_b);
    let stdin_c_ptr = RefCell::new(stdin_c);
    let stdin_d_ptr = RefCell::new(stdin_d);
    let stdin_e_ptr = RefCell::new(stdin_e);
    let mut schedule_cycle = vec![
        (
            (InterruptState::Running, "A"),
            amplifier_software.clone(),
            0,
            &stdin_a_ptr,
            &stdin_b_ptr,
        ),
        (
            (InterruptState::Running, "B"),
            amplifier_software.clone(),
            0,
            &stdin_b_ptr,
            &stdin_c_ptr,
        ),
        (
            (InterruptState::Running, "C"),
            amplifier_software.clone(),
            0,
            &stdin_c_ptr,
            &stdin_d_ptr,
        ),
        (
            (InterruptState::Running, "D"),
            amplifier_software.clone(),
            0,
            &stdin_d_ptr,
            &stdin_e_ptr,
        ),
        (
            (InterruptState::Running, "E"),
            amplifier_software.clone(),
            0,
            &stdin_e_ptr,
            &stdin_a_ptr,
        ),
    ];

    let mut global_clock = 0;
    loop {
        if schedule_cycle
            .iter()
            .all(|((interrupt_state, _), _, _, _, _)| *interrupt_state == InterruptState::Halted)
        {
            break;
        } else {
            let _status: Vec<(&InterruptState, &&str, &&RefCell<Vec<i64>>)> = schedule_cycle
                .iter()
                .map(|((i, n), _, _, _, out)| (i, n, out))
                .collect();
        }
        let ((_interrupt_state, n), p, ix, pipe1, pipe2) = schedule_cycle.remove(0);
        let (int, next_ix, next_p) =
            run_program_interruptable(p, ix, &mut pipe1.borrow_mut(), &mut pipe2.borrow_mut());
        schedule_cycle.push(((int, n), next_p, next_ix, pipe1, pipe2));
        if global_clock < 2000 {
            let _status: Vec<(&InterruptState, &&str, &&RefCell<Vec<i64>>)> = schedule_cycle
                .iter()
                .map(|((i, n), _, _, _, out)| (i, n, out))
                .collect();
            global_clock += 1;
        } else {
            break;
        }
    }
    return stdin_a_ptr.borrow()[0];
}

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day09.txt")?;
    let input_state: Vec<i64> = f
        .trim()
        .split(",")
        .map(|e| {
            let i: i64 = match e.parse() {
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
    dbg!(stdout);
    let instructions = input_state.clone();
    let mut stdin = vec![2];
    let mut stdout = vec![];
    let _final_state = run_program(instructions, &mut stdin, &mut stdout);
    dbg!(stdout);
    Ok(())
}

fn wrap_pos(try_pos: usize, length: usize) -> usize {
    if try_pos > length {
        try_pos - length
    } else {
        try_pos
    }
}

fn get_val(
    position: usize,
    relative_base: i64,
    _mode: ParameterMode,
    program: &Vec<i64>,
    dont_deref: bool,
) -> i64 {
    let a = wrap_pos(position, program.len() - 1);
    if dont_deref {
        match _mode {
            ParameterMode::Immediate => program[a], // eh
            ParameterMode::Position => program[a],
            ParameterMode::Relative => relative_base + program[a],
        }
    } else {
        match _mode {
            ParameterMode::Immediate => program[a],
            ParameterMode::Position => program[program[a] as usize],
            ParameterMode::Relative => program[(relative_base + program[a]) as usize],
        }
    }
}

type ProcessState = (InterruptState, usize, i64, Vec<i64>); // (int, pos, rel, memory)

pub fn step_forward(
    position: usize,
    mut program: Vec<i64>,
    relative_base: i64,
    stdin: &mut Vec<i64>,
    stdout: &mut Vec<i64>,
) -> ProcessState {
    use Opcode::*;
    let instruction = parse_opcode(program[position]);
    let mut interrupt = InterruptState::Running;
    let (new_pos, new_rel_base, new_state) = match instruction {
        Add(arg1, arg2, arg3) => {
            let (left, right, destination_pos) = (
                get_val(position + 1, relative_base, arg1, &program, false),
                get_val(position + 2, relative_base, arg2, &program, false),
                get_val(position + 3, relative_base, arg3, &program, true) as usize,
            );
            program[destination_pos] = left + right;
            (position + 4, relative_base, program)
        }
        Mult(arg1, arg2, _arg3) => {
            let (left, right, destination_pos) = (
                get_val(position + 1, relative_base, arg1, &program, false),
                get_val(position + 2, relative_base, arg2, &program, false),
                get_val(position + 3, relative_base, _arg3, &program, true) as usize,
            );
            program[destination_pos] = left * right;
            (position + 4, relative_base, program)
        }
        TakeInput(arg1) => {
            if stdin.len() == 0 {
                interrupt = InterruptState::Blocked;
                (position, relative_base, program)
            } else {
                let the_data = stdin.remove(0);
                let address = get_val(position + 1, relative_base, arg1, &program, true) as usize;
                program[address] = the_data;
                (position + 2, relative_base, program)
            }
        }
        ReturnInput(return_input_mode) => {
            //let address = program[wrap_pos(position + 1, program.len() - 1) as usize] as usize;
            //let the_data = program[address];
            let the_data = get_val(
                position + 1,
                relative_base,
                return_input_mode,
                &program,
                false,
            );
            stdout.push(the_data);
            (position + 2, relative_base, program)
        }
        JumpIfTrue(arg1, arg2) => {
            let (a, b) = (
                get_val(position + 1, relative_base, arg1, &program, false),
                get_val(position + 2, relative_base, arg2, &program, false),
            );
            if a != 0 {
                (b as usize, relative_base, program)
            } else {
                (position + 3, relative_base, program)
            }
        }
        JumpIfFalse(arg1, arg2) => {
            let (a, b) = (
                get_val(position + 1, relative_base, arg1, &program, false),
                get_val(position + 2, relative_base, arg2, &program, false),
            );
            if a == 0 {
                (b as usize, relative_base, program)
            } else {
                (position + 3, relative_base, program)
            }
        }
        LessThan(arg1, arg2, _arg3) => {
            let (left, right, destination_pos) = (
                get_val(position + 1, relative_base, arg1, &program, false),
                get_val(position + 2, relative_base, arg2, &program, false),
                get_val(position + 3, relative_base, _arg3, &program, true) as usize,
            );
            if left < right {
                program[destination_pos] = 1;
            } else {
                program[destination_pos] = 0;
            }
            (position + 4, relative_base, program)
        }
        Equals(arg1, arg2, _arg3) => {
            let (left, right, destination_pos) = (
                get_val(position + 1, relative_base, arg1, &program, false),
                get_val(position + 2, relative_base, arg2, &program, false),
                get_val(position + 3, relative_base, _arg3, &program, true) as usize,
            );
            if left == right {
                program[destination_pos] = 1;
            } else {
                program[destination_pos] = 0;
            }
            (position + 4, relative_base, program)
        }
        AdjustRelativeBase(a) => {
            let base_adjustment = get_val(position + 1, relative_base, a, &program, false);
            (position + 2, relative_base + base_adjustment, program)
        }
        Halt => {
            println!("got a stop");
            interrupt = InterruptState::Halted;
            (position + 4, relative_base, program)
        }
    };
    (
        interrupt,
        wrap_pos(new_pos, new_state.len()),
        new_rel_base,
        new_state,
    )
}

pub fn run_program(
    mut program: Vec<i64>,
    mut stdin: &mut Vec<i64>,
    mut stdout: &mut Vec<i64>,
) -> Vec<i64> {
    let mut counter = 0;
    let mut position = 0;
    let mut relative_base = 0;
    // lol
    for _ in 0..10000 {
        // shoutout to polina
        program.push(0);
    }
    loop {
        let peek_instr = program[position as usize];
        if peek_instr == 99 {
            return program;
        } else if counter > 1000000 {
            panic!("infinite loop?");
        } else {
            let (_, i, r, s) =
                step_forward(position, program, relative_base, &mut stdin, &mut stdout);
            position = i;
            relative_base = r;
            program = s;
            counter += 1;
        }
    }
}

pub fn run_program_interruptable(
    mut program: Vec<i64>,
    mut position: usize,
    mut stdin: &mut Vec<i64>,
    mut stdout: &mut Vec<i64>,
) -> (InterruptState, usize, Vec<i64>) {
    let mut counter = 0;
    loop {
        let peek_instr = program[position as usize];
        if peek_instr == 99 {
            return (InterruptState::Halted, position, program);
        } else if counter > 1000 {
            panic!("infinite loop?");
        } else {
            let (interrupt, pos, _next_base, p) =
                step_forward(position, program.clone(), 0, &mut stdin, &mut stdout);
            if interrupt == InterruptState::Blocked {
                return (interrupt, pos, p);
            }
            position = pos;
            program = p;
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
        let (_, next_position, _next_base, next_program) =
            step_forward(position, program, 0, &mut vec![0], &mut vec![0]);
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
        let (_, next_position, _next_base, next_program) =
            step_forward(position, program, 0, &mut vec![0], &mut vec![0]);
        assert_eq!(next_position, 4);
        assert_eq!(next_program, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn step_2() {
        let program = vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        let position = 4;
        let (_, next_position, _next_base, next_program) =
            step_forward(position, program, 0, &mut vec![0], &mut vec![0]);
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
            next_program[0..12].to_vec(),
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
    fn test_mult_parse_param_relative_mode() {
        // this is actually broken, you can't have an instr param be immediate....
        let i = parse_opcode(12002);
        assert_eq!(
            i,
            Opcode::Mult(
                ParameterMode::Position,
                ParameterMode::Relative,
                ParameterMode::Immediate
            )
        );
    }

    #[test]
    fn test_input_consume_removes_item() {
        let f = fs::read_to_string("../input/day05.txt").unwrap();
        let input_state: Vec<i64> = f
            .trim()
            .split(",")
            .map(|e| {
                let i: i64 = match e.parse() {
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
        let f = fs::read_to_string("../input/day05.txt").unwrap();
        let input_state: Vec<i64> = f
            .trim()
            .split(",")
            .map(|e| {
                let i: i64 = match e.parse() {
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
        let input_state: Vec<i64> = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let input_config: Vec<i64> = vec![4, 3, 2, 1, 0];
        let instructions = input_state.clone();
        let signal = get_amplifier_signal_part1(&instructions, input_config);
        assert_eq!(signal, 43210);
    }

    #[test]
    fn test_amplifier_checker_example2() {
        let input_state: Vec<i64> = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let input_config: Vec<i64> = vec![0, 1, 2, 3, 4];
        let instructions = input_state.clone();
        let signal = get_amplifier_signal_part1(&instructions, input_config);
        assert_eq!(signal, 54321);
    }

    #[test]
    fn test_amplifier_checker_example2_probably_max() {
        let input_state: Vec<i64> = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let input_config: Vec<i64> = vec![0, 1, 2, 4, 3];
        let instructions = input_state.clone();
        let signal = get_amplifier_signal_part1(&instructions, input_config);
        assert_eq!(signal < 54321, true);
    }

    #[test]
    fn test_find_max_of_example3() {
        let input_state: Vec<i64> = vec![
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
    fn test_try_position_take() {
        let input_state: Vec<i64> = vec![3, 3, 99, 0];
        let instructions = input_state.clone();
        let mut stdin = vec![7];
        let mut stdout = vec![];
        let halt_state = run_program(instructions, &mut stdin, &mut stdout);
        assert_eq!(halt_state[3], 7);
    }

    #[test]
    fn test_try_example1() {
        let input_state: Vec<i64> = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout);
        // this is the quine example
        assert_eq!(stdout, input_state);
    }

    #[test]
    fn test_try_example2() {
        let input_state: Vec<i64> = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout);
        assert_eq!(stdout[0], 1219070632396864); // probably right
    }

    #[test]
    fn test_try_position_return() {
        let input_state: Vec<i64> = vec![4, 2, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout);
        assert_eq!(stdout[0], 99);
    }

    #[test]
    fn test_adjust_base() {
        let input_state: Vec<i64> = vec![109, 10, 204, -6, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout);
        assert_eq!(stdout[0], 99);
    }

    #[test]
    fn test_try_immediate_return() {
        let input_state: Vec<i64> = vec![104, 2, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout);
        assert_eq!(stdout[0], 2);
    }

    #[test]
    fn test_try_example3() {
        let input_state: Vec<i64> = vec![104, 1125899906842624, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout);
        assert_eq!(stdout[0], 1125899906842624); // probably right
    }

    #[test]
    fn test_try_position_add() {
        let input_state: Vec<i64> = vec![9, 100, 1101, 1, 1, 7, 104, 0, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout);
        assert_eq!(stdout[0], 2); // probably right
    }

    #[test]
    fn test_try_relative_add() {
        let input_state: Vec<i64> = vec![9, 9, 21101, 1, 1, 14, 104, 0, 99, -7];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout);
        assert_eq!(stdout[0], 2); // probably right
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_BOOST_said_203_was_bad_check_zero_relative() {
        let input_state: Vec<i64> = vec![9, 7, 203, 5, 104, 3, 99, 0];
        let instructions = input_state.clone();
        let mut stdin = vec![8];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout);
        assert_eq!(stdout[0], 8);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_BOOST_said_203_was_bad_check_negative_relative() {
        // 203 means, take input and put it at `ix` where `ix` is adjusted
        let input_state: Vec<i64> = vec![9, 7, 203, 9, 104, 3, 99, -4];
        let instructions = input_state.clone();
        let mut stdin = vec![8];
        let mut stdout = vec![];
        dbg!(&run_program(instructions, &mut stdin, &mut stdout)[0..10]);
        assert_eq!(stdout[0], 8);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_BOOST_said_203_was_bad_check_zero_param() {
        // 203 means, take input and put it at `ix` where `ix` is adjusted
        let input_state: Vec<i64> = vec![9, 7, 3, 5, 104, 3, 99, -5];
        let instructions = input_state.clone();
        let mut stdin = vec![8];
        let mut stdout = vec![];
        dbg!(&run_program(instructions, &mut stdin, &mut stdout)[0..10]);
        assert_eq!(stdout[0], 8);
    }

    #[test]
    fn test_try_relative_add_immediate() {
        let input_state: Vec<i64> = vec![109, -7, 21101, 1, 1, 14, 104, 0, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout);
        assert_eq!(stdout[0], 2); // probably right
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_BOOST_said_203_was_bad_check_zero_relative_immediate() {
        // 203 means, take input and put it at `ix` where `ix` is adjusted
        let input_state: Vec<i64> = vec![109, 0, 203, 5, 104, 3, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![8];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout);
        assert_eq!(stdout[0], 8);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_BOOST_said_203_was_bad_check_negative_relative_immediate() {
        // 203 means, take input and put it at `ix` where `ix` is adjusted
        let input_state: Vec<i64> = vec![109, -4, 203, 9, 104, 3, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![8];
        let mut stdout = vec![];
        dbg!(&run_program(instructions, &mut stdin, &mut stdout)[0..10]);
        assert_eq!(stdout[0], 8);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_BOOST_said_203_was_bad_check_zero_param_immediate() {
        // 203 means, take input and put it at `ix` where `ix` is adjusted
        let input_state: Vec<i64> = vec![109, -5, 3, 5, 104, 3, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![8];
        let mut stdout = vec![];
        dbg!(&run_program(instructions, &mut stdin, &mut stdout)[0..10]);
        assert_eq!(stdout[0], 8);
    }

    #[test]
    fn test_from_frank() {
        let input_state: Vec<i64> = vec![9, 1, 203, 4, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![888];
        let mut stdout = vec![];
        let halt_state = run_program(instructions, &mut stdin, &mut stdout);
        assert_eq!(halt_state[5], 888);
    }
}
