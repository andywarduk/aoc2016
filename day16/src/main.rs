
const SEED: &str = "10111011111001111";

fn main() {
    println!("Checksum (part 1): {}", calc_checksum(SEED, 272));
    println!("Checksum (part 2): {}", calc_checksum(SEED, 35651584));
}

fn calc_checksum(seed: &str, len: u32) -> String {
    let mut pattern: Vec<bool> = pattern_from_string(seed);

    while pattern.len() < len as usize {
        mutate(&mut pattern);
    }

    pattern.truncate(len as usize);

    let csum = checksum(&pattern);

    pattern_to_string(&csum)
}

fn mutate(pattern: &mut Vec<bool>) {
    let length = pattern.len();

    pattern.push(false);

    for b in (0..length).rev() {
        pattern.push(!pattern[b]);
    }
}

fn checksum(pattern: &Vec<bool>) -> Vec<bool> {
    let mut last = pattern.clone();
    let mut checksum = Vec::new();

    loop {
        for i in (0..last.len()).step_by(2) {
            if last[i] == last[i + 1] {
                checksum.push(true);
            } else {
                checksum.push(false);
            }
        }

        if checksum.len() % 2 == 1 {
            break
        }

        last = checksum;
        checksum = Vec::new();
    }

    checksum
}

fn pattern_from_string(string: &str) -> Vec<bool> {
    string.chars().map(|c| match c {
        '1' => true,
        '0' => false,
        _ => panic!("Invalid char")
    }).collect()
}

fn pattern_to_string(vec: &Vec<bool>) -> String {
    vec.iter().map(|b| {
        if *b { '1' } else { '0' }
    }).collect::<String>()
}

#[test]
fn test_process() {
    fn test(before: &str, after: &str) {
        let mut pattern = pattern_from_string(before);
        mutate(&mut pattern);
        let string = pattern_to_string(&pattern);
        assert!(string == after.to_string());
    }

    test("1", "100");
    test("0", "001");
    test("11111", "11111000000");
    test("111100001010", "1111000010100101011110000");
}
