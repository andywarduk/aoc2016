use std::collections::HashMap;

const KEY: &str = "cuanljph";

fn main() {
    println!("--- Part 1 ---");
    let part1 = calculate(KEY, plain_md5);

    println!("--- Part 2 ---");
    let part2 = calculate(KEY, stretched_md5);

    println!("--------------");

    println!("Answer to part 1: {}", part1);
    println!("Answer to part 2: {}", part2);
}

struct HashCacheEnt {
    num: usize,
}

type HashFn = fn(key: &str, n: usize) -> String;

struct HashCache<'a> {
    cache: HashMap<usize, String>, // Cache of hashes
    bytemap: HashMap<u8, Vec<HashCacheEnt>>, // Map of byte to 5 byte repeat hash positions
    calc_to: usize, // Upper bound of 5 byte map
    key: &'a str, // Hash key
    hashfn: HashFn // Hash function
}

impl<'a> HashCache<'a> {
    fn new(key: &'a str, hashfn: HashFn) -> HashCache {
        HashCache {
            cache: HashMap::new(),
            bytemap: HashMap::new(),
            calc_to: 0,
            key,
            hashfn
        }
    }

    fn calc(&mut self, n: usize) -> String {
        let result;

        if let Some(dstr) = self.cache.get(&n) {
            result = dstr.clone()
        } else {
            let dstr = (self.hashfn)(self.key, n);
            result = dstr.clone();
            self.cache.insert(n, dstr);
        }

        result
    }

    fn calc_to(&mut self, to: usize) {
        for n in self.calc_to + 1..to {
            let dstr = self.calc(n);

            if let Some(byte_vec) = contains_run(&dstr, 5) {
                for byte in byte_vec {
                    let ent = HashCacheEnt {
                        num: n
                    };
        
                    if let Some(vec) = self.bytemap.get_mut(&byte) {
                        vec.push(ent);
                    } else {
                        self.bytemap.insert(byte, vec![ent]);
                    }
                }
            }
        }

        self.calc_to = to;
    }

    fn check(&mut self, byte: u8, from: usize, count: usize) -> Option<usize> {
        self.calc_to(from + 1 + count);

        if let Some(vec) = self.bytemap.get(&byte) {
            for ent in vec {
                if ent.num > from && ent.num <= from + count + 1 {
                    return Some(ent.num);
                }
            }
        }

        None
    }
}

fn calculate(key: &str, hashfn: HashFn) -> usize {
    let mut key_no = 0;
    let mut hashcache = HashCache::new(key, hashfn);

    for n in 0.. {
        let dstr = hashcache.calc(n);

        if let Some(byte_vec) = contains_run(&dstr, 3) {
            if let Some(next) = hashcache.check(byte_vec[0], n, 1000) {
                key_no += 1;
                println!("Found key {} at position {} (pair at {}, dist {})", key_no, n, next, next - n);
                if key_no == 64 {
                    return n
                }
            }
        }
    }

    0
}

fn contains_run(string: &str, num: usize) -> Option<Vec<u8>> {
    let mut result: Option<Vec<u8>> = None;
    let bytes = string.as_bytes();

    for i in 0..=bytes.len() - num {
        let mut run: bool = true;

        for j in i + 1..i + num {
            if bytes[j] != bytes[i] {
                run = false;
                break
            }
        }

        if run && (i + num >= bytes.len() || bytes[i + num] != bytes[i]) {
            // Got one
            if let Some(mut vec) = result {
                vec.push(bytes[i]);
                result = Some(vec)
            } else {
                result = Some(vec![bytes[i]])
            }
        }
    }

    result
}

fn plain_md5(key: &str, n: usize) -> String {
    let digest = md5::compute(format!("{}{}", key, n));

    format!("{:x}", digest)
}

fn stretched_md5(key: &str, n: usize) -> String {
    let mut dstr = plain_md5(key, n);

    for _ in 0..2016 {
        let digest = md5::compute(dstr);

        dstr = format!("{:x}", digest);
    }

    dstr
}

#[test]
fn test_example() {
    assert!(calculate("abc", plain_md5) == 22728);
}

#[test]
fn test_stretched_md5() {
    let dstr = stretched_md5("abc", 0);
    assert!(dstr == "a107ff634856bb300138cac6568c0f24");
}
