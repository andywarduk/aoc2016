use memmap::Mmap;
use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let compressed = load_input("input09.txt")?;

    let uncompressed_len1 = uncompressed_len(&compressed, 1);

    println!("Uncompressed length (part 1): {}", uncompressed_len1);

    let uncompressed_len2 = uncompressed_len(&compressed, 2);

    println!("Uncompressed length (part 2): {}", uncompressed_len2);

    Ok(())
}

fn uncompressed_len(compressed: &str, part: u8) -> usize {
    let compressed_chars: Vec<char> = compressed.chars().collect();

    uncompressed_section_len(&compressed_chars, part, 0, compressed_chars.len())
}

fn uncompressed_section_len(compressed_chars: &Vec<char>, part: u8, start: usize, end: usize) -> usize {
    let mut uncompressed_chars = 0;

    let mut i = start;

    while i < end {
        let c1 = compressed_chars[i];

        match c1 {
            '(' => {
                let (repchars, repeats, next) = get_compression_details(compressed_chars, i);

                if part == 1 {
                    uncompressed_chars += repeats * repchars;
                } else {
                    uncompressed_chars += repeats * uncompressed_section_len(compressed_chars, part, next, next + repchars);
                }

                i = next + repchars;
            }
            _ => {
                uncompressed_chars += 1;
                i += 1;
            }
        }
    }

    uncompressed_chars
}

fn get_compression_details(compressed_chars: &[char], start: usize) -> (usize, usize, usize) {
    let end;
    let mut ex = 0;

    let mut i = start + 1;
    loop {
        let c2 = compressed_chars[i];

        match c2 {
            ')' => {
                end = i;
                break;
            }
            'x' => ex = i,
            _ => {}
        }

        i += 1;
    }

    let repchars = compressed_chars[start + 1..ex].iter().collect::<String>().parse::<usize>().unwrap();
    let repeats = compressed_chars[ex + 1..end].iter().collect::<String>().parse::<usize>().unwrap();

    (repchars, repeats, end + 1)
}

fn load_input(file: &str) -> Result<String, Box<dyn std::error::Error>> {
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

        if !line.is_empty() {
            return Ok(line);
        }
    }

    Err("No data")?
}
