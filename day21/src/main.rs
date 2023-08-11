use std::{fs::File, io::{BufRead, BufReader}, str::SplitWhitespace};
use memmap::Mmap;

const INPUT: &str = "abcdefgh";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input21.txt")?;

    let instructions = parse_instructions(&lines);

    part1(&instructions, INPUT);
    part2(&instructions, "fbgdceah");

    Ok(())
}

fn part1(instructions: &[Instruction], input: &str) {
    let mut chars = input.chars().collect();

    for instr in instructions.iter() {
        chars = instr.action(chars, true);
    }

    let string: String = chars.iter().collect();

    println!("Scrambled password (part 1): {}", string);
}

fn part2(instructions: &[Instruction], input: &str) {
    let mut chars = input.chars().collect();

    for instr in instructions.iter().rev() {
        chars = instr.action(chars, false);
    }

    let string: String = chars.iter().collect();

    println!("Unscrambled password (part 2): {}", string);
}

#[derive(Debug)]
enum Instruction {
    SwapPos(usize, usize),
    SwapChar(char, char),
    RotateLeftAmt(usize),
    RotateRightAmt(usize),
    RotateRightPos(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Instruction {
    fn parse(line: &str) -> Instruction {
        let mut terms = line.split_whitespace();

        let usize_at = |terms: &mut SplitWhitespace, pos| {
            terms.nth(pos).unwrap().parse::<usize>().unwrap()
        };

        let char_at = |terms: &mut SplitWhitespace, pos| {
            terms.nth(pos).unwrap().chars().next().unwrap()
        };

        match terms.next().unwrap() {
            "swap" => {
                let term = terms.next().unwrap();
                match term {
                    "position" => Instruction::SwapPos(usize_at(&mut terms, 0), usize_at(&mut terms, 2)),
                    "letter" => Instruction::SwapChar(char_at(&mut terms, 0), char_at(&mut terms, 2)),
                    unk => panic!("Unknown swap term {}", unk)
                }
            }
            "rotate" => {
                match terms.next().unwrap() {
                    "left" => Instruction::RotateLeftAmt(usize_at(&mut terms, 0)),
                    "right" => Instruction::RotateRightAmt(usize_at(&mut terms, 0)),
                    "based" => Instruction::RotateRightPos(char_at(&mut terms, 4)),
                    unk => panic!("Unknown rotate term {}", unk)
                }
            }
            "reverse" => Instruction::Reverse(usize_at(&mut terms, 1), usize_at(&mut terms, 1)),
            "move" => Instruction::Move(usize_at(&mut terms, 1), usize_at(&mut terms, 2 )),
            unk => panic!("Unrecognised term {}", unk)
        }
    }

    fn action(&self, chars: Vec<char>, forwards: bool) -> Vec<char> {
        let mut new_chars = chars.clone();

        let swap_pos = |chars: &mut Vec<char>, p1, p2| {
            chars.swap(p1, p2);
        };

        let find_char = |chars: &Vec<char>, find_c| {
            chars.iter().position(|c| *c == find_c).unwrap()
        };

        let rotate_l = |chars: &mut Vec<char>, p1, p2, amt| {
            assert!(p1 < p2);
            for _ in 0..amt {
                let tmp = chars[p1];
                for i in p1..p2 {
                    chars[i] = chars[i + 1]
                }
                chars[p2] = tmp;
            }
        };

        let rotate_r = |chars: &mut Vec<char>, p1, p2, amt| {
            assert!(p1 < p2);
            for _ in 0..amt {
                let tmp = chars[p2];
                for i in (p1..p2).rev() {
                    chars[i + 1] = chars[i]
                }
                chars[p1] = tmp;
            }
        };

        let reverse = |chars: &mut Vec<char>, mut p1, mut p2| {
            assert!(p1 < p2);
            while p1 < p2 {
                chars.swap(p1, p2);

                p1 += 1;
                p2 -= 1;
            }
        };

        match self {
            Instruction::SwapPos(p1, p2) => swap_pos(&mut new_chars, *p1, *p2),
            Instruction::SwapChar(c1, c2) => {
                let p1 = find_char(&new_chars, *c1);
                let p2 = find_char(&new_chars, *c2);
                swap_pos(&mut new_chars, p1, p2);
            }
            Instruction::RotateLeftAmt(amt) => {
                if forwards {
                    rotate_l(&mut new_chars, 0, chars.len() - 1, *amt)
                } else {
                    rotate_r(&mut new_chars, 0, chars.len() - 1, *amt)
                }
            }
            Instruction::RotateRightAmt(amt) => {
                if forwards {
                    rotate_r(&mut new_chars, 0, chars.len() - 1, *amt)
                } else {
                    rotate_l(&mut new_chars, 0, chars.len() - 1, *amt)
                }
            }
            Instruction::RotateRightPos(c) => {
                let calc_amt = |pos| 1 + if pos >= 4 { pos + 1 } else { pos };

                if forwards {
                    let pos = find_char(&new_chars, *c);
                    rotate_r(&mut new_chars, 0, chars.len() - 1, calc_amt(pos));

                } else {
                    // Rotations by 'a' for 8 character strings:
                    // Orig       Pos  Amt   Result     Pos
                    // abcdefgh -> 0 -> 1 -> habcdefg -> 1
                    // habcdefg -> 1 -> 2 -> fghabcde -> 3
                    // ghabcdef -> 2 -> 3 -> defghabc -> 5
                    // fghabcde -> 3 -> 4 -> bcdefgha -> 7
                    // efghabcd -> 4 -> 6 -> ghabcdef -> 2
                    // defghabc -> 5 -> 7 -> efghabcd -> 4
                    // cdefghab -> 6 -> 8 -> cdefghab -> 6
                    // bcdefgha -> 7 -> 9 -> abcdefgh -> 0

                    // This yields a unique reverse position lookup for 8 character strings
                    
                    assert!(new_chars.len() == 8);

                    let pos = find_char(&new_chars, *c);

                    let orig_pos = if pos % 2 == 1 {
                        (pos - 1) / 2
                    } else {
                        (((pos + 7) % 8) + 7) / 2
                    };

                    rotate_l(&mut new_chars, 0, chars.len() - 1, calc_amt(orig_pos));
                }
            }
            Instruction::Reverse(p1, p2) => reverse(&mut new_chars, *p1, *p2),
            Instruction::Move(p1, p2) => {
                if p1 < p2 {
                    if forwards {
                        rotate_l(&mut new_chars, *p1, *p2, 1);
                    } else {
                        rotate_r(&mut new_chars, *p1, *p2, 1);
                    }
                } else if forwards {
                    rotate_r(&mut new_chars, *p2, *p1, 1);
                } else {
                    rotate_l(&mut new_chars, *p2, *p1, 1);
                }
            }
        }

        new_chars
    }
}

fn parse_instructions(lines: &[String]) -> Vec<Instruction> {
    lines.iter().map(|i| Instruction::parse(i)).collect()
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

#[test]
fn test_example() {
    let example = vec![
        "swap position 4 with position 0".to_string(),
        "swap letter d with letter b".to_string(),
        "reverse positions 0 through 4".to_string(),
        "rotate left 1 step".to_string(),
        "move position 1 to position 4".to_string(),
        "move position 3 to position 0".to_string(),
        "rotate based on position of letter b".to_string(),
        "rotate based on position of letter d".to_string(),
    ];

    let expected = [
        "ebcda",
        "edcba",
        "abcde",
        "bcdea",
        "bdeac",
        "abdec",
        "ecabd",
        "decab",
    ];

    let instructions = parse_instructions(&example);

    let input = "abcde";
    let mut chars: Vec<char> = input.chars().collect();

    println!("--- Forwards---");
    for (idx, instr) in instructions.iter().enumerate() {
        chars = instr.action(chars, true);

        let string: String = chars.iter().collect();

        println!("{:?} => {}", instr, string);

        assert!(string.as_str() == expected[idx], "should be {}", expected[idx]);
    }

    println!("--- Backwards---");
    for (idx, instr) in instructions.iter().enumerate().rev() {
        match instr {
            Instruction::RotateRightPos(_) => {
                // This can't be reversed for 5 character strings - cheat
                chars = expected[idx - 1].chars().collect();
            }
            _ => {
                chars = instr.action(chars, false);
            }
        }

        let string: String = chars.iter().collect();

        println!("{:?} => {}", instr, string);
        
        if idx > 0 {
            assert!(string.as_str() == expected[idx - 1], "should be {}", expected[idx - 1]);
        } else {
            assert!(string.as_str() == input, "should be {}", input);
        }
    }
}

#[test]
fn test_reverse() {
    let teststr = "abcdefgh";
    let testchars: Vec<char> = teststr.chars().collect();

    let test = |instr: Instruction| {
        let mut chars = testchars.clone();

        chars = instr.action(chars, true);
        let fstring: String = chars.iter().collect();

        chars = instr.action(chars, false);
        let rstring: String = chars.iter().collect();

        assert!(rstring.as_str() == teststr, "Instruction {:?} reversed {} to {}", instr, fstring, rstring);
    };

    for i in 0..6 {
        for j in i + 1..7 {
            test(Instruction::Move(i, j));
            test(Instruction::Reverse(i, j));
        }
    }

    for i in 0..7 {
        for j in 0..7 {
            if i != j {
                test(Instruction::SwapChar(testchars[i], testchars[j]));
                test(Instruction::Move(i, j));
            }
        }
    }

    for i in 1..=8 {
        test(Instruction::RotateLeftAmt(i));
        test(Instruction::RotateRightAmt(i));
    }

    for c in testchars.iter() {
        test(Instruction::RotateRightPos(*c));
    }
}
