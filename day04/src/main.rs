use memmap2::Mmap;
use std::{cmp::Ordering, collections::HashMap, fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rooms = load_input("input04.txt")?;

    let (answer1, rooms) = part1(rooms);

    let answer2 = part2(rooms);

    println!();
    println!("Sector sum for valid rooms (part 1): {}", answer1);
    println!("North pole object storage sector (part 2): {}", answer2);

    Ok(())
}

fn part1(rooms: Vec<RoomId>) -> (u32, Vec<RoomId>) {
    let mut sector_sum: u32 = 0;
    let mut valid_rooms = Vec::new();

    for r in rooms {
        let mut charmap: HashMap<char, u8> = HashMap::new();

        // Count chars
        for c in r.room.chars() {
            if c != '-' {
                if let Some(count) = charmap.get_mut(&c) {
                    *count += 1;
                } else {
                    charmap.insert(c, 1);
                }
            }
        }

        // Build vector from hashmap
        let mut charvec: Vec<(&char, &u8)> = charmap.iter().collect();

        // Sort by char occurrence descending then char
        charvec.sort_by(|&(&c1, &occ1), (&c2, &occ2)| {
            let cmp1 = occ2.cmp(&occ1);

            if cmp1 == Ordering::Equal {
                c1.cmp(&c2)
            } else {
                cmp1
            }
        });

        // Build expected checksum
        let expected_checksum: String = charvec.iter().take(5).map(|&(&c, _)| c).collect();

        if r.checksum == expected_checksum {
            sector_sum += r.sector as u32;
            valid_rooms.push(r);
        }
    }

    (sector_sum, valid_rooms)
}

fn part2(rooms: Vec<RoomId>) -> u16 {
    let mut answer = 0;

    for r in rooms {
        let decrypted: String = r.room.chars().map(|c| {
            if c == '-' {
                ' '
            } else {
                let mut letter = c as u16 - 'a' as u16;
                letter = (letter + r.sector) % 26;
                (letter as u8 + b'a') as char
            }
        }).collect();

        if decrypted == "northpole object storage" {
            answer = r.sector
        }

        println!("{} = {}", decrypted, r.sector);
    }

    answer
}

struct RoomId {
    room: String,
    sector: u16,
    checksum: String
}

fn load_input(file: &str) -> Result<Vec<RoomId>, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Create buf reader for mmapped file
    let buf_reader = BufReader::new(mmap.as_ref());

    // Create the rooms vector
    let mut rooms = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            let last_dash = line.rfind('-').unwrap();
            let bracket = line.rfind('[').unwrap();

            let room = line[0..last_dash].to_string();
            let sector = line[last_dash + 1..bracket].parse::<u16>().unwrap();
            let checksum = line[bracket + 1..line.len() - 1].to_string();

            rooms.push(RoomId {
                room,
                sector,
                checksum
            });
        }
    }

    Ok(rooms)
}
