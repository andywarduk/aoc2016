use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

mod map;

type Coord = u16;
type Dist = u16;

const FAV_NUM: u16 = 1358;
const MAP_DIM: Coord = 60;
const START_X: Coord = 1;
const START_Y: Coord = 1;
const DEST_X: Coord = 31;
const DEST_Y: Coord = 39;

fn main() {
    let map = map::Map::generate(FAV_NUM, MAP_DIM as usize);

    let work_state = shortest_path(&map, START_X, START_Y, DEST_X, DEST_Y);

    println!("Shortest path to {}x{} is {} (part 1)", DEST_X, DEST_Y, work_state.shortest_steps);
    println!("Path: {:?}", work_state.shortest_path);
}

#[derive(Debug)]
struct WorkState<'a> {
    end_x: Coord,
    end_y: Coord,
    shortest_steps: Dist,
    shortest_path: Vec<(Coord, Coord)>,
    queue: BinaryHeap<State>,
    map: &'a map::Map,
    visited: HashMap<(Coord, Coord), Dist>
}

impl<'a> WorkState<'a> {
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
        return ((self.end_x as isize - x as isize).abs() + (self.end_y as isize - y as isize).abs()) as Dist
    }
}

#[derive(Debug, Clone)]
struct State {
    dist: Dist,
    steps: Dist,
    path: Vec<(Coord, Coord)>,
    x: Coord,
    y: Coord,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Sort my dist ascending
        let mut cmp = self.dist.cmp(&other.dist);

        if cmp == Ordering::Equal {
            // Sort by steps descending
            cmp = other.steps.cmp(&self.steps);
        }

        cmp
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist && self.steps == other.steps
    }
}

impl Eq for State {}

fn shortest_path(map: &map::Map, sx: Coord, sy: Coord, dx: Coord, dy: Coord) -> WorkState {
    let mut work_state = WorkState {
        end_x: dx,
        end_y: dy,
        shortest_steps: Dist::MAX,
        shortest_path: Vec::new(),
        queue: BinaryHeap::new(),
        map,
        visited: HashMap::new()
    };

    work_state.queue.push(State {
        dist: work_state.distance_to(sx, sy),
        steps: 0,
        path: vec![(sx, sy)],
        x: sx,
        y: sy
    });

    loop {
        if let Some(next) = work_state.queue.pop() {
            next_moves(&mut work_state, next);
        } else {
            break
        }
    }

    work_state
}

fn next_moves(work_state: &mut WorkState, state: State) {
    let x = state.x;
    let y = state.y;

    // At destination?
    if state.x == work_state.end_x && state.y == work_state.end_y {
        if work_state.shortest_steps > state.steps {
            work_state.shortest_steps = state.steps;
            work_state.shortest_path = state.path.clone();
        }
        return
    }

    // Already taken too may steps compared to shortest path?
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

            let mut new_state = State {
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
