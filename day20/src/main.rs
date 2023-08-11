use std::{cmp, fs::File, io::{BufRead, BufReader}};
use memmap::Mmap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input20.txt")?;

    let ranges = parse_ranges(&lines);

    part1(&ranges);
    part2(&ranges);

    Ok(())
}

fn part1(ranges: &Vec<Range>) {
    let mut lowest: u32 = 0;

    for r in ranges {
        if r.lo > lowest {
            break
        }
        lowest = cmp::max(lowest, r.hi + 1);
    }

    println!("Lowest ip is {} (part 1)", lowest);
}

fn part2(ranges: &Vec<Range>) {
    let mut lowest: u32 = 0;
    let mut allowed: u32 = 0;

    for r in ranges {
        if r.lo > lowest{
            allowed += r.lo - lowest;
        }
        
        lowest = if r.hi == u32::MAX {
            u32::MAX
        } else {
            cmp::max(lowest, r.hi + 1)
        }
    }

    println!("Number of allowed ips is {} (part 2)", allowed);
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    lo: u32,
    hi: u32
}

impl Range {
    fn parse(line: &str) -> Range {
        let mut split = line.split('-');

        Range {
            lo: split.next().unwrap().parse::<u32>().unwrap(),
            hi: split.next().unwrap().parse::<u32>().unwrap(),
        }
    }
}

fn parse_ranges(lines: &[String]) -> Vec<Range> {
    let mut ranges: Vec<Range> = lines.iter().map(|s| Range::parse(s)).collect();
    
    ranges.sort();

    ranges
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
