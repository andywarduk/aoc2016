use memmap::Mmap;
use std::{borrow::Cow, fs::File, io::{BufRead, BufReader}};
use gif::{Frame, Encoder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input08.txt")?;

    let commands = parse_commands(lines);

    process(&commands);

    Ok(())
}

const ROWS: usize = 6;
const COLS: usize = 50;

const GIF_MULT: u8 = 8;

const GIF_W: u16 = (COLS * GIF_MULT as usize) as u16;
const GIF_H: u16 = (ROWS * GIF_MULT as usize) as u16;

type Board = [[char; COLS]; ROWS];

fn process(commands: &Vec<Command>) {
    let mut board: Board = [[' '; COLS]; ROWS];

    // Start GIF
    let mut image = File::create("output08.gif").unwrap();
    let color_map = &[0, 0, 0, 0xFF, 0xFF, 0xFF];
    let mut encoder = Encoder::new(&mut image, GIF_W, GIF_H, color_map).unwrap();

    // Write GIF frame
    write_gif_board(&board, &mut encoder);

    // Process commands
    for cmd in commands {
        match cmd {
            Command::Rect(w, h) => rect(&mut board, &mut encoder, *w, *h),
            Command::RotRow(y, shift) => rot_row_by(&mut board, &mut encoder, *y, *shift),
            Command::RotCol(x, shift) => rot_col_by(&mut board, &mut encoder, *x, *shift)
        }
    }

    println!("{} pixels lit (part 1)", count_lit(&board));

    println!();

    dump_board(&board);
}

fn rect(board: &mut Board, encoder: &mut Encoder<&mut File>, w: u8, h: u8) {
    for y in 0..h {
        for x in 0..w {
            board[y as usize][x as usize] = '#';
        }
    }

    // Write GIF frame
    write_gif_board(board, encoder);
}

fn rot_row_by(board: &mut Board, encoder: &mut Encoder<&mut File>, y: u8, shift: u8) {
    for _ in 0..shift {
        rot_row(board, y);

        // Write GIF frame
        write_gif_board(board, encoder);
    }
}

fn rot_row(board: &mut Board, y: u8) {
    let save_char = board[y as usize][COLS - 1];

    for x in (1..COLS).rev() {
        board[y as usize][x] = board[y as usize][x - 1];
    }

    board[y as usize][0] = save_char;
}

fn rot_col_by(board: &mut Board, encoder: &mut Encoder<&mut File>, x: u8, shift: u8) {
    for _ in 0..shift {
        rot_col(board, x);

        // Write GIF frame
        write_gif_board(board, encoder);
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

fn write_gif_board(board: &Board, encoder: &mut Encoder<&mut File>) {
    let mut frame_data: [u8; (GIF_W * GIF_H) as usize] = [0; (GIF_W * GIF_H) as usize];

    // Build frame
    for (y, row) in board.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '#' {
                let gx_orgn = x * GIF_MULT as usize;
                let gy_orgn = y * GIF_MULT as usize;

                for gy in gy_orgn..gy_orgn + GIF_MULT as usize {
                    let out_elem = (gy * GIF_W as usize) + gx_orgn;
                    for i in 0..GIF_MULT as usize {
                        frame_data[out_elem + i] = 1;
                    }
                }
            }
        }
    }

    // Write frame
    let frame = Frame {
        delay: 3,
        width: GIF_W,
        height: GIF_H,
        buffer: Cow::Borrowed(&frame_data),
        ..Frame::default()
    };

    encoder.write_frame(&frame).unwrap();
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
                let mut dims = terms.next().unwrap().split('x');
                let w = dims.next().unwrap().parse::<u8>().unwrap();
                let h = dims.next().unwrap().parse::<u8>().unwrap();

                commands.push(Command::Rect(w, h));
            }
            "rotate" => {
                match terms.next().unwrap() {
                    "row" => {
                        let y = terms.next().unwrap()[2..].parse::<u8>().unwrap();
                        let shift = terms.nth(1).unwrap().parse::<u8>().unwrap();
                        commands.push(Command::RotRow(y, shift));
                    }
                    "column" => {
                        let x = terms.next().unwrap()[2..].parse::<u8>().unwrap();
                        let shift = terms.nth(1).unwrap().parse::<u8>().unwrap();
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

        if !line.is_empty() {
            lines.push(line);
        }
    }

    Ok(lines)
}
