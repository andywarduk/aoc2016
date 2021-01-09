use std::collections::VecDeque;

const SEED: &str = "gdjjyniy";

const MAX_X: u8 = 3;
const MAX_Y: u8 = 3;

fn main() {
    let shortest_path = shortest(SEED);
    println!("Shortest path (part 1): {}", shortest_path.unwrap());

    let longest_len = longest(SEED);
    println!("Longest path (part 1): {}", longest_len);
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

        // Calculate unlocked doors
        let digest = md5::compute(format!("{}{}", seed, workitem.path));

        // Calculate next states

        // Down
        if y < MAX_Y {
            let d = (digest[0] & 0x0f) >= 0x0b;

            if d {
                workq.push_back(WorkItem {
                    x,
                    y: y + 1,
                    path: add_path(&workitem.path, 'D')
                });
            }
        }

        // Right
        if x < MAX_X {
            let r = (digest[1] & 0x0f) >= 0x0b;

            if r {
                workq.push_back(WorkItem {
                    x: x + 1,
                    y,
                    path: add_path(&workitem.path, 'R')
                });
            }
        }

        // Up
        if y > 0 {
            let u = (digest[0] >> 4) >= 0x0b;

            if u {
                workq.push_back(WorkItem {
                    x,
                    y: y - 1,
                    path: add_path(&workitem.path, 'U')
                });
            }
        }

        // Left
        if x > 0 {
            let l = (digest[1] >> 4) >= 0x0b;

            if l {
                workq.push_back(WorkItem {
                    x: x - 1,
                    y,
                    path: add_path(&workitem.path, 'L')
                });
            }
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

    // Calculate unlocked doors
    let digest = md5::compute(format!("{}{}", seed, path));

    // Calculate next states

    // Down
    if y < MAX_Y {
        let d = (digest[0] & 0x0f) >= 0x0b;

        if d {
            longest_iterate(x, y + 1, add_path(&path, 'D'), seed, longest_len);
        }
    }

    // Right
    if x < MAX_X {
        let r = (digest[1] & 0x0f) >= 0x0b;

        if r {
            longest_iterate(x + 1, y, add_path(&path, 'R'), seed, longest_len);
        }
    }

    // Up
    if y > 0 {
        let u = (digest[0] >> 4) >= 0x0b;

        if u {
            longest_iterate(x, y - 1, add_path(&path, 'U'), seed, longest_len);
        }
    }

    // Left
    if x > 0 {
        let l = (digest[1] >> 4) >= 0x0b;

        if l {
            longest_iterate(x - 1, y, add_path(&path, 'L'), seed, longest_len);
        }
    }
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
