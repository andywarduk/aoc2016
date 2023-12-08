use memmap2::Mmap;
use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let line = load_input("input18.txt")?;

    let map1 = Map::generate(&line, 40);

    println!("{} safe tiles (part 1)", map1.count_safe());

    let map2 = Map::generate(&line, 400000);

    println!("{} safe tiles (part 2)", map2.count_safe());

    Ok(())
}

#[derive(PartialEq)]
enum Block {
    Safe,
    Trap
}

struct Map {
    map: Vec<Vec<Block>>
}

impl Map {
    fn generate(line1: &str, line_cnt: usize) -> Map {
        let mut map = Map {
            map: Vec::with_capacity(line_cnt)
        };

        let cols = line1.len();

        map.map.push(string_to_map_row(line1));

        for y in 1..line_cnt {
            let mut row = Vec::with_capacity(cols);

            for x in 0..cols {
                let left = if x == 0 {
                    &Block::Safe
                } else {
                    &map.map[y - 1][x - 1]
                };

                let right = if x == cols - 1 {
                    &Block::Safe
                } else {
                    &map.map[y - 1][x + 1]
                };

                if (*left == Block::Trap && *right == Block::Safe) || (*left == Block::Safe && *right == Block::Trap) {
                    row.push(Block::Trap)
                } else {
                    row.push(Block::Safe)
                }
            }

            map.map.push(row);
        }

        map
    }

    fn count_safe(&self) -> usize {
        self.map.iter().map(|r| r.iter().filter(|&s| *s == Block::Safe).count()).sum()
    }
}

fn string_to_map_row(string: &str) -> Vec<Block> {
    string.chars().map(|c| match c {
        '.' => Block::Safe,
        '^' => Block::Trap,
        _ => panic!("Unrecognised map character '{}'", c)
    }).collect()
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
    let line = buf_reader.lines().next().unwrap()?;

    Ok(line)
}

#[test]
fn test_map_gen() {
    let map = Map::generate("..^^.", 3);

    assert!(map.map == vec![
        string_to_map_row("..^^."),
        string_to_map_row(".^^^^"),
        string_to_map_row("^^..^")
    ]);

    let map = Map::generate(".^^.^.^^^^", 10);
    
    assert!(map.map == vec![
        string_to_map_row(".^^.^.^^^^"),
        string_to_map_row("^^^...^..^"),
        string_to_map_row("^.^^.^.^^."),
        string_to_map_row("..^^...^^^"),
        string_to_map_row(".^^^^.^^.^"),
        string_to_map_row("^^..^.^^.."),
        string_to_map_row("^^^^..^^^."),
        string_to_map_row("^..^^^^.^^"),
        string_to_map_row(".^^^..^.^^"),
        string_to_map_row("^^.^^^..^^"),
    ])
}
