use std::{borrow::Cow, cmp, collections::{HashSet, VecDeque}, fs::File, io::{BufRead, BufReader}};
use gif::{Encoder, Frame};
use memmap2::Mmap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input22.txt")?;

    let servers = parse_servers(&lines);

    part1(&servers);

    part2(servers);

    Ok(())
}

fn part1(servers: &[Server]) {
    let mut pairs: HashSet<String> = HashSet::new();

    for (a_idx, a) in servers.iter().enumerate() {
        for (b_idx, b) in servers.iter().enumerate() {
            if a_idx == b_idx {
                continue
            }

            if a.used == 0 {
                continue
            }

            if a.used > b.size - b.used {
                continue
            } 

            pairs.insert(format!("{}-{}", cmp::min(a_idx, b_idx), cmp::max(a_idx, b_idx)));
        }
    }

    println!("{} viable pairs (part 1)", pairs.len())
}

const SQUARE_SPACING: u16 = 3;

fn part2(servers: Vec<Server>) {
    let max_size = servers.iter().map(|s| s.size).max().unwrap();
    let mut map = build_map(servers);

    // Calculate GIF sizes
    let gif_square = square_size(max_size);
    let gif_w = (map.width() * (gif_square + SQUARE_SPACING)) + SQUARE_SPACING;
    let gif_h = (map.height() * (gif_square + SQUARE_SPACING)) + SQUARE_SPACING;

    // Start GIF
    let mut image = File::create("output22.gif").unwrap();
    let color_map = &[0, 0, 0,  0x00, 0x00, 0xff,  0x00, 0xa0, 0x00,  0xff, 0xff, 0xff];
    let mut encoder = Encoder::new(&mut image, gif_w, gif_h, color_map).unwrap();

    // Draw initial map
    map.draw(&mut encoder, gif_w, gif_h, gif_square, SQUARE_SPACING);

    // Set up goal coords
    let mut goal = (map.width() - 1, 0u16);

    // Build goal path
    let mut goal_path = Vec::new();
    for x in 0..map.width() - 1 {
        goal_path.push((x, 0u16));
    }

    while let Some(next_goal) = goal_path.pop() {
        // Calculate shortest path to next goal path
        let space_path = shortest_path(&map, next_goal, goal);

        for mv in space_path {
            map.move_space(mv);

            // Draw frame
            map.draw(&mut encoder, gif_w, gif_h, gif_square, SQUARE_SPACING);
        }

        // Move goal to space
        map.move_space(goal);

        // Draw frame
        map.draw(&mut encoder, gif_w, gif_h, gif_square, SQUARE_SPACING);

        // Goal is now next goal
        goal = next_goal;
    }

    println!("Number of moves (part 2): {}", map.moves);
}

fn shortest_path(map: &Map, to: (u16, u16), avoid: (u16, u16)) -> Vec<(u16, u16)> {
    #[derive(Clone)]
    struct State {
        pos: (u16, u16),
        path: Vec<(u16, u16)>
    }

    let mut visited: HashSet<(u16, u16)> = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(State {
        pos: map.empty,
        path: Vec::new()
    });

    let mut path = Vec::new();

    while let Some(work_item) = queue.pop_front() {
        if work_item.pos.0 == to.0 && work_item.pos.1 == to.1 {
            path = work_item.path;
            break
        }

        let mut move_to = |next: (u16, u16)| {
            if next.0 == avoid.0 && next.1 == avoid.1 {
                return
            }

            if visited.get(&next).is_some() {
                return
            }

            visited.insert(next);

            if !map.valid_move(&next, &(work_item.pos.0, work_item.pos.1)) {
                return
            }

            let mut new_path = work_item.path.clone();
            new_path.push(next);

            queue.push_back(State {
                pos: next,
                path: new_path
            });
        };

        if work_item.pos.0 > 0 {
            move_to((work_item.pos.0 - 1, work_item.pos.1));
        }
        if work_item.pos.0 < map.map[0].len() as u16 - 1 {
            move_to((work_item.pos.0 + 1, work_item.pos.1));
        }
        if work_item.pos.1 > 0 {
            move_to((work_item.pos.0, work_item.pos.1 - 1));
        }
        if work_item.pos.1 < map.map.len() as u16 - 1 {
            move_to((work_item.pos.0, work_item.pos.1 + 1));
        }
    }

    path
}

#[derive(Debug)]
struct Server {
    x: u16,
    y: u16,
    size: u16,
    used: u16,
}

impl Server {
    fn parse(line: &str) -> Server {
        let mut terms = line.split_whitespace();

        let dev = terms.next().unwrap();
        let size_s = terms.next().unwrap();
        let used_s = terms.next().unwrap();

        let mut dev_split = dev.split('-');
        let xs = dev_split.nth(1).unwrap();
        let ys = dev_split.next().unwrap();

        let x = xs[1..].parse::<u16>().unwrap();
        let y = ys[1..].parse::<u16>().unwrap();

        let size = size_s[..size_s.len() - 1].parse::<u16>().unwrap();
        let used = used_s[..used_s.len() - 1].parse::<u16>().unwrap();

        Server { x, y, size, used }
    }
}

struct Map {
    map: Vec<Vec<Server>>,
    empty: (u16, u16),
    moves: u16
}

impl Map {
    fn width(&self) -> u16 {
        self.map[0].len() as u16
    }

    fn height(&self) -> u16 {
        self.map.len() as u16
    }

    fn valid_move(&self, from: &(u16, u16), to: &(u16, u16)) -> bool {
        // Assert adjacent
        assert!(to.1 != from.1 || to.0 == from.0 + 1 || to.0 == from.0 - 1);
        assert!(to.0 != from.0 || to.1 == from.1 + 1 || to.1 == from.1 - 1);

        // Check space
        self.map[to.1 as usize][to.0 as usize].size > self.map[from.1 as usize][from.0 as usize].used
    }

    fn move_space(&mut self, from: (u16, u16)) {
        assert!(self.valid_move(&from, &self.empty));
        assert!(self.map[self.empty.1 as usize][self.empty.0 as usize].used == 0);

        // Move data
        self.map[self.empty.1 as usize][self.empty.0 as usize].used = self.map[from.1 as usize][from.0 as usize].used;
        self.map[from.1 as usize][from.0 as usize].used = 0;
        self.empty = from;

        self.moves += 1;
    }

    fn draw(&self, encoder: &mut Encoder<&mut File>, gif_w: u16, gif_h: u16, square: u16, border: u16) {
        let frame_size = gif_w as usize * gif_h as usize;
        let mut frame_data = vec![0; frame_size];

        let draw_rect = |frame_data: &mut Vec<u8>, x, y, sx, sy, colour| {
            for sy in 0..sy {
                let mut frame_pos = ((y + sy) as usize * gif_w as usize) + x as usize;

                for _ in 0..sx {
                    frame_data[frame_pos] = colour;
                    frame_pos += 1;
                }
            }
        };

        let draw_box = |frame_data: &mut Vec<u8>, x, y, sx, sy, colour| {
            let mut frame_pos;

            frame_pos = (y as usize * gif_w as usize) + x as usize;
            for _ in 0..=sy {
                frame_data[frame_pos] = colour;
                frame_data[frame_pos + sx as usize] = colour;
                frame_pos += gif_w as usize;
            }

            frame_pos = (y as usize * gif_w as usize) + x as usize;
            for _ in 0..=sx {
                frame_data[frame_pos] = colour;
                frame_data[frame_pos + (sy as usize * gif_w as usize)] = colour;
                frame_pos += 1;
            }
        };

        for (y, row) in self.map.iter().enumerate() {
            let outy = border + (y as u16 * (square + border));

            for (x, server) in row.iter().enumerate() {
                let outx = border + (x as u16 * (square + border));

                let size_size = square_size(server.size);
                let offset = (square - size_size) / 2;

                draw_box(&mut frame_data, outx + offset - 1, outy + offset - 1, size_size + 1, size_size + 1, 3);
                draw_rect(&mut frame_data, outx + offset, outy + offset, size_size, size_size, 1);
                draw_rect(&mut frame_data, outx + offset, outy + offset, (server.used * size_size) / server.size, size_size, 2);
            }
        }

        // Write frame
        let frame = Frame {
            delay: 10,
            width: gif_w,
            height: gif_h,
            buffer: Cow::Borrowed(&frame_data),
            ..Frame::default()
        };

        encoder.write_frame(&frame).unwrap();    
    }
}

fn build_map(servers: Vec<Server>) -> Map {
    let mut map = Map {
        map: Vec::new(),
        empty: (0, 0),
        moves: 0
    };

    for s in servers {
        if s.y >= map.map.len() as u16 {
            assert!(map.map.len() as u16 == s.y);
            map.map.push(Vec::new());
        }

        assert!(map.map[s.y as usize].len() as u16 == s.x);

        if s.used == 0 {
            map.empty = (s.x, s.y)
        }

        map.map[s.y as usize].push(s);
    }

    map
}

fn square_size(val: u16) -> u16 {
    let root = (val as f64).sqrt();
    root.ceil() as u16
}

fn parse_servers(lines: &[String]) -> Vec<Server> {
    lines.iter().map(|s| Server::parse(s)).collect()
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

    // Create lines vector
    let mut lines = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if line.starts_with("/dev/") {
            lines.push(line);
        }
    }

    Ok(lines)
}
