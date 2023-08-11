use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}, fmt, hash::Hash, ops::{Deref, DerefMut}};

/*
first floor: strontium generator, strontium-compatible microchip, plutonium generator, plutonium-compatible microchip
second floor: thulium generator, ruthenium generator, ruthenium-compatible microchip, curium generator, curium-compatible microchip
third floor: thulium-compatible microchip
fourth floor: 

F4
F3                  TC
F2               TG    RG RC CG CC
F1 E SG SC PG PC
*/

fn main() {
    part1();
    part2();
}

fn part1() {
    // Initialise state
    let mut state: State = Default::default();

    // Add input objects
    add_input_objs(&mut state);

    // Initialise answer
    let mut answer: Answer = Default::default();

    // Make the next move (recursively)
    process(&mut answer, state);

    // Print results
    println!("{} moves (part 1)", answer.min_moves);
    println!("Moves are: {:?}", answer.moves);
}

fn part2() {
    // Initialise state
    let mut state: State = Default::default();

    // Add input objects
    add_input_objs(&mut state);

    // Extra objects for part 2
    state.add_obj(0, Source::Elerium, Type::Gen);
    state.add_obj(0, Source::Elerium, Type::Chip);
    state.add_obj(0, Source::Dilithium, Type::Gen);
    state.add_obj(0, Source::Dilithium, Type::Chip);

    // Initialise answer
    let mut answer: Answer = Default::default();

    // Make the next move (recursively)
    process(&mut answer, state);

    // Print results
    println!("{} moves (part 2)", answer.min_moves);
    println!("Moves are: {:?}", answer.moves);
}

fn add_input_objs(state: &mut State) {
    // Add objects (from input)
    state.add_obj(0, Source::Strontium, Type::Gen);
    state.add_obj(0, Source::Strontium, Type::Chip);
    state.add_obj(0, Source::Plutonium, Type::Gen);
    state.add_obj(0, Source::Plutonium, Type::Chip);

    state.add_obj(1, Source::Thulium, Type::Gen);
    state.add_obj(1, Source::Ruthenium, Type::Gen);
    state.add_obj(1, Source::Ruthenium, Type::Chip);
    state.add_obj(1, Source::Curium, Type::Gen);
    state.add_obj(1, Source::Curium, Type::Chip);

    state.add_obj(2, Source::Thulium, Type::Chip);
}

#[derive(Debug)]
enum Type {
    Gen,
    Chip
}

const GEN_SHIFT: u8 = 8;

#[derive(Debug, Clone, Copy)]
enum Source {
    Strontium,
    Plutonium,
    Thulium,
    Ruthenium,
    Curium,
    Elerium,
    Dilithium
}

const SOURCE_VEC: [Source; 7] = [Source::Strontium, Source::Plutonium, Source::Thulium, Source::Ruthenium,
    Source::Curium, Source::Elerium, Source::Dilithium];

#[derive(Clone)]
struct Floor(u16);

impl Floor {
    fn get_objects(&self) -> Vec<Object> {
        let mut result = Vec::new();
    
        for e in SOURCE_VEC.iter() {
            let src_u8 = *e as u8;
            let cobj = Object(1 << src_u8);
            let gobj = Object(1 << (GEN_SHIFT + src_u8));
    
            if self.0 & *cobj != 0 {
                result.push(cobj)
            }

            if self.0 & *gobj != 0 {
                result.push(gobj)
            }
        }
    
        result
    }

    fn empty(&self) -> bool {
        self.0 == 0
    }

    fn valid(&self) -> bool {
        // Enumerate all chips
        for e1 in SOURCE_VEC.iter() {
            let src1_u8 = *e1 as u8;
            let cbit1 = 1 << src1_u8;
    
            if self.0 & cbit1 != 0 {
                // Got this chip - look for matching generator
                let gbit1 = 1 << (GEN_SHIFT + src1_u8);
    
                if self.0 & gbit1 == 0 {
                    // No generator for this chip - check there are no other generators
                    for e2 in SOURCE_VEC.iter() {
                        let src2_u8 = *e2 as u8;
                        let gbit2 = 1 << (GEN_SHIFT + src2_u8);
    
                        if self.0 & gbit2 != 0 {
                            // Got a generator
                            return false
                        }
                    }
                }
            }
        }
    
        true
    }
    
    fn hash(&self) -> FloorHash {
        let mut pairs: u8 = 0;
        let mut chips: u8 = 0;
        let mut gens: u8 = 0;
    
        for e in SOURCE_VEC.iter() {
            let cbit = 1 << *e as u8;
            let gbit = 1 << (GEN_SHIFT + *e as u8);
    
            let cmask = self.0 & cbit;
            let gmask = self.0 & gbit;
    
            if cmask != 0{
                if gmask != 0 {
                    pairs += 1;
                } else {
                    chips += 1;
                }
            } else if gmask != 0 {
                gens += 1;
            }
        }
    
        FloorHash::new(pairs, chips, gens)
    }    
}

impl fmt::Debug for Floor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let obj = self.get_objects();
        let list = obj.iter().map(|o| format!("{:?}", o)).collect::<Vec<String>>().join(", ");
        f.write_fmt(format_args!("{}", list))
    }
}

struct FloorHash(u16); // Actually 9 bits used

impl FloorHash {
    fn new(pairs: u8, chips: u8, gens: u8) -> FloorHash {
        #[cfg(debug_assertions)]
        assert!(pairs < 8 && chips < 8 && gens < 8);

        FloorHash(((pairs as u16) << 6) | ((chips as u16) << 3) | (gens as u16))
    }
}

impl Deref for FloorHash {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for FloorHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("[p={} c={} g={}]", self.0 >> 6, (self.0 >> 3) & 0x7, self.0 & 0x7))
    }
}

#[derive(Clone, Debug)]
struct Map(Vec<Floor>);

impl Map {
    fn hash(&self, floor: usize) -> MapHash {
        let mut hash: u64 = 0;
    
        // Shift in 9 bits per floor (total 27 bits)
        for floor in &self.0 {
            hash <<= 9;
            hash |= *floor.hash() as u64;
        }
    
        // Shift in current floor (2 bits => total 29 bits)
        hash <<= 2;

        #[cfg(debug_assertions)]
        assert!(floor < 4);

        hash |= floor as u64;

        MapHash(hash)
    }
}

impl Deref for Map {
    type Target = Vec<Floor>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(PartialEq, Eq, Clone)]
struct MapHash(u64);

impl Hash for MapHash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.0)
    }
}

impl fmt::Debug for MapHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tmp = self.0;
        let mut outstr = String::from("[");

        // Shift out floor
        let floor = tmp & 0x3;
        tmp >>= 2;

        for i in (0..4).rev() {
            outstr.push_str(&format!("{}", i));
            if i == floor {
                outstr.push('*');
            }
            outstr.push_str(&format!("={:?}", FloorHash((tmp & 0xffff) as u16)));
            outstr.push(if i == 0 { ']'} else { ' ' });

            tmp >>= 9;
        }

        f.write_str(&outstr)
    }
}

#[derive(Clone)]
struct Object(u16);

impl Object {
    fn from_parts(src: &Source, typ: &Type) -> Object {
        let shift = match typ {
            Type::Chip => 0,
            Type::Gen => GEN_SHIFT
        };
    
        #[cfg(debug_assertions)]
        assert!((1 << *src as u8) < (1 << GEN_SHIFT));
    
        let bit = 1 << (shift + *src as u8);
    
        Object(bit)
    }
}

impl Deref for Object {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for e in SOURCE_VEC.iter() {
            let cbit = 1 << *e as u8;
            let gbit = 1 << (GEN_SHIFT + *e as u8);

            if self.0 & cbit != 0 {
                return f.write_fmt(format_args!("{:?}-{:?}", *e, Type::Chip))
            }

            if self.0 & gbit != 0 {
                return f.write_fmt(format_args!("{:?}-{:?}", *e, Type::Gen))
            }
        }

        f.write_fmt(format_args!("Invalid object: {}", self.0))
    }
}

#[derive(Clone, Debug)]
struct State {
    floor: usize,
    map: Map,
    score: usize,
    moves: usize,
    last_moves: Vec<Move>,
    add_hash: MapHash
}

impl State {
    fn add(&mut self, floor: usize, object: &Object) {
        self.map[floor].0 |= object.0;
        self.score += floor;
    }

    fn add_obj(&mut self, floor: usize, src: Source, typ: Type) {
        self.add(floor, &Object::from_parts(&src, &typ))
    }

    fn remove(&mut self, floor: usize, object: &Object) {
        self.map[floor].0 &= !object.0;
        self.score -= floor;
    }

    fn make_move(&mut self, mv: Move) -> bool {
        // Calculate new floor
        let new_floor = match mv {
            Move::One(dir, _) | Move::Two(dir, _, _) => {
                (self.floor as isize + dir as isize) as usize
            }
        };

        // Move out
        self.move_out(self.floor, &mv);

        // Validate
        let from_floor = &mut self.map[self.floor];
        if !from_floor.valid() {
            return false
        }
    
        // Move in
        self.move_in(new_floor, &mv);

        // Validate
        let to_floor = &mut self.map[new_floor];
        if !to_floor.valid() {
            return false
        }

        // Set new floor
        self.floor = new_floor;

        // Increment moves
        self.moves += 1;

        // Save last move
        self.last_moves.push(mv);

        true
    }

    fn move_out(&mut self, from: usize, mv: &Move) {
        match mv {
            Move::One(_, obj) => {
                self.remove(from, obj);
            }
            Move::Two(_, obj1, obj2) => {
                self.remove(from, obj1);
                self.remove(from, obj2);
            }
        }
    }

    fn move_in(&mut self, to: usize, mv: &Move) {
        match mv {
            Move::One(_, obj) => {
                self.add(to, obj);
            }
            Move::Two(_, obj1, obj2) => {
                self.add(to, obj1);
                self.add(to, obj2);
            }
        }
    }

    fn finished(&self) -> bool {
        self.map[0].empty() && self.map[1].empty() && self.map[2].empty()
    }
}

impl Default for State {
    fn default() -> Self {
        let map = Map(vec![Floor(0), Floor(0), Floor(0), Floor(0)]);
        let floor = 0;
        let hash = map.hash(floor);

        State {
            floor,
            map,
            score: 0,
            moves: 0,
            last_moves: Vec::new(),
            add_hash: hash
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Sort by score descending and moves ascending
        let mut cmp = self.score.cmp(&other.score);
        if cmp == Ordering::Equal {
            cmp = other.moves.cmp(&self.moves);
        }
        cmp
    }
}

impl PartialOrd<State> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for State {}

impl PartialEq<State> for State {
    fn eq(&self, other: &State) -> bool {
        self.score == other.score && self.moves == other.moves
    }
}

#[derive(Clone)]
enum Move {
    One(i8, Object),
    Two(i8, Object, Object)
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dir_str = match self {
            Move::One(dir, _) | Move::Two(dir, _, _) => match dir {
                1 => "Up",
                -1 => "Down",
                _ => "<Unknown>"
            }
        };

        match self {
            Move::One(_, i1) => f.write_fmt(format_args!("{} with {:?}", dir_str, i1)),
            Move::Two(_, i1, i2) => f.write_fmt(format_args!("{} {:?} and {:?}", dir_str, i1, i2))
        }
    }
}

struct Answer {
    min_moves: usize,
    moves: Vec<Move>,
    workq: BinaryHeap<State>,
    seen_states: HashMap<MapHash, usize>
}

impl Default for Answer {
    fn default() -> Self {
        Answer {
            min_moves: usize::MAX,
            moves: Vec::new(),
            workq: BinaryHeap::new(),
            seen_states: HashMap::new()
        }
    }
}

fn process(answer: &mut Answer, state: State) {
    answer.workq.push(state);

    while let Some(state) = answer.workq.pop() {
        next_move(answer, state);
    }
}

fn next_move(answer: &mut Answer, state: State) {
    // Check we can beat the best answer so far
    if state.moves >= answer.min_moves {
        return
    }

    // Hash the current state
    let hash = &state.add_hash;

    // Check we haven't seen this state before
    if let Some(moves) = answer.seen_states.get_mut(hash) {
        // Seen this state - was it less moves?
        if *moves <= state.moves {
            #[cfg(test)]
            println!("Seen hash {:?} before ({} moves). Current moves {}", hash, *moves, state.moves);
            return
        }

        #[cfg(test)]
        println!("Seen hash {:?} before ({} moves). Setting to {}", hash, *moves, state.moves);

        *moves = state.moves
    } else {
        // Not seen this state
        #[cfg(test)]
        println!("Adding hash {:?} ({} moves)", hash, state.moves);

        if answer.seen_states.insert(hash.clone(), state.moves).is_some() {
            panic!("seen_state insertion error");
        }
    }

    // Get current floor
    let floor = &state.map[state.floor];

    // Get items on this floor
    let singles = floor.get_objects();

    // Calculate combinations of objects that can move together
    let combinations = calc_combinations(&singles);

    #[cfg(test)]
    {
        // Print separator
        println!("----------------------- Work Item -----------------------");
        println!("Score={}, Moves={}", state.score, state.moves);

        // Print map
        for (i, f) in state.map.iter().enumerate().rev() {
            println!("{}: {} {:?}", i, if state.floor == i { 'E' } else { ' ' }, f);
        }

        println!("Single moves: {:?}", singles);
        println!("Double moves: {:?}", combinations);
    }

    if state.floor < 3 {
        // Consider double moves up
        for (i1, i2) in &combinations {
            let mv = Move::Two(1, i1.clone(), i2.clone());

            try_move(&state, answer, mv);
        }

        // Consider single moves up
        for i in &singles {
            let mv = Move::One(1, i.clone());

            try_move(&state, answer, mv);
        }
    }

    if state.floor > 0 {
        // Consider single moves down
        for i in &singles {
            let mv = Move::One(-1, i.clone());

            try_move(&state, answer, mv);
        }
    }
}

fn calc_combinations(singles: &Vec<Object>) -> Vec<(Object, Object)> {
    let mut result = Vec::new();

    for i in 0..singles.len() - 1 {
        for j in i + 1..singles.len() {
            result.push((singles[i].clone(), singles[j].clone()));
        }
    }

    result
}

fn try_move(state: &State, answer: &mut Answer, mv: Move) {
    // Clone the state
    let mut new_state = state.clone();

    // Try and make the move on the new state
    if new_state.make_move(mv) {
        // Successful

        // Finished?
        if new_state.finished() {
            // Found a solution
            answer.min_moves = new_state.moves;
            answer.moves = new_state.last_moves.clone();

            #[cfg(test)]
            println!("Found solution in {} moves", new_state.moves);

            return
        }

        // Hash the new state
        let hash = new_state.map.hash(state.floor);

        // Check we haven't seen this state before
        if let Some(moves) = answer.seen_states.get(&hash) {
            // Seen this state - was it less or equal moves?
            if *moves <= state.moves {
                // Yes - no point continuing
                return
            }
        }
        
        // Save the hash for later
        new_state.add_hash = hash;

        // Add this state to the work queue
        answer.workq.push(new_state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor_valid() {
        let mut f;
        
        // Valid floors
        f = Floor(*Object::from_parts(&Source::Strontium, &Type::Chip));
        assert!(f.valid());

        f = Floor(*Object::from_parts(&Source::Strontium, &Type::Gen));
        assert!(f.valid());

        f = Floor(*Object::from_parts(&Source::Strontium, &Type::Gen) |
                *Object::from_parts(&Source::Strontium, &Type::Chip));
        assert!(f.valid());

        f = Floor(*Object::from_parts(&Source::Strontium, &Type::Gen) |
                *Object::from_parts(&Source::Strontium, &Type::Chip) |
                *Object::from_parts(&Source::Thulium, &Type::Gen));
        assert!(f.valid());

        // Invalid floors
        f = Floor(*Object::from_parts(&Source::Strontium, &Type::Chip) |
                *Object::from_parts(&Source::Thulium, &Type::Gen) |
                *Object::from_parts(&Source::Thulium, &Type::Chip));
        assert!(!f.valid());

        f = Floor(*Object::from_parts(&Source::Strontium, &Type::Chip) |
                *Object::from_parts(&Source::Thulium, &Type::Gen));
        assert!(!f.valid());

        f = Floor(*Object::from_parts(&Source::Strontium, &Type::Chip) |
                *Object::from_parts(&Source::Thulium, &Type::Gen) |
                *Object::from_parts(&Source::Thulium, &Type::Chip) |
                *Object::from_parts(&Source::Curium, &Type::Gen));
        assert!(!f.valid());
    }

    #[test]
    fn test_example() {
        let mut state: State = Default::default();

        state.add_obj(0, Source::Strontium, Type::Chip);
        state.add_obj(0, Source::Curium, Type::Chip);

        state.add_obj(1, Source::Strontium, Type::Gen);

        state.add_obj(2, Source::Curium, Type::Gen);

        let mut answer: Answer = Default::default();

        process(&mut answer, state);

        println!("{} moves (test): {:?}", answer.min_moves, answer.moves);

        assert!(answer.min_moves == 11);
    }

    #[test]
    fn test_state_hash() {
        let mut state1: State = Default::default();

        let mut state2 = state1.clone();
        let mut state3 = state1.clone();
        let mut state4 = state1.clone();
        let mut state5 = state1.clone();
        let mut state6 = state1.clone();
        let mut state7 = state1.clone();

        state1.add_obj(0, Source::Curium, Type::Gen);
        let hash1 = state1.map.hash(1);

        state2.add_obj(0, Source::Plutonium, Type::Gen);
        let hash2 = state2.map.hash(1);

        assert!(hash1 == hash2);

        state3.add_obj(0, Source::Plutonium, Type::Gen);
        let hash3 = state3.map.hash(2);

        assert!(hash2 != hash3);

        state4.add_obj(0, Source::Plutonium, Type::Gen);
        state4.add_obj(0, Source::Plutonium, Type::Chip);
        let hash4 = state4.map.hash(2);

        state5.add_obj(0, Source::Curium, Type::Gen);
        state5.add_obj(0, Source::Curium, Type::Chip);
        let hash5 = state5.map.hash(2);

        assert!(hash4 == hash5);

        state6.add_obj(3, Source::Strontium, Type::Chip);
        state6.add_obj(3, Source::Plutonium, Type::Chip);
        state6.add_obj(3, Source::Thulium, Type::Chip);
        state6.add_obj(3, Source::Ruthenium, Type::Chip);
        state6.add_obj(2, Source::Strontium, Type::Gen);
        state6.add_obj(2, Source::Plutonium, Type::Gen);
        state6.add_obj(2, Source::Ruthenium, Type::Gen);
        state6.add_obj(2, Source::Curium, Type::Chip);
        state6.add_obj(2, Source::Curium, Type::Gen);
        state6.add_obj(0, Source::Thulium, Type::Gen);
        let hash6 = state6.map.hash(0);

        state7.add_obj(3, Source::Strontium, Type::Chip);
        state7.add_obj(3, Source::Plutonium, Type::Chip);
        state7.add_obj(3, Source::Thulium, Type::Chip);
        state7.add_obj(3, Source::Ruthenium, Type::Chip);
        state7.add_obj(2, Source::Strontium, Type::Gen);
        state7.add_obj(2, Source::Plutonium, Type::Gen);
        state7.add_obj(2, Source::Thulium, Type::Gen); // Swapped
        state7.add_obj(2, Source::Curium, Type::Chip);
        state7.add_obj(2, Source::Curium, Type::Gen);
        state7.add_obj(0, Source::Ruthenium, Type::Gen); // Swapped
        let hash7 = state7.map.hash(0);

        assert!(hash6 == hash7);
    }

    #[test]
    fn test_state_binhash() {
        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        let mut state1: State = Default::default();

        let mut state2 = state1.clone();
        let mut state3 = state1.clone();
        let mut state4 = state1.clone();
        let mut state5 = state1.clone();

        state1.add_obj(0, Source::Plutonium, Type::Gen);
        state1.moves = 5;
        assert!(state1.score == 0);
        heap.push(state1);

        state2.add_obj(0, Source::Plutonium, Type::Gen);
        state2.moves = 4;
        assert!(state2.score == 0);
        heap.push(state2);

        state3.add_obj(0, Source::Plutonium, Type::Gen);
        state3.add_obj(1, Source::Plutonium, Type::Chip);
        state3.moves = 3;
        assert!(state3.score == 1);
        heap.push(state3);

        state4.add_obj(0, Source::Plutonium, Type::Gen);
        state4.add_obj(1, Source::Plutonium, Type::Chip);
        state4.moves = 2;
        assert!(state4.score == 1);
        heap.push(state4);

        state5.add_obj(1, Source::Plutonium, Type::Gen);
        state5.add_obj(2, Source::Plutonium, Type::Chip);
        state5.moves = 1;
        assert!(state5.score == 3);
        heap.push(state5);

        let out1 = heap.pop().unwrap();
        assert!(out1.score == 3);
        assert!(out1.moves == 1);

        let out2 = heap.pop().unwrap();
        assert!(out2.score == 1);
        assert!(out2.moves == 2);

        let out3 = heap.pop().unwrap();
        assert!(out3.score == 1);
        assert!(out3.moves == 3);

        let out4 = heap.pop().unwrap();
        assert!(out4.score == 0);
        assert!(out4.moves == 4);

        let out5 = heap.pop().unwrap();
        assert!(out5.score == 0);
        assert!(out5.moves == 5);
    }
}
