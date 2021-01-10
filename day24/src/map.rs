use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Coord {
    x: usize,
    y: usize
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord {x, y }
    }

    pub fn new_relative(&self, xadd: isize, yadd: isize) -> Coord {
        Coord {
            x: (self.x as isize + xadd) as usize,
            y: (self.y as isize + yadd) as usize
        }
    }
}

#[derive(Debug)]
enum Block {
    Space,
    Wall,
}

#[derive(Default, Debug)]
pub struct Map {
    map: Vec<Vec<Block>>,
    width: u16,
    height: u16,
    items: BTreeMap<u8, Coord>,
    item_pos: HashMap<Coord, u8>
}

impl Map {
    pub fn parse(lines: &Vec<String>) -> Map {
        let mut map: Map = Default::default();

        for (y, l) in lines.iter().enumerate() {
            let row: Vec<Block> = l.chars().enumerate().map(|(x, c)| match c {
                '.' => Block::Space,
                '#' => Block::Wall,
                _ => {
                    let item = c as u8 - '0' as u8;

                    map.items.insert(item, Coord::new(x, y));
                    map.item_pos.insert(Coord::new(x, y), item);

                    Block::Space
                }
            }).collect();

            map.map.push(row);
        }

        map.height = map.map.len() as u16;
        map.width = map.map[0].len() as u16;

        map
    }

    pub fn draw(&self, frame_data: &mut[u8], frame_w: u16, block_size: u16, space_colour: u8, wall_colour: u8, item_colour: u8) {
        // Draw map on to frame
        
        let mut draw_block = |x, y, colour| {
            let mut outelemy = (y * block_size as usize * frame_w as usize) + (x * block_size as usize);

            for _ in 0..block_size {
                let mut outelemx = outelemy;

                for _ in 0..block_size {
                    frame_data[outelemx] = colour;
                    outelemx += 1;
                }

                outelemy += frame_w as usize;
            }
        };

        for (my, row) in self.map.iter().enumerate() {
            for (mx, block) in row.iter().enumerate() {
                let colour = match block {
                    Block::Space => space_colour,
                    Block::Wall => wall_colour
                };

                draw_block(mx, my, colour);
            }
        }

        // Draw items
        for (_, pos) in &self.items {
            draw_block(pos.x, pos.y, item_colour);
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn valid_move(&self, pos: &Coord) -> bool {
        match self.map[pos.y][pos.x] {
            Block::Space => true,
            _ => false
        }
    }

    pub fn items(&self) -> &BTreeMap<u8, Coord> {
        &self.items
    }

    pub fn item_check(&self, coord: &Coord) -> Option<&u8> {
        self.item_pos.get(coord)
    }
}
