use std::{collections::VecDeque, mem};

// Part 1 solution based on https://en.wikipedia.org/wiki/Josephus_problem
// Part 2 based on aceshades python solution

const ELVES: usize = 3014387;

fn main() {
    println!("Winner out of {} elves (part 1): {}", ELVES, winner_fast(ELVES));
    println!("Winner out of {} elves (part 2): {}", ELVES, winner_opposite(ELVES));
}

fn winner_fast(elves: usize) -> usize {
    // Remove most significant bit and shift a 1 in at the bottom
    let next_power = 1 << (((mem::size_of::<usize>() * 8) - 1) - elves.leading_zeros() as usize);
    ((elves & !next_power) << 1) | 1
}

fn winner_opposite(elves: usize) -> usize {
    let mut left = VecDeque::with_capacity((elves / 2) + 1);
    let mut right = VecDeque::with_capacity((elves / 2) + 1);

    for i in 0..elves {
        if i < elves / 2 {
            left.push_back(i + 1);
        } else {
            right.push_front(i + 1);
        }
    }

    loop {
        let llen = left.len();
        let rlen = right.len();

        if llen + rlen <= 1 {
            break
        }

        // Steal
        if llen > rlen {
            left.pop_back().unwrap();
        } else {
            right.pop_back().unwrap();
        }
    
        // Rotate
        right.push_front(left.pop_front().unwrap());
        left.push_back(right.pop_back().unwrap());
    }

    if let Some(winner) = left.pop_front() {
        winner
    } else {
        right.pop_front().unwrap()
    }
}

#[test]
fn test_winner_fast() {
    for i in 2..=100 {
        assert!(winner_fast(i) == winner(i), "Test for {} failed", i);
    }
}

#[test]
fn test_winner_opposite() {
    assert!(winner_opposite(5) == 2);
    assert!(winner_opposite(6) == 3);
    assert!(winner_opposite(7) == 5);
}

#[cfg(test)]
fn winner(elves: usize) -> usize {
    let mut presents = Vec::new();

    for _ in 0..elves {
        presents.push(1);
    }

    let mut turn = 0;

    loop {
        if presents[turn] > 0 {
            // Steal from who?
            if let Some(steal) = steal_from(&presents, turn, 1) {
                presents[turn] += presents[steal];
                presents[steal] = 0;
            } else {
                // Found winner
                break turn + 1
            }
        }

        turn += 1;
        if turn >= elves {
            turn = 0;
        }
    }
}

#[cfg(test)]
fn steal_from(presents: &Vec<usize>, turn: usize, skip: usize) -> Option<usize> {
    let mut steal = turn;
    let mut left = skip;

    loop {
        steal = (steal + 1) % presents.len();

        if steal == turn {
            // No other players left
            return None
        }

        if presents[steal] != 0 {
            left -= 1;

            if left == 0 {
                break
            }
        }
    }

    Some(steal)
}
