use memmap2::Mmap;
use std::{fmt, fs::File, io::{BufRead, BufReader}};

const SAMPLE_SIZE: u16 = 1000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input25.txt")?;

    let program = parse_instructions(&lines);

    for init_a in 0.. {
        // Clone the program
        let mut program1 = program.clone();

        // Output closure
        let output = |state: &mut State, value: MachineInt| {
            // Set next signal state
            state.signal_state = match state.signal_state {
                SignalState::Latch => {
                    if value == 1 || value == 0 {
                        SignalState::Good(1, value)
                    } else {
                        SignalState::Bad
                    }
                }
                SignalState::Good(num, last) => {
                    if (last == 1 && value == 0) || (last == 0 && value == 1) {
                        if num == SAMPLE_SIZE {
                            SignalState::Perfect
                        } else {
                            SignalState::Good(num + 1, value)
                        }
                    } else {
                        SignalState::Bad
                    }
                }
                _ => panic!("Unexpected state")
            };
        };

        // Set up state
        let mut state: State = State::new(&output);

        // Set register 'a'
        state.reg[0] = init_a;

        // Run the program
        while state.pc >= 0 && (state.pc as usize) < program.len() {
            step(&mut state, &mut program1);

            match state.signal_state {
                SignalState::Bad | SignalState::Perfect => break,
                _ => {}
            }
        }

        // Success?
        if let SignalState::Perfect = state.signal_state {
            println!("Value of register 'a' to generate clock signal: {}", init_a);
            break
        }
    }

    Ok(())
}

enum SignalState {
    Latch,
    Good(u16, MachineInt),
    Bad,
    Perfect
}

type MachineInt = i32;

struct State<'a> {
    reg: [MachineInt; 4],
    pc: MachineInt,
    output: &'a dyn Fn(&mut State, MachineInt),
    signal_state: SignalState
}

impl<'a> State<'a> {
    fn new(output: &'a dyn Fn(&mut State, MachineInt)) -> State {
        State {
            reg: [0; 4],
            pc: 0,
            output,
            signal_state: SignalState::Latch
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Cpy(RegImm, RegImm),
    Inc(RegImm),
    Dec(RegImm),
    Jnz(RegImm, RegImm),
    Tgl(RegImm),
    Out(RegImm),
}

#[derive(Clone, PartialEq, Eq)]
enum RegImm {
    Reg(u8),
    Imm(MachineInt)
}

impl RegImm {
    fn parse(string: &str) -> Option<RegImm> {
        if let Some(r) = parse_reg(string) {
            Some(RegImm::Reg(r))
        } else if let Ok(i) = string.parse::<MachineInt>() {
            Some(RegImm::Imm(i))
        } else {
            None
        }
    }

    fn get(&self, state: &State) -> MachineInt {
        match self {
            RegImm::Reg(r) => state.reg[*r as usize],
            RegImm::Imm(i) => *i
        }
    }
}

impl fmt::Debug for RegImm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegImm::Reg(r) => f.write_fmt(format_args!("{}", (*r + b'a') as char))?,
            RegImm::Imm(i) => f.write_fmt(format_args!("{}", *i))?
        }
        Ok(())
    }
}

type Program = Vec<Instruction>;

fn step(state: &mut State, program: &mut Program) {
    match &program[state.pc as usize] {
        Instruction::Cpy(ri1, ri2) => {
            match ri2 {
                RegImm::Reg(r) => state.reg[*r as usize] = ri1.get(state),
                RegImm::Imm(_) => {}
            }
        }
        Instruction::Inc(ri) => {
            match ri {
                RegImm::Reg(r) => state.reg[*r as usize] += 1,
                RegImm::Imm(_) => {}
            }
        }
        Instruction::Dec(ri) => {
            match ri {
                RegImm::Reg(r) => state.reg[*r as usize] -= 1,
                RegImm::Imm(_) => {}
            }
        }
        Instruction::Jnz(ri1, ri2) => {
            if ri1.get(state) != 0 {
                state.pc += ri2.get(state) - 1;
            }
        }
        Instruction::Tgl(ri) => {
            let ins_s = state.pc + ri.get(state);

            if ins_s >=0 && ins_s < program.len() as i32 {
                let ins = ins_s as usize;

                let prog_ins = program[ins].clone();

                program[ins] = match prog_ins {
                    Instruction::Cpy(ri1, ri2) => Instruction::Jnz(ri1, ri2),
                    Instruction::Inc(ri) => Instruction::Dec(ri),
                    Instruction::Dec(ri) => Instruction::Inc(ri),
                    Instruction::Jnz(ri1, ri2) => Instruction::Cpy(ri1, ri2),
                    Instruction::Tgl(ri) => Instruction::Inc(ri),
                    Instruction::Out(ri) => Instruction::Inc(ri)
                }
            }
        }
        Instruction::Out(ri) => {
            let value = ri.get(state);
            (state.output)(state, value);
        }
    }

    state.pc += 1;
}

fn parse_reg(string: &str) -> Option<u8> {
    match string {
        "a" => Some(0),
        "b" => Some(1),
        "c" => Some(2),
        "d" => Some(3),
        _ => None
    }
}

fn parse_instructions(lines: &Vec<String>) -> Program {
    let mut program = Vec::new();

    for l in lines {
        let mut terms = l.split_whitespace();

        let instr = match terms.next().unwrap() {
            "cpy" => Instruction::Cpy(RegImm::parse(terms.next().unwrap()).unwrap(), RegImm::parse(terms.next().unwrap()).unwrap()),
            "inc" => Instruction::Inc(RegImm::parse(terms.next().unwrap()).unwrap()),
            "dec" => Instruction::Dec(RegImm::parse(terms.next().unwrap()).unwrap()),
            "jnz" => Instruction::Jnz(RegImm::parse(terms.next().unwrap()).unwrap(), RegImm::parse(terms.next().unwrap()).unwrap()),
            "tgl" => Instruction::Tgl(RegImm::parse(terms.next().unwrap()).unwrap()),
            "out" => Instruction::Out(RegImm::parse(terms.next().unwrap()).unwrap()),
            _ => { panic!("Unrecognised instruction {}", l)}
        };

        program.push(instr);
    }

    program
}

fn load_input(file: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Create buf reader for mmapped file
    let buf_reader = BufReader::new(mmap.as_ref());

    // Create lines vector
    let mut lines = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            lines.push(line);
        }
    }

    Ok(lines)
}
