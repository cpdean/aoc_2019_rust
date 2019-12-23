use std::collections::HashMap;
/// day 13, arcade intcodecomputer
use std::fs;

pub struct IntCodeComputer {
    instruction_pointer: usize,
    memory: HashMap<usize, i64>,
    relative_base: i64,
    clock_counter: usize,
}

impl IntCodeComputer {
    pub fn into_memory(instructions: Vec<i64>) -> HashMap<usize, i64> {
        let mut mem = HashMap::new();
        for i in 0..instructions.len() {
            mem.insert(i, instructions[i]);
        }
        mem
    }
    pub fn new(instructions: Vec<i64>) -> IntCodeComputer {
        IntCodeComputer {
            instruction_pointer: 0,
            memory: IntCodeComputer::into_memory(instructions),
            relative_base: 0,
            clock_counter: 0,
        }
    }

    pub fn run_program_interruptable(
        &mut self,
        mut stdin: &mut Vec<i64>,
        mut stdout: &mut Vec<i64>,
    ) -> InterruptState {
        loop {
            let peek_instr = self.memory.get(&self.instruction_pointer).unwrap();
            if *peek_instr == 99 {
                return InterruptState::Halted;
            } else if self.clock_counter > 10_000_000 {
                panic!("infinite loop?");
            } else {
                let interrupt = self.step_forward(&mut stdin, &mut stdout);
                if interrupt == InterruptState::Blocked {
                    return interrupt;
                }
                self.clock_counter += 1;
            }
        }
    }

    pub fn step_forward(&mut self, stdin: &mut Vec<i64>, stdout: &mut Vec<i64>) -> InterruptState {
        use Opcode::*;
        let instruction = parse_opcode(*self.memory.get(&self.instruction_pointer).unwrap());
        match instruction {
            Add(arg1, arg2, arg3) => {
                let (left, right, destination_pos) = (
                    self.get_val(self.instruction_pointer + 1, arg1, false),
                    self.get_val(self.instruction_pointer + 2, arg2, false),
                    self.get_val(self.instruction_pointer + 3, arg3, true) as usize,
                );
                self.memory.insert(destination_pos, left + right);
                self.instruction_pointer += 4;
                InterruptState::Running
            }
            Mult(arg1, arg2, _arg3) => {
                let (left, right, destination_pos) = (
                    self.get_val(self.instruction_pointer + 1, arg1, false),
                    self.get_val(self.instruction_pointer + 2, arg2, false),
                    self.get_val(self.instruction_pointer + 3, _arg3, true) as usize,
                );
                self.memory.insert(destination_pos, left * right);
                self.instruction_pointer += 4;
                InterruptState::Running
            }
            TakeInput(arg1) => {
                if stdin.len() == 0 {
                    return InterruptState::Blocked;
                } else {
                    let the_data = stdin.remove(0);
                    let address = self.get_val(self.instruction_pointer + 1, arg1, true) as usize;
                    self.memory.insert(address, the_data);
                    self.instruction_pointer += 2;
                    InterruptState::Running
                }
            }
            ReturnInput(return_input_mode) => {
                //let address = program[wrap_pos(self.instruction_pointer + 1, program.len() - 1) as usize] as usize;
                //let the_data = program[address];
                let the_data = self.get_val(self.instruction_pointer + 1, return_input_mode, false);
                stdout.push(the_data);
                self.instruction_pointer += 2;
                InterruptState::Running
            }
            JumpIfTrue(arg1, arg2) => {
                let (a, b) = (
                    self.get_val(self.instruction_pointer + 1, arg1, false),
                    self.get_val(self.instruction_pointer + 2, arg2, false),
                );
                if a != 0 {
                    self.instruction_pointer = b as usize;
                    return InterruptState::Running;
                } else {
                    self.instruction_pointer += 3;
                    return InterruptState::Running;
                }
            }
            JumpIfFalse(arg1, arg2) => {
                let (a, b) = (
                    self.get_val(self.instruction_pointer + 1, arg1, false),
                    self.get_val(self.instruction_pointer + 2, arg2, false),
                );
                if a == 0 {
                    self.instruction_pointer = b as usize;
                    return InterruptState::Running;
                } else {
                    self.instruction_pointer += 3;
                    return InterruptState::Running;
                }
            }
            LessThan(arg1, arg2, _arg3) => {
                let (left, right, destination_pos) = (
                    self.get_val(self.instruction_pointer + 1, arg1, false),
                    self.get_val(self.instruction_pointer + 2, arg2, false),
                    self.get_val(self.instruction_pointer + 3, _arg3, true) as usize,
                );
                if left < right {
                    self.memory.insert(destination_pos, 1);
                } else {
                    self.memory.insert(destination_pos, 0);
                }
                self.instruction_pointer += 4;
                InterruptState::Running
            }
            Equals(arg1, arg2, _arg3) => {
                let (left, right, destination_pos) = (
                    self.get_val(self.instruction_pointer + 1, arg1, false),
                    self.get_val(self.instruction_pointer + 2, arg2, false),
                    self.get_val(self.instruction_pointer + 3, _arg3, true) as usize,
                );
                if left == right {
                    self.memory.insert(destination_pos, 1);
                } else {
                    self.memory.insert(destination_pos, 0);
                }
                self.instruction_pointer += 4;
                InterruptState::Running
            }
            AdjustRelativeBase(a) => {
                let base_adjustment = self.get_val(self.instruction_pointer + 1, a, false);
                self.relative_base += base_adjustment;
                self.instruction_pointer += 2;
                InterruptState::Running
            }
            Halt => {
                println!("got a stop");
                InterruptState::Halted
            }
        }
    }

    fn memory_get(&self, position: &usize) -> i64 {
        match self.memory.get(position) {
            Some(x) => *x,
            None => 0,
        }
    }

    fn get_val(&self, position: usize, _mode: ParameterMode, dont_deref: bool) -> i64 {
        let a = position;
        if dont_deref {
            match _mode {
                ParameterMode::Immediate => self.memory_get(&a), // eh
                ParameterMode::Position => *self.memory.get(&a).unwrap(),
                ParameterMode::Relative => self.relative_base + *self.memory.get(&a).unwrap(),
            }
        } else {
            match _mode {
                ParameterMode::Immediate => *self.memory.get(&a).unwrap(),
                ParameterMode::Position => {
                    self.memory_get(&(*self.memory.get(&a).unwrap() as usize))
                }
                ParameterMode::Relative => *self
                    .memory
                    .get(&((self.relative_base + *self.memory.get(&a).unwrap()) as usize))
                    .unwrap(),
            }
        }
    }
}

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

/*
fn canvas_get(c: &HashMap<(i32, i32), CellType>, coord: &(i32, i32)) -> bool {
    if c.contains_key(coord) {
        *c.get(coord).unwrap()
    } else {
        false
    }
}
*/

/*
fn show_canvas(c: &HashMap<(i32, i32), CellType>) {
    let (min_width, max_width) = (
        c.keys().map(|(x, _)| x).min().unwrap(),
        c.keys().map(|(x, _)| x).max().unwrap(),
    );
    let (min_height, max_height) = (
        c.keys().map(|(_, y)| y).min().unwrap(),
        c.keys().map(|(_, y)| y).max().unwrap(),
    );
    for y in *min_height..*max_height + 1 {
        let mut line = vec![];
        for x in *min_width..*max_width + 1 {
            line.push(if canvas_get(c, &(x, y)) { "#" } else { "_" });
        }
        println!("{}", line.join(""));
    }
    //dbg!((min_width, max_width));
    //dbg!((min_height, max_height));
}
*/

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn turn(direction: &Direction, command: &i64) -> Direction {
    use Direction::*;
    match command {
        // left
        0 => match direction {
            North => West,
            West => South,
            South => East,
            East => North,
        },
        // right
        1 => match direction {
            North => East,
            East => South,
            South => West,
            West => North,
        },
        _ => panic!("bad turn command {}", command),
    }
}

fn travel(coord: &(i32, i32), d: &Direction) -> (i32, i32) {
    use Direction::*;
    let velocity = match d {
        North => (0, -1),
        East => (1, 0),
        South => (0, 1),
        West => (-1, 0),
    };
    let (v_x, v_y) = velocity;
    let (x, y) = coord;
    (x + v_x, y + v_y)
}

#[derive(PartialEq, Debug, Clone)]
enum CellType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
    TheScore(i64),
}

fn get_raw_screen(stdout: &mut Vec<i64>) -> Vec<((i64, i64), CellType)> {
    let mut out = vec![];
    while stdout.len() > 0 {
        let (x, y, t) = (stdout.remove(0), stdout.remove(0), stdout.remove(0));
        let cell_type = match t {
            0 => CellType::Empty,
            1 => CellType::Wall,
            2 => CellType::Block,
            3 => CellType::Paddle,
            4 => CellType::Ball,
            x => CellType::TheScore(x),
        };
        out.push(((x, y), cell_type));
    }
    out
}

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day13.txt")?;
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
    let mut canvas: HashMap<(i64, i64), CellType> = HashMap::new();
    let mut frame = 0;
    let instructions = input_state.clone();
    let mut computer = IntCodeComputer::new(instructions);
    // now with quarters
    computer.memory.insert(0, 2);
    let mut stdin = vec![0];
    let mut stdout = vec![];
    let mut halt_state = InterruptState::Running;
    while halt_state != InterruptState::Halted {
        halt_state = computer.run_program_interruptable(&mut stdin, &mut stdout);
        //dbg!(&halt_state);
        /*
        if frame == 0 {
            let part1 = get_raw_screen(&mut stdout)
                .iter()
                .filter(|((x, y), t)| *t == CellType::Block)
                .count();
            dbg!(part1);
        }
        */
        let the_screen = get_raw_screen(&mut stdout);
        let mut b: Vec<((i64, i64), CellType)> = the_screen
            .iter()
            .filter(|((x, y), cell_type)| cell_type == &CellType::Ball)
            .map(|e| e.clone())
            .collect();

        if dbg!(&b).len() > 0 {
            let ((ball_x, ball_y), _) = b.remove(0);

            let mut p: Vec<((i64, i64), CellType)> = the_screen
                .iter()
                .filter(|((x, y), cell_type)| cell_type == &CellType::Paddle)
                .map(|e| e.clone())
                .collect();
            let ((paddle_x, paddle_y), _) = p.remove(0);

            if ball_x < paddle_x {
                stdin.push(-1);
            } else if ball_x > paddle_x {
                stdin.push(1);
            } else {
                stdin.push(0);
            }
        }

        let s: Vec<((i64, i64), CellType)> = the_screen
            .iter()
            .filter(|((x, y), cell_type)| match cell_type {
                CellType::TheScore(x) => true,
                anything_else => false,
            })
            .map(|e| e.clone())
            .collect();
        dbg!(s);
        if dbg!(frame) > 300 {
            dbg!(the_screen);
        }

        frame += 1;
    }
    //dbg!(&canvas);
    //show_canvas(&canvas);

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
) -> Result<Vec<i64>, String> {
    let mut counter = 0;
    let mut position = 0;
    let mut relative_base = 0;
    // lol
    for _ in 0..10 {
        // shoutout to polina
        program.push(0);
    }
    loop {
        let peek_instr = program[position as usize];
        if peek_instr == 99 {
            return Ok(program);
        } else if counter > 100_000_000 {
            return Err("infinite loop?".to_string());
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
        } else if counter > 10_000 {
            panic!("infinite loop?");
        } else {
            let (interrupt, pos, _next_base, p) =
                step_forward(position, program.clone(), 0, &mut stdin, &mut stdout);
            //dbg!(&(&interrupt, pos, _next_base));
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
        let mut computer = IntCodeComputer::new(program.clone());
        computer.step_forward(&mut vec![0], &mut vec![0]);
        assert_eq!(computer.instruction_pointer, 4);
        assert_eq!(
            computer.memory,
            IntCodeComputer::into_memory(vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50])
        );
    }

    #[test]
    fn it_works2() {
        let program = vec![1101, 100, -1, 4, 0];
        let mut computer = IntCodeComputer::new(program.clone());
        computer.step_forward(&mut vec![0], &mut vec![0]);
        assert_eq!(computer.instruction_pointer, 4);
        assert_eq!(
            computer.memory,
            IntCodeComputer::into_memory(vec![1101, 100, -1, 4, 99])
        );
    }

    #[test]
    fn test_run() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut computer = IntCodeComputer::new(program.clone());
        computer.run_program_interruptable(&mut vec![0], &mut vec![0]);
        let mut copied = vec![];
        for i in 0..12 {
            copied.push(*computer.memory.get(&i).unwrap());
        }
        assert_eq!(copied, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
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
        let mut computer = IntCodeComputer::new(instructions.clone());
        computer.run_program_interruptable(&mut stdin, &mut stdout);
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
        let mut computer = IntCodeComputer::new(instructions.clone());
        computer.run_program_interruptable(&mut stdin, &mut stdout);
        assert_eq!(stdin, vec![]);
        assert_eq!(stdout, vec![15486302]);
    }

    #[test]
    fn test_try_position_take() {
        let input_state: Vec<i64> = vec![3, 3, 99, 0];
        let instructions = input_state.clone();
        let mut stdin = vec![7];
        let mut stdout = vec![];
        let halt_state = run_program(instructions, &mut stdin, &mut stdout).unwrap();
        assert_eq!(halt_state[3], 7);
    }

    #[test]
    fn test_try_example1() {
        let input_state: Vec<i64> = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let instructions = input_state.clone();
        let mut computer = IntCodeComputer::new(instructions.clone());
        let mut stdin = vec![];
        let mut stdout = vec![];
        computer.run_program_interruptable(&mut stdin, &mut stdout);
        // this is the quine example
        assert_eq!(stdout, input_state);
    }

    #[test]
    fn test_try_example2() {
        let input_state: Vec<i64> = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout).unwrap();
        assert_eq!(stdout[0], 1219070632396864); // probably right
    }

    #[test]
    fn test_try_position_return() {
        let input_state: Vec<i64> = vec![4, 2, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout).unwrap();
        assert_eq!(stdout[0], 99);
    }

    #[test]
    fn test_adjust_base() {
        let input_state: Vec<i64> = vec![109, 10, 204, -6, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout).unwrap();
        assert_eq!(stdout[0], 99);
    }

    #[test]
    fn test_try_immediate_return() {
        let input_state: Vec<i64> = vec![104, 2, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout).unwrap();
        assert_eq!(stdout[0], 2);
    }

    #[test]
    fn test_try_example3() {
        let input_state: Vec<i64> = vec![104, 1125899906842624, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout).unwrap();
        assert_eq!(stdout[0], 1125899906842624); // probably right
    }

    #[test]
    fn test_try_position_add() {
        let input_state: Vec<i64> = vec![9, 100, 1101, 1, 1, 7, 104, 0, 99];
        let instructions = input_state.clone();
        let mut computer = IntCodeComputer::new(instructions.clone());
        let mut stdin = vec![];
        let mut stdout = vec![];
        computer.run_program_interruptable(&mut stdin, &mut stdout);
        assert_eq!(stdout[0], 2); // probably right
    }

    #[test]
    fn test_try_relative_add() {
        let input_state: Vec<i64> = vec![9, 9, 21101, 1, 1, 14, 104, 0, 99, -7];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout).unwrap();
        assert_eq!(stdout[0], 2); // probably right
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_BOOST_said_203_was_bad_check_zero_relative() {
        let input_state: Vec<i64> = vec![9, 7, 203, 5, 104, 3, 99, 0];
        let instructions = input_state.clone();
        let mut stdin = vec![8];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout).unwrap();
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
        dbg!(&run_program(instructions, &mut stdin, &mut stdout).unwrap()[0..10]);
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
        dbg!(&run_program(instructions, &mut stdin, &mut stdout).unwrap()[0..10]);
        assert_eq!(stdout[0], 8);
    }

    #[test]
    fn test_try_relative_add_immediate() {
        let input_state: Vec<i64> = vec![109, -7, 21101, 1, 1, 14, 104, 0, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![];
        let mut stdout = vec![];
        run_program(instructions, &mut stdin, &mut stdout).unwrap();
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
        run_program(instructions, &mut stdin, &mut stdout).unwrap();
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
        dbg!(&run_program(instructions, &mut stdin, &mut stdout).unwrap()[0..10]);
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
        dbg!(&run_program(instructions, &mut stdin, &mut stdout).unwrap()[0..10]);
        assert_eq!(stdout[0], 8);
    }

    #[test]
    fn test_from_frank() {
        let input_state: Vec<i64> = vec![9, 1, 203, 4, 99];
        let instructions = input_state.clone();
        let mut stdin = vec![888];
        let mut stdout = vec![];
        let halt_state = run_program(instructions, &mut stdin, &mut stdout).unwrap();
        assert_eq!(halt_state[5], 888);
    }
}
