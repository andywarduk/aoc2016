use memmap::Mmap;
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let strings = load_input("input06.txt")?;

    // Create a hashmap for each char pos
    let mut char_occs: Vec<HashMap<char, u8>> = vec![HashMap::new(); strings[0].len()];

    // Count character occs for each string
    for s in strings {
        for (idx, c) in s.chars().enumerate() {
            if let Some(ent) = char_occs[idx].get_mut(&c) {
                *ent += 1;
            } else {
                char_occs[idx].insert(c, 1);
            }
        }
    }

    // Build message 1
    let message1: String = char_occs.iter().map(|hm| {
        let mut occ_vec: Vec<(&char, &u8)> = hm.iter().collect();
        occ_vec.sort_by(|&(_, &cnt1), &(_, &cnt2)| cnt2.cmp(&cnt1));
        occ_vec[0].0
    }).collect();

    // Build message 2
    let message2: String = char_occs.iter().map(|hm| {
        let mut occ_vec: Vec<(&char, &u8)> = hm.iter().collect();
        occ_vec.sort_by(|&(_, &cnt1), &(_, &cnt2)| cnt1.cmp(&cnt2));
        occ_vec[0].0
    }).collect();

    // Print results
    println!("Message (part 1): {}", message1);
    println!("Message (part 2): {}", message2);

    Ok(())
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

    // Create the lines vector
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
