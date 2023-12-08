use memmap2::Mmap;
use std::{collections::{HashMap, VecDeque}, fs::File, io::{BufRead, BufReader}, rc::Rc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input10.txt")?;

    let (inputs, bots) = parse_instructions(&lines);

    let mut state = State {
        bots,
        giveq: inputs,
        outputs: HashMap::new()
    };

    run(&mut state);

    let out0 = state.outputs.get(&0).unwrap();
    let out1 = state.outputs.get(&1).unwrap();
    let out2 = state.outputs.get(&2).unwrap();

    println!("Outputs: 0={}, 1={}, 2={} - product {} (part 2)", out0, out1, out2, *out0 as u32 * *out1 as u32 * *out2 as u32);

    Ok(())
}

struct State {
    bots: HashMap<u16, Bot>,
    giveq: VecDeque<Movement>,
    outputs: HashMap<u16, u16>
}

struct Movement {
    chip: u16,
    to: Rc<Dest>
}

#[derive(Debug)]
enum Dest {
    Output(u16),
    Bot(u16)
}

impl Dest {
    fn new(typ: &str, num: &str) -> Dest {
        match typ {
            "output" => Dest::Output(num.parse::<u16>().unwrap()),
            "bot" => Dest::Bot(num.parse::<u16>().unwrap()),
            _ => panic!("Unrecognised dest type {}", typ)
        }
    }
}

struct Bot {
    num: u16,
    lo_to: Rc<Dest>,
    hi_to: Rc<Dest>,
    chips: Vec<u16>
}

impl Bot {
    fn give(&mut self, giveq: &mut VecDeque<Movement>, chip: u16) {
        self.chips.push(chip);

        if self.chips.len() == 2 {
            self.chips.sort();

            if self.chips[0] == 17 && self.chips[1] == 61 {
                println!("Robot {} compares 17 with 61 (part 1)", self.num);
            }

            giveq.push_back(Movement {
                chip: self.chips[0],
                to: self.lo_to.clone()
            });

            giveq.push_back(Movement {
                chip: self.chips[1],
                to: self.hi_to.clone()
            });

            self.chips.clear();
        }
    }
}

fn run(state: &mut State) {
    loop {
        match state.giveq.pop_front(){
            None => break,
            Some(movement) => {
                give(state, movement.chip, &movement.to);
            }
        }
    }
}

fn give(state: &mut State, chip: u16, dest: &Dest) {
    match dest {
        Dest::Output(out) => {
            state.outputs.insert(*out, chip);
        }
        Dest::Bot(bot) => {
            give_bot(state, *bot, chip);
        }
    }
}

fn give_bot(state: &mut State, bot: u16, chip: u16) {
    let bot = state.bots.get_mut(&bot).unwrap();

    bot.give(&mut state.giveq, chip);
}

fn parse_instructions(lines: &Vec<String>) -> (VecDeque<Movement>, HashMap<u16, Bot>) {
    let mut inputs = VecDeque::new();
    let mut bots = HashMap::new();

    for l in lines {
        let mut terms = l.split_whitespace();

        match terms.next().unwrap() {
            "bot" => {
                let bot_no = terms.next().unwrap().parse::<u16>().unwrap();
                let lo = Dest::new(terms.nth(3).unwrap(), terms.next().unwrap());
                let hi = Dest::new(terms.nth(3).unwrap(), terms.next().unwrap());

                bots.insert(bot_no, Bot {
                    num: bot_no,
                    lo_to: Rc::new(lo),
                    hi_to: Rc::new(hi),
                    chips: Vec::new()
                });
            },
            "value" => {
                let val = terms.next().unwrap().parse::<u16>().unwrap();
                let bot = terms.nth(3).unwrap().parse::<u16>().unwrap();

                inputs.push_back(Movement {
                    chip: val,
                    to: Rc::new(Dest::Bot(bot))
                })
            }
            _ => panic!("Can't parse line: {}", l)
        }
    }

    (inputs, bots)
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
