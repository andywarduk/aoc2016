use std::{fmt, fs::File, io::{BufRead, BufReader}};

use memmap::Mmap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input15.txt")?;

    let eqns = parse_equations(&lines);

    // Part 1
    println!("{:?}", eqns);
    println!("Discs aligned at time {} (part 1)", solve(&eqns, false));

    // Part 2
    let mut eqns2 = eqns.clone();
    eqns2.push(build_disc(11, 0, eqns.len() as u64 + 1));
    println!("{:?}", eqns2);
    println!("Discs aligned at time {} (part 2)", solve(&eqns2, false));

    Ok(())
}

#[derive(Clone, PartialEq)]
struct ModEqn {
    a: u64,
    n: u64
}

impl ModEqn {
    fn new(a: u64, n: u64) -> ModEqn {
        ModEqn { a, n }
    }
}

impl fmt::Debug for ModEqn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("x ≡ {} (mod {})", self.a, self.n))
    }
}

fn solve(eqns_in: &[ModEqn], debug: bool) -> u64 {
    // Sort by a descending
    let mut eqns = eqns_in.to_vec();

    eqns.sort_by(|e1, e2| {
        e2.a.cmp(&e1.a)
    });

    let mut result = eqns[0].a;
    let mut last_eqn = eqns[0].clone();

    for eqn in eqns.iter().skip(1) {
        if debug {
            println!("Calculating {:?} and {:?}", last_eqn, eqn);
        }

        loop {
            let remain = result % eqn.n;

            if debug {
                println!("{} mod {} = {}", result, eqn.n, remain);
            }

            if remain == eqn.a {
                // Found solution
                last_eqn = ModEqn { a: remain, n: last_eqn.n * eqn.n };
                break
            } else {
                // Try next
                result += last_eqn.n;
            }
        }
    }

    result
}

fn parse_equations(lines: &Vec<String>) -> Vec<ModEqn> {
    let mut result = Vec::new();
    let mut time = 0;

    for l in lines {
        let mut terms = l.split_whitespace();

        let positions = terms.nth(3).unwrap().parse::<u64>().unwrap();
        let startpos = terms.nth(7).unwrap().split('.').next().unwrap().parse::<u64>().unwrap();

        time += 1;

        result.push(build_disc(positions, startpos, time));
    }

    result
}

fn build_disc(positions: u64, startpos: u64, time: u64) -> ModEqn {
    let n = positions;
    let mut calc_a = (n as i64 - startpos as i64) + n as i64 - time as i64;
    while calc_a < 0 {
        calc_a += positions as i64;
    }
    let a = calc_a as u64 % n;

    ModEqn::new(a, n)
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
fn test_solve() {
    // Test example from wikipedia: https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Search_by_sieving
    let eqns = vec![ModEqn::new(0, 3), ModEqn::new(3, 4), ModEqn::new(4, 5)];

    assert!(solve(&eqns, true) == 39);

    // AOC example
    let lines = vec![
        "Disc #1 has 5 positions; at time=0, it is at position 4.".to_string(), // =>  x ≡ (5 - 4) - timeoffset (mod 5)  =>  x ≡ 0 (mod 5)
        "Disc #2 has 2 positions; at time=0, it is at position 1.".to_string()  // =>  x ≡ (2 - 1) - timeoffset (mod 2)  =>  x ≡ 1 (mod 2)
    ];
    let eqns = parse_equations(&lines);
    let expected_eqns = vec![ModEqn::new(0, 5), ModEqn::new(1, 2)];

    assert!(eqns == expected_eqns);

    assert!(solve(&eqns, true) == 5);
}
