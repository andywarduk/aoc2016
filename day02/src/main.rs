use memmap::Mmap;
use std::{cmp::{max, min}, fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let directions = load_input("input02.txt")?;

    part1(&directions);
    part2(&directions);

    Ok(())
}

const KEYS1: [[char; 3]; 3] = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9']
];

fn part1(directions: &Vec<Vec<char>>) {
    let mut key_presses: Vec<char> = Vec::new();
    let mut x: i8 = 1;
    let mut y: i8 = 1;

    for keypress in directions {
        for dir in keypress {
            match dir {
                'U' => y = max(0, y - 1),
                'D' => y = min(2, y + 1),
                'L' => x = max(0, x - 1),
                'R' => x = min(2, x + 1),
                _ => panic!("Unrecognised direction {}", dir)
            }
        }

        key_presses.push(KEYS1[y as usize][x as usize]);
    }

    println!("Key code (part 1): {}", key_presses.iter().collect::<String>());
}

const KEYS2: [[char; 5]; 5] = [
    [' ', ' ', '1', ' ', ' '],
    [' ', '2', '3', '4', ' '],
    ['5', '6', '7', '8', '9'],
    [' ', 'A', 'B', 'C', ' '],
    [' ', ' ', 'D', ' ', ' '],
];

fn part2(directions: &Vec<Vec<char>>) {
    let mut key_presses: Vec<char> = Vec::new();
    let mut x: i8 = 1;
    let mut y: i8 = 1;

    for keypress in directions {
        for dir in keypress {
            let (newx, newy) = match dir {
                'U' => (x, y - 1),
                'D' => (x, y + 1),
                'L' => (x - 1, y),
                'R' => (x + 1, y),
                _ => panic!("Unrecognised direction {}", dir)
            };

            if !(0..=4).contains(&newx) || !(0..=4).contains(&newy) {
                continue
            }

            if KEYS2[newy as usize][newx as usize] == ' ' {
                continue
            }

            x = newx;
            y = newy;
        }

        key_presses.push(KEYS2[y as usize][x as usize]);
    }

    println!("Key code (part 2): {}", key_presses.iter().collect::<String>());
}

fn load_input(file: &str) -> Result<Vec<Vec<char>>, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Create buf reader for mmapped file
    let buf_reader = BufReader::new(mmap.as_ref());

    // Create the directions vector
    let mut directions = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            directions.push(line.chars().collect());
        }
    }

    Ok(directions)
}
