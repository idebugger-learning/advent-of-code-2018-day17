use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Duration;
use crate::parser::Rows;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Clay,
    Sand,
    Water,
}

pub struct Map {
    tiles: Vec<Tile>,
    min_x: usize,
    width: usize,
    min_y: usize,
    max_y: usize,
    queue: VecDeque<(usize, usize)>,
    backtrack: VecDeque<(usize, usize)>,
}

impl Map {
    pub fn new(rows: &Rows) -> Map {
        let min_x = rows.y.iter()
            .map(|row| row.from)
            .chain(rows.x.iter().map(|row| row.line))
            .min().unwrap() - 2;
        let max_x = rows.y.iter()
            .map(|row| row.to)
            .chain(rows.x.iter().map(|row| row.line))
            .max().unwrap() + 2;
        let min_y = rows.x.iter()
            .map(|row| row.from)
            .chain(rows.y.iter().map(|row| row.line))
            .min().unwrap();
        let max_y = rows.x.iter()
            .map(|row| row.to)
            .chain(rows.y.iter().map(|row| row.line))
            .max().unwrap();

        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        let mut tiles = vec![Tile::Sand; width * height];

        for row in &rows.x {
            for y in row.from..=row.to {
                let index = ((y - min_y) * width) + (row.line - min_x);
                tiles[index] = Tile::Clay;
            }
        }

        for row in &rows.y {
            for x in row.from..=row.to {
                let index = ((row.line - min_y) * width) + (x - min_x);
                tiles[index] = Tile::Clay;
            }
        }

        tiles[500 - min_x] = Tile::Water;

        Map {
            tiles,
            min_x,
            width,
            min_y,
            max_y,
            queue: VecDeque::from(vec![(500, min_y)]),
            backtrack: VecDeque::new(),
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        ((y - self.min_y) * self.width) + (x - self.min_x)
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        let index = self.get_index(x, y) as usize;
        self.tiles[index]
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        let index = self.get_index(x, y) as usize;
        self.tiles[index] = tile;
    }

    pub fn tick(&mut self) -> bool {
        if !self.queue.is_empty() {
            self.tick_flow();
            return !self.queue.is_empty() || !self.backtrack.is_empty();
        }

        if !self.backtrack.is_empty() {
            self.tick_backtrack();
            return !self.queue.is_empty() || !self.backtrack.is_empty();
        }

        return !self.queue.is_empty() || !self.backtrack.is_empty();
    }

    fn tick_flow(&mut self) {
        let Some((x, y)) = self.queue.pop_front() else {
            unreachable!("tick_flow called with empty queue");
        };

        if y > self.max_y {
            return;
        }

        if y < self.max_y {
            let below = self.get(x, y + 1);
            if matches!(below, Tile::Sand | Tile::Water) {
                self.set(x, y + 1, Tile::Water);
                self.queue.push_back((x, y + 1));
                self.backtrack.push_back((x, y));
                return;
            }

            if matches!(below, Tile::Clay) {
                if self.get(x - 1, y) == Tile::Sand {
                    self.set(x - 1, y, Tile::Water);
                    self.queue.push_back((x - 1, y));
                }

                if self.get(x + 1, y) == Tile::Sand {
                    self.set(x + 1, y, Tile::Water);
                    self.queue.push_back((x + 1, y));
                }

                return;
            }
        }
    }

    fn tick_backtrack(&mut self) {
        let Some((x, y)) = self.backtrack.pop_back() else {
            unreachable!("tick_backtrack called with empty backtrack");
        };

        if y >= self.max_y || y <= self.min_y || x <= self.min_x || x >= self.min_x + self.width - 1 {
            return;
        }

        if self.is_full_waterline(x, y + 1) && self.get(x - 1, y) != Tile::Clay {
            let mut left = x;
            while self.get(left, y + 1) != Tile::Clay && self.get(left, y) != Tile::Clay && left > self.min_x {
                self.set(left, y, Tile::Water);
                left -= 1;
            }
            if self.get(left, y) != Tile::Clay {
                self.set(left, y, Tile::Water);
                self.queue.push_back((left, y));
            }
        }

        if self.is_full_waterline(x, y + 1) && self.get(x + 1, y) != Tile::Clay {
            let mut right = x;
            while self.get(right, y + 1) != Tile::Clay && self.get(right, y) != Tile::Clay && right < self.min_x + self.width - 1 {
                self.set(right, y, Tile::Water);
                right += 1;
            }
            if self.get(right, y) != Tile::Clay {
                self.set(right, y, Tile::Water);
                self.queue.push_back((right, y));
            }
        }

        if !self.queue.is_empty() {
            self.tick_flow();
        }
    }

    fn is_full_waterline(&self, x: usize, y: usize) -> bool {
        let mut left = x;
        while self.get(left, y) == Tile::Water && left > self.min_x {
            left -= 1;
        }
        if !matches!(self.get(left, y), Tile::Clay | Tile::Water) {
            return false;
        }

        let mut right = x;
        while self.get(right, y) == Tile::Water && right < self.min_x + self.width - 1 {
            right += 1;
        }
        if !matches!(self.get(right, y), Tile::Clay | Tile::Water) {
            return false;
        }

        return true;
    }

    pub fn count_water(&self) -> usize {
        let mut count = 0;
        for y in self.tiles.iter() {
            if matches!(y, Tile::Water) {
                count += 1;
            }
        }
        count
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "     ")?;
        for x in self.min_x..(self.min_x + self.width) {
            write!(f, "{}", x / 100)?;
        }
        write!(f, "\n     ")?;
        for x in self.min_x..(self.min_x + self.width) {
            write!(f, "{}", (x / 10) % 10)?;
        }
        write!(f, "\n     ")?;
        for x in self.min_x..(self.min_x + self.width) {
            write!(f, "{}", x % 10)?;
        }
        write!(f, "\n     ")?;
        for x in self.min_x..(self.min_x + self.width) {
            if x == 500 {
                write!(f, "+")?;
            } else {
                write!(f, " ")?;
            }
        }
        writeln!(f)?;
        for y in self.min_y..=self.max_y {
            write!(f, "{:4} ", y)?;
            for x in self.min_x..(self.min_x + self.width) {
                let tile = self.get(x, y);
                let c = match tile {
                    Tile::Clay => '#',
                    Tile::Sand => ' ',
                    Tile::Water => '~',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}