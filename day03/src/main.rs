use memmap2::Mmap;
use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let triangles = load_input("input03.txt")?;

    part1(&triangles);
    part2(&triangles);

    Ok(())
}

fn part1(triangles: &Vec<Vec<u16>>) {
    let mut valid: u16 = 0;

    for t in triangles {
        if triangle_valid(t[0], t[1], t[2]) {
            valid += 1;
        }
    }

    println!("{} valid triangles (part 1)", valid);
}

fn part2(triangles: &[Vec<u16>]) {
    let mut valid: u16 = 0;
    let mut ti = triangles.iter();

    loop {
        let t1 = ti.next();
        if t1.is_none() {
            break
        }

        let t1 = t1.unwrap();
        let t2 = ti.next().unwrap();
        let t3 = ti.next().unwrap();

        if triangle_valid(t1[0], t2[0], t3[0]) {
            valid += 1;
        }

        if triangle_valid(t1[1], t2[1], t3[1]) {
            valid += 1;
        }

        if triangle_valid(t1[2], t2[2], t3[2]) {
            valid += 1;
        }
    }

    println!("{} valid triangles (part 2)", valid);
}

fn triangle_valid(s1: u16, s2: u16, s3: u16) -> bool {
    s1 + s2 > s3 && s1 + s3 > s2 && s2 + s3 > s1
}

fn load_input(file: &str) -> Result<Vec<Vec<u16>>, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Create buf reader for mmapped file
    let buf_reader = BufReader::new(mmap.as_ref());

    // Create the triangles vector
    let mut triangles = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            triangles.push(line.split_whitespace().map(|ls| ls.parse::<u16>().unwrap()).collect());
        }
    }

    Ok(triangles)
}
