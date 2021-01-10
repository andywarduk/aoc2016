use memmap::Mmap;
use std::{fmt, fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input23.txt")?;

    let program = parse_instructions(&lines);

    part1(&program);
    part2(&program);
    
    Ok(())
}

fn part1(program: &Vec<Instruction>) {
    let mut program1 = program.clone();
    let mut state: State = Default::default();
    state.reg[0] = 7;
    run(&mut state, &mut program1);
    println!("Register a (part 1) is: {}", state.reg[0]);
}

fn part2(program: &Vec<Instruction>) {
    let mut program2 = program.clone();
    let mut state: State = Default::default();
    state.reg[0] = 12;
    run(&mut state, &mut program2);
    println!("Register a (part 2) is: {}", state.reg[0]);
}

type MachineInt = i32;

#[derive(Default, Debug)]
struct State {
    reg: [MachineInt; 4],
    pc: MachineInt
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Cpy(RegImm, RegImm),
    Inc(RegImm),
    Dec(RegImm),
    Jnz(RegImm, RegImm),
    Tgl(RegImm),
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
            RegImm::Reg(r) => f.write_fmt(format_args!("{}", (*r + 'a' as u8) as char))?,
            RegImm::Imm(i) => f.write_fmt(format_args!("{}", *i))?
        }
        Ok(())
    }
}

type Program = Vec<Instruction>;

fn run(state: &mut State, program: &mut Program) {
    while state.pc >= 0 && (state.pc as usize) < program.len() {
        step(state, program)
    }
}

fn step(state: &mut State, program: &mut Program) {
    match &program[state.pc as usize] {
        Instruction::Cpy(ri1, ri2) => {
            match ri2 {
                RegImm::Reg(r) => state.reg[*r as usize] = ri1.get(&state),
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
            if ri1.get(&state) != 0 {
                state.pc += ri2.get(&state) - 1;
            }
        }
        Instruction::Tgl(ri) => {
            let ins_s = state.pc + ri.get(&state);

            if ins_s >=0 && ins_s < program.len() as i32 {
                let ins = ins_s as usize;

                let prog_ins = program[ins].clone();

                program[ins] = match prog_ins {
                    Instruction::Cpy(ri1, ri2) => Instruction::Jnz(ri1, ri2),
                    Instruction::Inc(ri) => Instruction::Dec(ri),
                    Instruction::Dec(ri) => Instruction::Inc(ri),
                    Instruction::Jnz(ri1, ri2) => Instruction::Cpy(ri1, ri2),
                    Instruction::Tgl(ri) => Instruction::Inc(ri),
                }
            }
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

        if line != "" {
            lines.push(line);
        }
    }

    Ok(lines)
}

#[test]
fn test_exec() {
    let lines = vec![
        "cpy 2 a".to_string(),
        "tgl a".to_string(),
        "tgl a".to_string(),
        "tgl a".to_string(),
        "cpy 1 a".to_string(),
        "dec a".to_string(),
        "dec a".to_string(),
    ];

    let mut program = parse_instructions(&lines);

    let mut state: State = Default::default();

    let pstep = |state: &mut State, program: &mut Program| {
        println!("----- step -----");
        println!("{:?}", state);
        println!("Executing: {:?}", program[state.pc as usize]);
        step(state, program);
        println!("{:?}", state);
        println!("{:?}", program);
    };

    // cpy 2 a initializes register a to 2
    pstep(&mut state, &mut program);
    assert!(state.reg[0] == 2);

    // tgl a modifies the instruction a (2) away from it, which changes the third tgl a into inc a
    pstep(&mut state, &mut program);
    assert!(program[3] == Instruction::Inc(RegImm::Reg(0)));

    // tgl a modifies the instruction a (2) away from it, which changes the cpy 1 a into jnz 1 a
    pstep(&mut state, &mut program);
    assert!(program[4] == Instruction::Jnz(RegImm::Imm(1), RegImm::Reg(0)));

    // The fourth instruction, which is now inc a, increments a to 3
    pstep(&mut state, &mut program);
    assert!(state.reg[0] == 3);

    // The fifth instruction, which is now jnz 1 a, jumps a (3) instructions ahead, skipping the dec a instructions
    pstep(&mut state, &mut program);
    assert!(state.pc == 7);
}
