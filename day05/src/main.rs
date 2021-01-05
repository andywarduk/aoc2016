use md5::Digest;

const KEY: &str = "reyedfim";

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut chars = Vec::new();

    for n in 0.. {
        let digest = md5::compute(format!("{}{}", KEY, n));

        if check5(&digest) {
            let charnum = digest[2] & 0x0f;

            let c = if charnum < 0xa {
                (charnum + '0' as u8) as char
            } else {
                ((charnum - 10) + 'a' as u8) as char
            };

            println!("Password character {} from seed {} is {}", chars.len() + 1, n, c);

            chars.push(c);
            if chars.len() == 8 {
                break
            }
        }
    }

    let password: String = chars.iter().collect();

    println!("Password (part 1): {}", password);
}

fn part2() {
    let mut chars: [char; 8] = [' '; 8];
    let mut found = 0;

    for n in 0.. {
        let digest = md5::compute(format!("{}{}", KEY, n));

        if check5(&digest) {
            let charpos = digest[2] & 0x0f;

            if charpos > 7 || chars[charpos as usize] != ' ' {
                continue
            }

            let charnum = (digest[3] & 0xf0) >> 4;

            let c = if charnum < 0xa {
                (charnum + '0' as u8) as char
            } else {
                ((charnum - 10) + 'a' as u8) as char
            };

            println!("Password character {} from seed {} is {}", charpos, n, c);

            chars[charpos as usize] = c;

            found += 1;

            if found == 8 {
                break
            }
        }
    }

    let password: String = chars.iter().collect();

    println!("Password (part 2): {}", password);
}

#[inline]
fn check5(digest: &Digest) -> bool{
    digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0
}
