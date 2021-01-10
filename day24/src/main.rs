mod map;
use crate::map::*;

use memmap::Mmap;
use std::{borrow::Cow, cmp, collections::{HashMap, HashSet, VecDeque}, fs::File, io::{BufRead, BufReader}};
use gif::{Frame, Encoder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input24.txt")?;

    let map = map::Map::parse(&lines);

    part1(&map);

    Ok(())
}

fn part1(map: &Map) {
    let gif_scale = 8;
    let gif_width = map.width() * gif_scale;
    let gif_height = map.height() * gif_scale;

    // Start GIF
    let mut image = File::create("output24-1.gif").unwrap();
    let color_map = &[0, 0, 0,  0xff, 0xff, 0xff,  0x00, 0xff, 0xff];
    let mut encoder = Encoder::new(&mut image, gif_width, gif_height, color_map).unwrap();

    // Create frame
    let mut frame_data = vec![0; gif_width as usize * gif_height as usize];

    map.draw(&mut frame_data, gif_width, gif_scale, 0, 1, 2);

    // Write frame
    let mut frame = Frame::default();
    frame.delay = 3;
    frame.width = gif_width;
    frame.height = gif_height;
    frame.buffer = Cow::Borrowed(&frame_data);
    encoder.write_frame(&frame).unwrap();

    // Calculate distances between map items
    let distances = calc_distances(&map);

    // Calculate shortest journey between items
    let shortest = journey(&distances, &map);

    println!("Shortest journey (part 1): {}", shortest);

    // Calculate shortest rond trip
    let shortest = round_trip(&distances, &map);

    println!("Shortest round trip (part 2): {}", shortest);
}

#[derive(Debug)]
struct Distances {
    map: HashMap<(u8, u8), u16>, // Map 2 items to distance, smallest always first
    to_find: Vec<u8>,
}

impl Distances {
    fn new(size: u8) -> Distances {
        let to_find = (size as u16 * (size as u16 - 1)) / 2;

        Distances {
            map: HashMap::with_capacity(to_find as usize),
            to_find: vec![size - 1; size as usize],
        }
    }

    fn add(&mut self, item1: u8, item2: u8, steps: u16) {
        let min_item = cmp::min(item1, item2);
        let max_item = cmp::max(item1, item2);

        if let Some(old_steps) = self.map.get(&(min_item, max_item)) {
            if *old_steps != steps {
                panic!("Distance insertion error")
            }
        } else {
            self.map.insert((min_item, max_item), steps);

            self.to_find[item1 as usize] -= 1;
            self.to_find[item2 as usize] -= 1;
        }
    }

    fn lookup(&self, item1: u8, item2: u8) -> u16 {
        let min_item = cmp::min(item1, item2);
        let max_item = cmp::max(item1, item2);

        *self.map.get(&(min_item, max_item)).unwrap()
    }
}

fn calc_distances(map: &Map) -> Distances {
    let mut distances = Distances::new(map.items().len() as u8);

    for (item, pos) in map.items() {
        walk_from(*item, pos, map, &mut distances);
    }

    distances
}

fn walk_from(start_item: u8, start_pos: &Coord, map: &Map, distances: &mut Distances) {
    #[derive(Clone)]
    struct State {
        pos: Coord,
        steps: u16
    }

    let mut visited: HashSet<Coord> = HashSet::new();

    // Create queue and add first work item (start position)
    let mut queue = VecDeque::new();

    queue.push_back(State {
        pos: start_pos.clone(),
        steps: 0
    });

    while let Some(work_item) = queue.pop_front() {
        if let Some(item) = map.item_check(&work_item.pos) {
            if *item != start_item {
                // Found an item
                distances.add(start_item, *item, work_item.steps);

                // All found?
                if distances.to_find[start_item as usize] == 0 {
                    return
                }
            }
        }

        let mut move_to = |next: Coord| {
            loop {
                if visited.get(&next).is_some() {
                    break
                }

                visited.insert(next.clone());

                if !map.valid_move(&next) {
                    break
                }

                queue.push_back(State {
                    pos: next,
                    steps: work_item.steps + 1
                });
                
                break
            }
        };

        move_to(work_item.pos.new_relative(-1, 0));
        move_to(work_item.pos.new_relative(1, 0));
        move_to(work_item.pos.new_relative(0, -1));
        move_to(work_item.pos.new_relative(0, 1));
    }
}

fn journey(distances: &Distances, map: &Map) -> u16 {
    let nodes = map.items().iter().filter_map(|(&item, _)| {
        if item == 0 {
            None
        } else {
            Some(item)
        }
    }).collect();

    let mut opt_dist = u16::MAX;

    journey_iter(distances, 0, nodes, 0, &mut opt_dist);

    opt_dist
}

fn journey_iter(distances: &Distances, last_node: u8,  nodes: Vec<u8>, dist: u16, opt_dist: &mut u16) {
    for n in &nodes {
        // Get list of next nodes
        let new_nodes: Vec<u8> = nodes.iter().cloned().filter(|new_n| *new_n != *n).collect();

        // Calculate new distance (current distance plus distance of this leg)
        let new_dist = dist + distances.lookup(last_node, *n);

        if new_nodes.len() == 0 {
            // No more nodes - check against current optimal distance
            if new_dist < *opt_dist {
                // New optimal distance
                *opt_dist = new_dist;
            }
        } else {
            // Iterate in to other nodes
            journey_iter(distances, *n, new_nodes, new_dist, opt_dist);
        }
    }
}

fn round_trip(distances: &Distances, map: &Map) -> u16 {
    let nodes = map.items().iter().filter_map(|(&item, _)| {
        if item == 0 {
            None
        } else {
            Some(item)
        }
    }).collect();

    let mut opt_dist = u16::MAX;

    round_trip_iter(distances, 0, nodes, 0, &mut opt_dist);

    opt_dist
}

fn round_trip_iter(distances: &Distances, last_node: u8,  nodes: Vec<u8>, dist: u16, opt_dist: &mut u16) {
    for n in &nodes {
        // Get list of next nodes
        let new_nodes: Vec<u8> = nodes.iter().cloned().filter(|new_n| *new_n != *n).collect();

        // Calculate new distance (current distance plus distance of this leg)
        let mut new_dist = dist + distances.lookup(last_node, *n);

        if new_nodes.len() == 0 {
            // No more nodes - travel back to the start
            new_dist += distances.lookup(*n, 0);

            // Check against current optimal distance
            if new_dist < *opt_dist {
                // New optimal distance
                *opt_dist = new_dist;
            }
        } else {
            // Iterate in to other nodes
            round_trip_iter(distances, *n, new_nodes, new_dist, opt_dist);
        }
    }
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

#[test]
fn test_dist() {
    let lines = vec![
        "###########".to_string(),
        "#0.1.....2#".to_string(),
        "#.#######.#".to_string(),
        "#4.......3#".to_string(),
        "###########".to_string(),
    ];

    let map = map::Map::parse(&lines);

    // Calculate distances between map items
    let distances = calc_distances(&map);

    println!("Distances ({}): {:?}", distances.map.len(), distances);

    assert!(distances.lookup(0, 1) == 2);
    assert!(distances.lookup(0, 2) == 8);
    assert!(distances.lookup(0, 3) == 10);
    assert!(distances.lookup(0, 4) == 2);
    assert!(distances.lookup(1, 2) == 6);
    assert!(distances.lookup(1, 3) == 8);
    assert!(distances.lookup(1, 4) == 4);
    assert!(distances.lookup(2, 3) == 2);
    assert!(distances.lookup(2, 4) == 10);
    assert!(distances.lookup(3, 4) == 8);

    let opt_dist = journey(&distances, &map);
    println!("Optimal distance: {}", opt_dist);

    assert!(opt_dist == 14);
}
