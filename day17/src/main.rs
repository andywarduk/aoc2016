use std::collections::VecDeque;

// Seed from input
const SEED: &str = "gdjjyniy";

const MAX_X: u8 = 3;
const MAX_Y: u8 = 3;

fn main() {
    let shortest_path = shortest(SEED);
    println!("Shortest path (part 1): {}", shortest_path.unwrap());

    let longest_len = longest(SEED);
    println!("Longest path (part 2): {}", longest_len);
}

#[derive(Debug)]
struct WorkItem {
    x: u8,
    y: u8,
    path: String
}

fn shortest(seed: &str) -> Option<String> {
    let mut shortest_path = None;

    // Create work queue
    let mut workq = VecDeque::new();

    // Add initial work item
    workq.push_back(WorkItem {
        x: 0,
        y: 0,
        path: "".to_string()
    });

    // Process work queue
    while let Some(workitem) = workq.pop_front() {
        let x = workitem.x;
        let y = workitem.y;

        if x == MAX_X && y == MAX_Y {
            // Got solution
            if shortest_path == None {
                shortest_path = Some(workitem.path.clone());
            }

            break
        }

        for m in get_moves(x, y, seed, &workitem.path) {
            workq.push_back(WorkItem {
                x: m.x,
                y: m.y,
                path: add_path(&workitem.path, m.dir)
            });
        }
    }

    shortest_path
}

fn longest(seed: &str) -> usize {
    let mut longest_len = 0;

    longest_iterate(0, 0, "".to_string(), seed, &mut longest_len);

    longest_len
}

fn longest_iterate(x: u8, y: u8, path: String, seed: &str, longest_len: &mut usize) {
    if x == MAX_X && y == MAX_Y {
        // Got solution
        if path.len() > *longest_len {
            *longest_len = path.len();
        }

        return
    }

    for m in get_moves(x, y, seed, &path) {
        longest_iterate(m.x, m.y, add_path(&path, m.dir), seed, longest_len);
    }
}

struct Move {
    x: u8,
    y: u8,
    dir: char
}

fn get_moves(x: u8, y: u8, seed: &str, path: &String) -> Vec<Move> {
    let mut moves = Vec::with_capacity(4);

    // Calculate unlocked doors
    let digest = md5::compute(format!("{}{}", seed, path));

    let unlocked = |byte, shift: u8| (digest[byte] >> shift ) & 0x0f >= 0x0b;

    let mut add = |x, y, dir| moves.push(Move { x, y, dir });

    // Down
    if y < MAX_Y && unlocked(0, 0) {
        add(x, y + 1, 'D')
    }

    // Right
    if x < MAX_X && unlocked(1, 0) {
        add(x + 1, y, 'R')
    }

    // Up
    if y > 0 && unlocked(0, 4) {
        add(x, y - 1, 'U')
    }

    // Left
    if x > 0 && unlocked(1, 4) {
        add(x - 1, y, 'L')
    }

    moves
}

#[inline]
fn add_path(old_path: &String, new_dir: char) -> String {
    let mut new_path = String::with_capacity(old_path.len() + 1);

    new_path.push_str(old_path);
    new_path.push(new_dir);

    new_path
}

#[test]
fn test_walk() {
    let shortest_path = shortest("ihgpwlah").unwrap();
    assert!(shortest_path.as_str() == "DDRRRD");

    let longest_len = longest("ihgpwlah");
    assert!(longest_len == 370);
}
