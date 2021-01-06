use memmap::Mmap;
use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input08.txt")?;

    let commands = parse_commands(lines);

    process(&commands);

    Ok(())
}

const ROWS: usize = 6;
const COLS: usize = 50;

type Board = [[char; 50]; 6];

fn process(commands: &Vec<Command>) {
    let mut board: Board = [[' '; COLS]; ROWS];

    for cmd in commands {
        match cmd {
            Command::Rect(w, h) => rect(&mut board, *w, *h),
            Command::RotRow(y, shift) => rot_row_by(&mut board, *y, *shift),
            Command::RotCol(x, shift) => rot_col_by(&mut board, *x, *shift)
        }
    }

    println!("{} pixels lit (part 1)", count_lit(&board));

    dump_board(&board);
}

fn rect(board: &mut Board, w: u8, h: u8) {
    for y in 0..h {
        for x in 0..w {
            board[y as usize][x as usize] = '#';
        }
    }
}

fn rot_row_by(board: &mut Board, y: u8, shift: u8) {
    for _ in 0..shift {
        rot_row(board, y)
    }
}

fn rot_row(board: &mut Board, y: u8) {
    let save_char = board[y as usize][COLS - 1];

    for x in (1..COLS).rev() {
        board[y as usize][x] = board[y as usize][x - 1];
    }

    board[y as usize][0] = save_char;
}

fn rot_col_by(board: &mut Board, x: u8, shift: u8) {
    for _ in 0..shift {
        rot_col(board, x)
    }
}

fn rot_col(board: &mut Board, x: u8) {
    let save_char = board[ROWS - 1][x as usize];

    for y in (1..ROWS).rev() {
        board[y][x as usize] = board[y - 1][x as usize];
    }

    board[0][x as usize] = save_char;
}

fn count_lit(board: &Board) -> u16 {
    board.iter().map(|row| {
        row.iter().map(|&c| {
            if c == '#' {
                1
            } else {
                0
            }
        }).sum::<u16>()
    }).sum()
}

fn dump_board(board: &Board) {
    for row in board {
        println!("{}", row.iter().collect::<String>());
    }
}

#[derive(Debug)]
enum Command {
    Rect(u8, u8),
    RotRow(u8, u8),
    RotCol(u8, u8)
}

fn parse_commands(lines: Vec<String>) -> Vec<Command> {
    let mut commands = Vec::new();

    for l in lines {
        let mut terms = l.split_whitespace();

        match terms.next().unwrap() {
            "rect" => {
                let mut dims = terms.next().unwrap().split("x");
                let w = dims.next().unwrap().parse::<u8>().unwrap();
                let h = dims.next().unwrap().parse::<u8>().unwrap();

                commands.push(Command::Rect(w, h));
            }
            "rotate" => {
                match terms.next().unwrap() {
                    "row" => {
                        let y = terms.next().unwrap()[2..].parse::<u8>().unwrap();
                        let shift = terms.skip(1).next().unwrap().parse::<u8>().unwrap();
                        commands.push(Command::RotRow(y, shift));
                    }
                    "column" => {
                        let x = terms.next().unwrap()[2..].parse::<u8>().unwrap();
                        let shift = terms.skip(1).next().unwrap().parse::<u8>().unwrap();
                        commands.push(Command::RotCol(x, shift));
                    }
                    _ => panic!("Unrecognised command {}", l)
                }
            }
            _ => panic!("Unrecognised command {}", l)
        }
    }

    commands
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
