use std::{collections::{HashMap, HashSet, hash_map::DefaultHasher}, fmt, hash::{Hash, Hasher}};

/*
    first floor: strontium generator, strontium-compatible microchip, plutonium generator, plutonium-compatible microchip
    second floor: thulium generator, ruthenium generator, ruthenium-compatible microchip, curium generator, curium-compatible microchip
    third floor: thulium-compatible microchip
    fourth floor: 

    F4
    F3                  TM
    F2               TG    RG RM CG CM
    F1 E SG SM PG PM
*/

fn main() {
    let mut state = State {
        floor: 0,
        map: vec![Floor::new(), Floor::new(), Floor::new(), Floor::new()],
        moves: 0,
        last_moves: Vec::new(),
    };

    state.map[0].add(Source::Strontium, &Type::Gen);
    state.map[0].add(Source::Strontium, &Type::Chip);
    state.map[0].add(Source::Plutonium, &Type::Gen);
    state.map[0].add(Source::Plutonium, &Type::Chip);

    state.map[1].add(Source::Thulium, &Type::Gen);
    state.map[1].add(Source::Ruthenium, &Type::Gen);
    state.map[1].add(Source::Ruthenium, &Type::Chip);
    state.map[1].add(Source::Curium, &Type::Gen);
    state.map[1].add(Source::Curium, &Type::Chip);

    state.map[2].add(Source::Thulium, &Type::Chip);

    let mut answer = Answer {
        min_moves: usize::MAX,
        moves: Vec::new(),
        seen_states: HashMap::new()
    };

    next_move(&mut answer, &mut state);

    println!("{} moves: {:?}", answer.min_moves, answer.moves);
}

#[derive(Debug, Clone, PartialEq)]
enum Type {
    Gen,
    Chip
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Source {
    Strontium,
    Plutonium,
    Thulium,
    Ruthenium,
    Curium,
#[cfg(test)]
    Hydrogen,
#[cfg(test)]
    Lithium,
}

#[derive(Clone, Debug)]
struct Floor {
    gen: HashSet<Source>,
    chip: HashSet<Source>
}

impl Floor {
    fn new() -> Self {
        Floor {
            gen: HashSet::new(),
            chip: HashSet::new()
        }
    }

    fn add(&mut self, src: Source, typ: &Type) {
        if !match typ {
            Type::Chip => self.chip.insert(src),
            Type::Gen => self.gen.insert(src),
        } {
            panic!("Error adding {:?} to floor", typ)
        }
    }

    fn remove(&mut self, src: &Source, typ: &Type) {
        if !match typ {
            Type::Chip => self.chip.remove(src),
            Type::Gen => self.gen.remove(src),
        } {
            panic!("Error adding {:?} {:?} to floor", typ, src)
        }
    }
}

impl Hash for Floor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Build u64 for the floor state
        let mut bits: u64 = 0;

        for g in &self.gen {
            let bit = 1 << (*g as u8);
            bits |= bit;
        }

        for c in &self.chip {
            let bit = 1 << 32 + (*c as u8);
            bits |= bit;
        }

        bits.hash(state);
    }
}

type Object = (Source, Type);
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

        // Calculat new floor
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
        self.map[0].chip.len() == 0 && self.map[1].chip.len() == 0 && self.map[2].chip.len() == 0 &&
        self.map[0].gen.len() == 0 && self.map[1].gen.len() == 0 && self.map[2].gen.len() == 0
    }
}
#[derive(Clone, PartialEq)]
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
            Move::One(_, i1) => f.write_fmt(format_args!("{} [{:?}]", dir_str, i1)),
            Move::Two(_, i1, i2) => f.write_fmt(format_args!("{} [{:?} {:?}]", dir_str, i1, i2))
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

    let combinations = calc_combinations(&floor);
    let singles = calc_singles(&floor);

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

        // Consider double moves down
        for (i1, i2) in &combinations {
            let mv = Move::Two(-1, i1.clone(), i2.clone());

            if valid_move(&state, answer, &mv) {
                moves.push(mv);
            }
        }
    }
    
    //println!("Moves: {} {} {:?}", state.moves, state.floor, moves);

    for m in moves {
        let mut new_state = state.clone();

        //println!("Moving: {:?}", m);

        new_state.make_move(answer, m);

        if new_state.moves < answer.min_moves {
            if new_state.finished() {
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
    let mut new_map = state.map.clone();

    let floor_from = &mut new_map[state.floor];

    // Move out and test
    move_out(floor_from, &mv);

    if !floor_valid(&floor_from) {
        return false
    }

    // Move in and test
    let floor_to;

    match mv {
        Move::One(dir, _) | Move::Two(dir, _, _)=> {
            floor_to = &mut new_map[(state.floor as isize + *dir as isize) as usize];
        }
    };

    move_in(floor_to, &mv);

    if !floor_valid(&floor_to) {
        return false
    }

    // Have we seen this state in less moves before?
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
        Move::One(_, (src1, typ1)) => {
            from.remove(src1, typ1);
        }
        Move::Two(_, (src1, typ1), (src2, typ2)) => {
            from.remove(src1, typ1);
            from.remove(src2, typ2);
        }
    }
}

fn move_in(to: &mut Floor, mv: &Move) {
    match mv {
        Move::One(_, (src1, typ1)) => {
            to.add(src1.clone(), typ1);
        }
        Move::Two(_, (src1, typ1), (src2, typ2)) => {
            to.add(src1.clone(), typ1);
            to.add(src2.clone(), typ2);
        }
    }
}

fn floor_valid(floor: &Floor) -> bool {
    for c in floor.chip.iter() {
        if floor.gen.get(&c) == None {
            // No generator for this chip - check there are no generators without chips
            for g in floor.gen.iter() {
                if floor.chip.get(&g) == None {
                    // Got generator with no chip
                    return false
                }
            }
        }
    }

    true
}

fn calc_singles(floor: &Floor) -> Vec<(Source, Type)> {
    let mut result = Vec::new();

    for c in floor.chip.iter() {
        result.push((c.clone(), Type::Chip));
    }

    for g in floor.gen.iter() {
        result.push((g.clone(), Type::Gen));
    }

    result
}

fn calc_combinations(floor: &Floor) -> Vec<((Source, Type), (Source, Type))> {
    let mut result = Vec::new();

    let clen = floor.chip.len();
    let glen = floor.gen.len();

    if clen > 0 && glen > 0 {
        for c in floor.chip.iter() {
            for g in floor.gen.iter() {
                result.push(((c.clone(), Type::Chip), (g.clone(), Type::Gen)));
            }
        }

        if clen > 1 {
            for (idx, c1) in floor.chip.iter().take(clen - 1).enumerate() {
                for c2 in floor.chip.iter().skip(idx + 1) {
                    result.push(((c1.clone(), Type::Chip), (c2.clone(), Type::Chip)));
                }
            }
        }

        if glen > 1 {
            for (idx, g1) in floor.gen.iter().take(glen - 1).enumerate() {
                for g2 in floor.gen.iter().skip(idx + 1) {
                    result.push(((g1.clone(), Type::Gen), (g2.clone(), Type::Gen)));
                }
            }
        }
    }

    result
}

fn map_hash(map: &Map) -> u64 {
    let mut s = DefaultHasher::new();
    map.hash(&mut s);
    let hash = s.finish();

    hash
}

#[test]
fn test_part1() {
    let mut state = State {
        floor: 0,
        map: vec![Floor::new(), Floor::new(), Floor::new(), Floor::new()],
        moves: 0,
        last_moves: Vec::new(),
    };

    state.map[0].add(Source::Hydrogen, &Type::Chip);
    state.map[0].add(Source::Lithium, &Type::Chip);

    state.map[1].add(Source::Hydrogen, &Type::Gen);

    state.map[2].add(Source::Lithium, &Type::Gen);

    let mut answer = Answer {
        min_moves: usize::MAX,
        moves: Vec::new(),
        seen_states: HashMap::new()
    };

    next_move(&mut answer, &mut state);

    println!("{} moves (test): {:?}", answer.min_moves, answer.moves);
}
