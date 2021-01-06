use std::{collections::HashMap, fmt};

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
    // Initialise state
    let mut state = State {
        floor: 0,
        map: vec![Floor(0), Floor(0), Floor(0), Floor(0)],
        moves: 0,
        last_moves: Vec::new(),
    };

    // Add objects (from input)
    state.map[0].add_obj(Source::Strontium, Type::Gen);
    state.map[0].add_obj(Source::Strontium, Type::Chip);
    state.map[0].add_obj(Source::Plutonium, Type::Gen);
    state.map[0].add_obj(Source::Plutonium, Type::Chip);

    state.map[1].add_obj(Source::Thulium, Type::Gen);
    state.map[1].add_obj(Source::Ruthenium, Type::Gen);
    state.map[1].add_obj(Source::Ruthenium, Type::Chip);
    state.map[1].add_obj(Source::Curium, Type::Gen);
    state.map[1].add_obj(Source::Curium, Type::Chip);

    state.map[2].add_obj(Source::Thulium, Type::Chip);

    // Initialise answer
    let mut answer = Answer {
        min_moves: usize::MAX,
        moves: Vec::new(),
        seen_states: HashMap::new()
    };

    // Make the next move (recursively)
    next_move(&mut answer, &mut state);

    // Print results
    println!("{} moves (part 1)", answer.min_moves);
    println!("Moved are: {:?}", answer.moves);
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
}

const SOURCE_VEC: [Source; 5] = [Source::Strontium, Source::Plutonium, Source::Thulium, Source::Ruthenium, Source::Curium];

#[derive(Clone, Debug)]
struct Floor(u16);

impl Floor {
    fn add(&mut self, object: &Object) {
        self.0 |= object.0;
    }

    fn add_obj(&mut self, src: Source, typ: Type) {
        self.add(&parts_to_object(&src, &typ))
    }

    fn remove(&mut self, object: &Object) {
        self.0 &= !object.0
    }

    fn get_objects(&self) -> Vec<Object> {
        let mut result = Vec::new();
    
        for e in SOURCE_VEC.iter() {
            let src_u8 = *e as u8;
            let cobj = Object(1 << src_u8);
            let gobj = Object(1 << (GEN_SHIFT + src_u8));
    
            if self.0 & cobj.0 != 0 {
                result.push(cobj)
            }

            if self.0 & gobj.0 != 0 {
                result.push(gobj)
            }
        }
    
        result
    }
}

#[derive(Clone)]
struct Object(u16);

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

type Map = Vec<Floor>;

#[derive(Clone)]
struct State {
    floor: usize,
    map: Map,
    moves: usize,
    last_moves: Vec<Move>,
}

impl State {
    fn make_move(&mut self, answer: &mut Answer, mv: Move) {
        let new_floor;

        // Calculate new floor
        match mv {
            Move::One(dir, _) | Move::Two(dir, _, _) => {
                new_floor = (self.floor as isize + dir as isize) as usize;
            }
        }

        // Move out
        let from_floor = &mut self.map[self.floor];
        move_out(from_floor, &mv);

        // Move in
        let to_floor = &mut self.map[new_floor];
        move_in(to_floor, &mv);

        // Set new floor
        self.floor = new_floor;

        // Increment moves
        self.moves += 1;

        // Save last move
        self.last_moves.push(mv);

        // Save map hash
        let hash = map_hash(&self.map);

        if let Some(moves) = answer.seen_states.get_mut(&hash) {
            if *moves < self.moves {
                panic!("seen_state update error")
            }
            *moves = self.moves
        } else {
            if answer.seen_states.insert(hash, self.moves) != None {
                panic!("seen_state insertion error");
            }
        }
    }

    fn finished(&self) -> bool {
        self.map[0].0 == 0 && self.map[1].0 == 0 && self.map[2].0 == 0
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
    seen_states: HashMap<u64, usize>
}

fn next_move(answer: &mut Answer, state: &mut State) {
    let floor = &state.map[state.floor];
    let mut moves: Vec<Move> = Vec::new();

    let singles = floor.get_objects();
    let combinations = calc_combinations(&singles);

    #[cfg(test)]
    println!("Singles: {:?}", singles);
    #[cfg(test)]
    println!("Combs: {:?}", combinations);

    if state.floor < 3 {
        // Consider double moves up
        for (i1, i2) in &combinations {
            let mv = Move::Two(1, i1.clone(), i2.clone());

            if valid_move(&state, answer, &mv) {
                moves.push(mv);
            }
        }

        // Consider single moves up
        for i in &singles {
            let mv = Move::One(1, i.clone());

            if valid_move(&state, answer, &mv) {
                moves.push(mv);
            }
        }
    }

    if state.floor > 0 {
        // Consider single moves down
        for i in &singles {
            let mv = Move::One(-1, i.clone());

            if valid_move(&state, answer, &mv) {
                moves.push(mv);
            }
        }
    }
    
    #[cfg(test)]
    println!("Moves: {} {} {:?}", state.moves, state.floor, moves);

    for m in moves {
        let mut new_state = state.clone();

        #[cfg(test)]
        println!("Moving: {:?}", m);

        new_state.make_move(answer, m);

        if new_state.moves < answer.min_moves {
            if new_state.finished() {
                #[cfg(test)]
                println!("Found solution in {} moves", new_state.moves);

                answer.min_moves = new_state.moves;
                answer.moves = new_state.last_moves.clone();
            } else if new_state.moves < answer.min_moves {
                next_move(answer, &mut new_state);
            }
        }
    }
}

fn valid_move(state: &State, answer: &Answer, mv: &Move) -> bool {
    // Move out and test
    let mut floor_from = state.map[state.floor].clone();

    move_out(&mut floor_from, &mv);

    if !floor_valid(&floor_from) {
        return false
    }

    // Move in and test
    let floor_num_to;
    let mut floor_to;

    match mv {
        Move::One(dir, _) | Move::Two(dir, _, _)=> {
            floor_num_to = (state.floor as isize + *dir as isize) as usize;
            floor_to = state.map[floor_num_to].clone();
        }
    };

    move_in(&mut floor_to, &mv);

    if !floor_valid(&floor_to) {
        return false
    }

    // State valid - have we seen it in fewer moves before?
    let mut new_map = state.map.clone();
    new_map[state.floor] = floor_from;
    new_map[floor_num_to] = floor_to;
    let hash = map_hash(&new_map);
    
    if let Some(seen) = answer.seen_states.get(&hash) {
        if *seen <= state.moves {
            return false
        }
    }
    
    true
}

fn move_out(from: &mut Floor, mv: &Move) {
    match mv {
        Move::One(_, obj) => {
            from.remove(obj);
        }
        Move::Two(_, obj1, obj2) => {
            from.remove(obj1);
            from.remove(obj2);
        }
    }
}

fn move_in(to: &mut Floor, mv: &Move) {
    match mv {
        Move::One(_, obj) => {
            to.add(obj);
        }
        Move::Two(_, obj1, obj2) => {
            to.add(obj1);
            to.add(obj2);
        }
    }
}

fn floor_valid(floor: &Floor) -> bool {
    // Enumerate all chips
    for e1 in SOURCE_VEC.iter() {
        let src1_u8 = *e1 as u8;
        let cbit1 = 1 << src1_u8;

        if floor.0 & cbit1 != 0 {
            // Got this chip - look for matching generator
            let gbit1 = 1 << (GEN_SHIFT + src1_u8);

            if floor.0 & gbit1 == 0 {
                // No generator for this chip - check there are no other generators
                for e2 in SOURCE_VEC.iter() {
                    let src2_u8 = *e2 as u8;
                    let gbit2 = 1 << (GEN_SHIFT + src2_u8);

                    if floor.0 & gbit2 != 0 {
                        // Got a generator
                        return false
                    }
                }
            }
        }
    }

    true
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

fn map_hash(map: &Map) -> u64 {
    let mut hash: u64 = 0;

    for floor in map {
        hash <<= 16;
        hash |= hash_floor(floor) as u64;
    }

    hash
}

fn hash_floor(floor: &Floor) -> u16 {
    let mut pairs: u8 = 0;
    let mut chips: u8 = 0;
    let mut gens: u8 = 0;

    for e in SOURCE_VEC.iter() {
        let cbit = 1 << *e as u8;
        let gbit = 1 << (GEN_SHIFT + *e as u8);

        let cmask = floor.0 & cbit;
        let gmask = floor.0 & gbit;

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

    ((pairs as u16) << 10) | ((chips as u16) << 5) | (gens as u16)
}

fn parts_to_object(src: &Source, typ: &Type) -> Object {
    let shift = match typ {
        Type::Chip => 0,
        Type::Gen => GEN_SHIFT
    };

    let bit = 1 << (shift + *src as u8);

    Object(bit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor_valid()
    {
        let mut f;
        
        // Valid floors
        f = Floor(parts_to_object(&Source::Strontium, &Type::Chip).0);
        assert!(floor_valid(&f));

        f = Floor(parts_to_object(&Source::Strontium, &Type::Gen).0);
        assert!(floor_valid(&f));

        f = Floor(parts_to_object(&Source::Strontium, &Type::Gen).0 |
                parts_to_object(&Source::Strontium, &Type::Chip).0);
        assert!(floor_valid(&f));

        f = Floor(parts_to_object(&Source::Strontium, &Type::Gen).0 |
                parts_to_object(&Source::Strontium, &Type::Chip).0 |
                parts_to_object(&Source::Thulium, &Type::Gen).0);
        assert!(floor_valid(&f));

        // Invalid floors
        f = Floor(parts_to_object(&Source::Strontium, &Type::Chip).0 |
                parts_to_object(&Source::Thulium, &Type::Gen).0 |
                parts_to_object(&Source::Thulium, &Type::Chip).0);
        assert!(!floor_valid(&f));

        f = Floor(parts_to_object(&Source::Strontium, &Type::Chip).0 |
                parts_to_object(&Source::Thulium, &Type::Gen).0);
        assert!(!floor_valid(&f));

        f = Floor(parts_to_object(&Source::Strontium, &Type::Chip).0 |
                parts_to_object(&Source::Thulium, &Type::Gen).0 |
                parts_to_object(&Source::Thulium, &Type::Chip).0 |
                parts_to_object(&Source::Curium, &Type::Gen).0);
        assert!(!floor_valid(&f));
    }

    #[test]
    fn test_part1() {
        let mut state = State {
            floor: 0,
            map: vec![Floor(0), Floor(0), Floor(0), Floor(0)],
            moves: 0,
            last_moves: Vec::new(),
        };

        state.map[0].add_obj(Source::Strontium, Type::Chip);
        state.map[0].add_obj(Source::Curium, Type::Chip);

        state.map[1].add_obj(Source::Strontium, Type::Gen);

        state.map[2].add_obj(Source::Curium, Type::Gen);

        let mut answer = Answer {
            min_moves: usize::MAX,
            moves: Vec::new(),
            seen_states: HashMap::new()
        };

        next_move(&mut answer, &mut state);

        println!("{} moves (test): {:?}", answer.min_moves, answer.moves);

        assert!(answer.min_moves == 11);
    }
}
