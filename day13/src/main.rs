use std::{borrow::Cow, cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet, VecDeque}, fs::File};

use gif::{Encoder, Frame};

mod map;

type Coord = u16;
type Dist = u16;

const FAV_NUM: u16 = 1358;
const MAP_DIM: Coord = 60;
const START_X: Coord = 1;
const START_Y: Coord = 1;
const DEST_X: Coord = 31;
const DEST_Y: Coord = 39;

const GIF_MULT: u8 = 8;
const GIF_DIM: u16 = MAP_DIM as u16 * GIF_MULT as u16;

fn main() {
    let map = map::Map::generate(FAV_NUM, MAP_DIM as usize);

    let (steps, path) = shortest_path(&map, START_X, START_Y, DEST_X, DEST_Y);

    println!("Shortest path to {}x{} is {} (part 1)", DEST_X, DEST_Y, steps);
    println!("Path: {:?}", path);

    let visited = walk_for(&map, START_X, START_Y, 50);

    println!("Locations visited is {} (part 2)", visited);
}
struct WorkState1<'a> {
    end_x: Coord,
    end_y: Coord,
    shortest_steps: Dist,
    shortest_path: Vec<(Coord, Coord)>,
    queue: BinaryHeap<State1>,
    map: &'a map::Map,
    visited: HashMap<(Coord, Coord), Dist>,
    gif_encoder: Encoder<&'a mut File>
}

impl<'a> WorkState1<'a> {
    fn have_visited(&self, x: Coord, y: Coord, steps: Dist) -> bool {
        if let Some(d) = self.visited.get(&(x, y)) {
            if *d > steps {
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    fn set_visited(&mut self, x: Coord, y: Coord, steps: Dist) -> bool {
        if let Some(d) = self.visited.get_mut(&(x, y)) {
            if *d < steps {
                *d = steps;
                true
            } else {
                false
            }
        } else {
            self.visited.insert((x, y), steps);
            true
        }
    }

    fn distance_to(&self, x: Coord, y: Coord) -> Dist {
        let xd = (self.end_x as f32 - x as f32).abs();
        let yd =  (self.end_y as f32 - y as f32).abs();

        return ((xd * xd) + (yd * yd)).sqrt() as Dist
    }
}

#[derive(Debug, Clone)]
struct State1 {
    dist: Dist,
    steps: Dist,
    path: Vec<(Coord, Coord)>,
    x: Coord,
    y: Coord,
}

impl Ord for State1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Sort my dist ascending
        let cmp = self.dist.cmp(&other.dist);

        if cmp == Ordering::Equal {
            // Sort by steps descending
            other.steps.cmp(&self.steps)
        } else {
            cmp
        }
    }
}

impl PartialOrd for State1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State1 {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist && self.steps == other.steps
    }
}

impl Eq for State1 {}

fn shortest_path(map: &map::Map, sx: Coord, sy: Coord, dx: Coord, dy: Coord) -> (Dist, Vec<(Coord, Coord)>) {
    // Start GIF
    let mut image = File::create("output13-1.gif").unwrap();
    let color_map = &[0, 0, 0,  0xFF, 0xFF, 0xFF,  0xA0, 0x00, 0x00,  0x00, 0xA0, 0x00,  0xC0, 0xC0, 0x00];
    let encoder = Encoder::new(&mut image, GIF_DIM, GIF_DIM, color_map).unwrap();
    
    // Set up work state
    let mut work_state = WorkState1 {
        end_x: dx,
        end_y: dy,
        shortest_steps: Dist::MAX,
        shortest_path: Vec::new(),
        queue: BinaryHeap::new(),
        map,
        visited: HashMap::new(),
        gif_encoder: encoder
    };

    // Add initial state
    work_state.queue.push(State1 {
        dist: work_state.distance_to(sx, sy),
        steps: 0,
        path: vec![(sx, sy)],
        x: sx,
        y: sy
    });

    // Process the work queue
    while let Some(next) = work_state.queue.pop() {
        next_moves(&mut work_state, next);
    }

    (work_state.shortest_steps, work_state.shortest_path)
}

fn next_moves(work_state: &mut WorkState1, state: State1) {
    let x = state.x;
    let y = state.y;

    // Draw GIF frame
    draw_frame1(work_state, &state);

    // At destination?
    if state.x == work_state.end_x && state.y == work_state.end_y {
        if work_state.shortest_steps > state.steps {
            work_state.shortest_steps = state.steps;
            work_state.shortest_path = state.path.clone();
        }
        return
    }

    // Already taken too many steps compared to shortest path?
    if state.steps >= work_state.shortest_steps {
        return
    }
    
    // Already visited?
    if !work_state.set_visited(x, y, state.steps) {
        return
    }

    // Add moves
    let mut add_move = |ix: isize, iy: isize| {
        loop {
            if ix < 0 || iy < 0 {
                break
            }

            let x = ix as Coord;
            let y = iy as Coord;

            if !work_state.map.movable(x as usize, y as usize) {
                break
            }

            if work_state.have_visited(x, y, state.steps) {
                break
            }

            let mut new_state = State1 {
                x,
                y,
                steps: state.steps + 1,
                path: state.path.clone(),
                dist: work_state.distance_to(x, y)
            };

            new_state.path.push((x, y));

            work_state.queue.push(new_state);

            break
        }
    };

    add_move(x as isize - 1 , y as isize);
    add_move(x as isize + 1, y as isize);
    add_move(x as isize, y as isize - 1);
    add_move(x as isize, y as isize + 1);
}

fn draw_frame1(work_state: &mut WorkState1, state: &State1) {
    let mut frame_data: [u8; GIF_DIM as usize * GIF_DIM as usize] = [0; GIF_DIM as usize * GIF_DIM as usize];

    // Draw the walls
    work_state.map.draw_gif(&mut frame_data, GIF_MULT, GIF_DIM, 1);

    // Draw visited in colour 2
    for ((x, y), _) in &work_state.visited {
        work_state.map.draw_block(*x, *y, 2, &mut frame_data, GIF_MULT, GIF_DIM);
    }

    // Draw current path in colour 3
    for (x, y) in &state.path {
        work_state.map.draw_block(*x, *y, 3, &mut frame_data, GIF_MULT, GIF_DIM);
    }

    // Draw queue in colour 4
    for s in &work_state.queue {
        work_state.map.draw_block(s.x, s.y, 4, &mut frame_data, GIF_MULT, GIF_DIM);
    }
    
    // Write frame
    let mut frame = Frame::default();
    frame.delay = 3;
    frame.width = GIF_DIM;
    frame.height = GIF_DIM;
    frame.buffer = Cow::Borrowed(&frame_data);
    work_state.gif_encoder.write_frame(&frame).unwrap();
}

struct WorkState2<'a> {
    dist: Dist,
    queue: VecDeque<State2>,
    map: &'a map::Map,
    visited: HashSet<(Coord, Coord)>,
    gif_encoder: Encoder<&'a mut File>
}

impl<'a> WorkState2<'a> {
    fn have_visited(&self, x: Coord, y: Coord) -> bool {
        self.visited.get(&(x, y)).is_some()
    }

    fn set_visited(&mut self, x: Coord, y: Coord) {
        self.visited.insert((x, y));
    }
}

#[derive(Debug, Clone)]
struct State2 {
    steps: Dist,
    x: Coord,
    y: Coord,
}

fn walk_for(map: &map::Map, sx: Coord, sy: Coord, dist: Dist) -> usize {
    // Start GIF
    let mut image = File::create("output13-2.gif").unwrap();
    let color_map = &[0, 0, 0,  0xFF, 0xFF, 0xFF,  0xA0, 0x00, 0x00,  0xC0, 0xC0, 0x00];
    let encoder = Encoder::new(&mut image, GIF_DIM, GIF_DIM, color_map).unwrap();
    
    // Set up work state
    let mut work_state = WorkState2 {
        dist,
        queue: VecDeque::new(),
        map,
        visited: HashSet::new(),
        gif_encoder: encoder
    };

    // Add initial state
    work_state.queue.push_back(State2 {
        steps: 0,
        x: sx,
        y: sy
    });
    
    // Process the work queue
    while let Some(next) = work_state.queue.pop_front() {
        walk(&mut work_state, next);
    }
    
    work_state.visited.len()
}

fn walk(work_state: &mut WorkState2, state: State2) {
    let x = state.x;
    let y = state.y;

    // Mark as visited
    work_state.set_visited(x, y);
    
    // Draw GIF frame
    draw_frame2(work_state);

    // Already taken too may steps compared to shortest path?
    if state.steps == work_state.dist {
        return
    }

    // Add moves
    let mut add_move = |ix: isize, iy: isize| {
        loop {
            if ix < 0 || iy < 0 {
                break
            }

            let x = ix as Coord;
            let y = iy as Coord;

            if !work_state.map.movable(x as usize, y as usize) {
                break
            }

            if work_state.have_visited(x, y) {
                break
            }

            let new_state = State2 {
                x,
                y,
                steps: state.steps + 1,
            };

            work_state.queue.push_back(new_state);

            break
        }
    };

    add_move(x as isize - 1 , y as isize);
    add_move(x as isize + 1, y as isize);
    add_move(x as isize, y as isize - 1);
    add_move(x as isize, y as isize + 1);
}

fn draw_frame2(work_state: &mut WorkState2) {
    let mut frame_data: [u8; GIF_DIM as usize * GIF_DIM as usize] = [0; GIF_DIM as usize * GIF_DIM as usize];

    // Draw the walls
    work_state.map.draw_gif(&mut frame_data, GIF_MULT, GIF_DIM, 1);

    // Draw visited in colour 2
    for (x, y) in &work_state.visited {
        work_state.map.draw_block(*x, *y, 2, &mut frame_data, GIF_MULT, GIF_DIM);
    }

    // Draw queue in colour 3
    for s in &work_state.queue {
        work_state.map.draw_block(s.x, s.y, 3, &mut frame_data, GIF_MULT, GIF_DIM);
    }
    
    // Write frame
    let mut frame = Frame::default();
    frame.delay = 2;
    frame.width = GIF_DIM;
    frame.height = GIF_DIM;
    frame.buffer = Cow::Borrowed(&frame_data);
    work_state.gif_encoder.write_frame(&frame).unwrap();
}
