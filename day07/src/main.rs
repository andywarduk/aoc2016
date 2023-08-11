use memmap::Mmap;
use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addresses = load_input("input07.txt")?;

    let mut tls = 0;
    let mut ssl = 0;

    for a in addresses {
        let elems: Vec<&str> = a.split(|c| {
            matches!(c, '[' | ']')
        }).collect();
    
        if address_supports_tls(&elems) {
            tls += 1;
        }

        if address_supports_ssl(&elems) {
            ssl += 1;
        }
    }

    println!("{} addresses support TLS (part 1)", tls);
    println!("{} addresses support SSL (part 2)", ssl);

    Ok(())
}

fn address_supports_tls(elems: &[&str]) -> bool {
    let mut result = false;

    for (idx, e) in elems.iter().enumerate() {
        if idx % 2 == 0 {
            // Outside brackets
            if contains_abba(e) {
                result = true;
            }
        } else {
            // Inside brackets
            if contains_abba(e) {
                return false;
            }
        }
    }

    result
}

fn address_supports_ssl(elems: &[&str]) -> bool {
    let mut result = false;

    let mut abas = Vec::new();

    // Loop strings outside brackets
    for e in elems.iter().step_by(2) {
        contains_aba(e, &mut abas);
    }

    if !abas.is_empty() {
        // Loop strings outside brackets
        for e in elems.iter().skip(1).step_by(2) {
            for aba in &abas {
                if contains_bab(e, aba.1, aba.0) {
                    result = true;
                    break
                }
            }

            if result {
                break
            }
        }
    }

    result
}

fn contains_abba(string: &str) -> bool {
    let chars: Vec<char> = string.chars().collect();

    for i in 0..=string.len() - 4 {
        if chars[i] == chars[i + 3] && chars[i] != chars[i + 1] && chars[i + 1] == chars[i + 2] {
            return true
        }
    }

    false
}

fn contains_aba(string: &str, abas: &mut Vec<(char, char)>) {
    let chars: Vec<char> = string.chars().collect();

    for i in 0..=string.len() - 3 {
        if chars[i] == chars[i + 2] && chars[i] != chars[i + 1] {
            abas.push((chars[i], chars[i + 1]))
        }
    }
}

fn contains_bab(string: &str, c1: char, c2: char) -> bool {
    let chars: Vec<char> = string.chars().collect();

    for i in 0..=string.len() - 3 {
        if chars[i] == c1 && chars[i + 1] == c2 && chars[i + 2] == c1 {
            return true
        }
    }

    false
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
