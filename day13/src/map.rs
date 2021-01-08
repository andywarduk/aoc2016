use std::fmt;

#[derive(Debug)]
enum Block {
    Space,
    Wall
}

#[derive(Default)]
pub struct Map {
    map: Vec<Vec<Block>>
}

impl Map {
    pub fn generate(seed: u16, dim: usize) -> Map {
        let mut map: Map = Default::default();

        for y in 0..dim {
            let mut row = Vec::new();

            for x in 0..dim {
                let mut calc = (x * x) + (3 * x) + (2 * x * y) + y + (y * y);

                calc += seed as usize;

                if calc.count_ones() % 2 == 0 {
                    row.push(Block::Space)
                } else {
                    row.push(Block::Wall)
                }
            }

            map.map.push(row);
        }

        map
    }

    pub fn movable(&self, x: usize, y: usize) -> bool {
        match self.map[y][x] {
            Block::Space => true,
            Block::Wall => false
        }
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\n")?;
        for row in self.map.iter() {
            let row_str = row.iter().map(|b| match b {
                Block::Space => '.',
                Block::Wall => '#'
            }).collect::<String>();
            f.write_str(&row_str)?;
            f.write_str("\n")?;
        }

        Ok(())
    }
}
