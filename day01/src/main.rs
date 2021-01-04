use memmap::Mmap;
use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let directions = load_input("input01.txt")?;

    let mut direction: i16 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut visited: HashSet<String> = HashSet::new();
    let mut first_double: Option<(i16, i16)> = None;

    visited.insert(format!("{}x{}", x, y));

    for (turn, length) in directions {
        match turn {
            'L' => {
                direction = direction - 90;
                if direction < 0 { direction += 360 }
            }
            'R' => direction = (direction + 90) % 360,
            _ => panic!("Invalid turn {}", turn)
        }

        let (xadd, yadd) = match direction {
            0 => (0, 1),
            90 => (1, 0),
            180 => (0, -1),
            270 => (-1 , 0),
            _ => panic!("Invalid direction {}", direction)
        };

        for _ in 0..length {
            x += xadd;
            y += yadd;

            if !visited.insert(format!("{}x{}", x, y)) {
                if first_double == None {
                    first_double = Some((x, y))
                }
            }
        }
    }

    println!("End location (part 1): x {}, y {} => distance {}", x, y, x.abs() + y.abs());
    if let Some((x, y)) = first_double {
        println!("First location visted twice (part 2): x {}, y {} => distance {}", x, y, x.abs() + y.abs());
    } else {
        panic!("No location visited twice")
    }

    Ok(())
}

type Direction = (char, u8);

fn load_input(file: &str) -> Result<Vec<Direction>, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Create buf reader for mmapped file
    let buf_reader = BufReader::new(mmap.as_ref());

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if line != "" {
            let directions: Vec<(char, u8)> = line.split(", ").map(|d| {
                (d.chars().next().unwrap(), d[1..].parse::<u8>().unwrap())
            }).collect();

            return Ok(directions);
        }
    }

    Err("Directions not found")?
}
