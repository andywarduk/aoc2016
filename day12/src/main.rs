use memmap2::Mmap;
use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input12.txt")?;

    let program = parse_instructions(&lines);

    // Part 1
    let mut state: State = Default::default();
    run(&mut state, &program);
    println!("Register a (part 1) is: {}", state.reg[0]);

    // Part 2
    let mut state: State = Default::default();
    state.reg[2] = 1;
    run(&mut state, &program);
    println!("Register a (part 2) is: {}", state.reg[0]);
    
    Ok(())
}

type MachineInt = i32;

#[derive(Default, Debug)]
struct State {
    reg: [MachineInt; 4],
    pc: MachineInt
}

#[derive(Debug)]
enum Instruction {
    Cpy(RegImm, Reg),
    Inc(Reg),
    Dec(Reg),
    Jnz(RegImm, MachineInt)
}

#[derive(Debug)]
enum RegImm {
    Reg(Reg),
    Imm(MachineInt)
}

impl RegImm {
    fn parse(string: &str) -> Option<RegImm> {
        if let Some(r) = Reg::parse(string) {
            Some(RegImm::Reg(r))
        } else if let Ok(i) = string.parse::<MachineInt>() {
            Some(RegImm::Imm(i))
        } else {
            None
        }
    }

    fn get(&self, state: &State) -> MachineInt {
        match self {
            RegImm::Reg(Reg(r)) => state.reg[*r as usize],
            RegImm::Imm(i) => *i
        }
    }
}

#[derive(Debug)]
struct Reg(u8);

type Program = Vec<Instruction>;

fn run(state: &mut State, program: &Program) {
    while state.pc >= 0 && (state.pc as usize) < program.len() {
        match &program[state.pc as usize] {
            Instruction::Cpy(ri, Reg(r)) => {
                state.reg[*r as usize] = ri.get(state);
            }
            Instruction::Inc(Reg(r)) => {
                state.reg[*r as usize] += 1;
            }
            Instruction::Dec(Reg(r)) => {
                state.reg[*r as usize] -= 1;
            }
            Instruction::Jnz(ri, i) => {
                if ri.get(state) != 0 {
                    state.pc += i - 1;
                }
            }
        }

        state.pc += 1;
    }
}

impl Reg {
    fn parse(string: &str) -> Option<Reg> {
        match string {
            "a" => Some(Reg(0)),
            "b" => Some(Reg(1)),
            "c" => Some(Reg(2)),
            "d" => Some(Reg(3)),
            _ => None
        }
    }
}

fn parse_instructions(lines: &Vec<String>) -> Program {
    let mut program = Vec::new();

    for l in lines {
        let mut terms = l.split_whitespace();

        let instr = match terms.next().unwrap() {
            "cpy" => Instruction::Cpy(RegImm::parse(terms.next().unwrap()).unwrap(), Reg::parse(terms.next().unwrap()).unwrap()),
            "inc" => Instruction::Inc(Reg::parse(terms.next().unwrap()).unwrap()),
            "dec" => Instruction::Dec(Reg::parse(terms.next().unwrap()).unwrap()),
            "jnz" => Instruction::Jnz(RegImm::parse(terms.next().unwrap()).unwrap(), terms.next().unwrap().parse::<MachineInt>().unwrap()),
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
