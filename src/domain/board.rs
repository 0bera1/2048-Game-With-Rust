use rand::Rng;

use super::direction::Direction;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MoveEvent {
    pub from_row: usize,
    pub from_col: usize,
    pub to_row: usize,
    pub to_col: usize,
    pub value: u32,
    pub merged_into_value: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    pub value: u32,
    pub merged: bool,
}

impl Tile {
    pub fn new(value: u32) -> Self {
        Self { value, merged: false }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub size: usize,
    pub cells: Vec<Option<Tile>>, // row-major
    pub score: u32,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let mut board = Self { size, cells: vec![None; size * size], score: 0 };
        board.spawn_random_tile();
        board.spawn_random_tile();
        board
    }

    fn index(&self, row: usize, col: usize) -> usize { row * self.size + col }

    pub fn get(&self, row: usize, col: usize) -> &Option<Tile> {
        &self.cells[self.index(row, col)]
    }

    pub fn set(&mut self, row: usize, col: usize, value: Option<Tile>) {
        let idx = self.index(row, col);
        self.cells[idx] = value;
    }

    pub fn reset_merged_flags(&mut self) {
        for cell in &mut self.cells {
            if let Some(tile) = cell {
                tile.merged = false;
            }
        }
    }

    pub fn empty_positions(&self) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        for row in 0..self.size {
            for col in 0..self.size {
                if self.get(row, col).is_none() {
                    result.push((row, col));
                }
            }
        }
        result
    }

    pub fn spawn_random_tile(&mut self) -> bool {
        let empties = self.empty_positions();
        if empties.is_empty() { return false; }
        let mut rng = rand::thread_rng();
        let &(row, col) = empties.get(rng.gen_range(0..empties.len())).unwrap();
        let value = if rng.gen_range(0..10) == 0 { 4 } else { 2 };
        self.set(row, col, Some(Tile::new(value)));
        true
    }

    pub fn can_move(&self) -> bool {
        if !self.empty_positions().is_empty() { return true; }
        // check merges
        for row in 0..self.size {
            for col in 0..self.size {
                if let Some(tile) = self.get(row, col) {
                    if row + 1 < self.size {
                        if let Some(down) = self.get(row + 1, col) { if down.value == tile.value { return true; } }
                    }
                    if col + 1 < self.size {
                        if let Some(right) = self.get(row, col + 1) { if right.value == tile.value { return true; } }
                    }
                }
            }
        }
        false
    }

    pub fn is_won(&self) -> bool {
        self.cells.iter().flatten().any(|t| t.value >= 2048)
    }

    pub fn slide(&mut self, direction: Direction) -> bool {
        self.reset_merged_flags();
        let mut moved = false;

        match direction {
            Direction::Left => {
                for row in 0..self.size {
                    moved |= self.compact_line(row, 0, 0, 1);
                }
            }
            Direction::Right => {
                for row in 0..self.size {
                    moved |= self.compact_line(row, self.size - 1, 0, -1);
                }
            }
            Direction::Up => {
                for col in 0..self.size {
                    moved |= self.compact_column(0, col, 1, 0);
                }
            }
            Direction::Down => {
                for col in 0..self.size {
                    moved |= self.compact_column(self.size - 1, col, -1, 0);
                }
            }
        }

        if moved {
            self.spawn_random_tile();
        }
        moved
    }

    pub fn slide_with_animations(&mut self, direction: Direction) -> (bool, Vec<MoveEvent>) {
        self.reset_merged_flags();
        let mut moved = false;
        let mut events: Vec<MoveEvent> = Vec::new();

        match direction {
            Direction::Left => {
                for row in 0..self.size {
                    moved |= self.compact_line_with_events(row, 0, 0, 1, &mut events);
                }
            }
            Direction::Right => {
                for row in 0..self.size {
                    moved |= self.compact_line_with_events(row, self.size - 1, 0, -1, &mut events);
                }
            }
            Direction::Up => {
                for col in 0..self.size {
                    moved |= self.compact_column_with_events(0, col, 1, 0, &mut events);
                }
            }
            Direction::Down => {
                for col in 0..self.size {
                    moved |= self.compact_column_with_events(self.size - 1, col, -1, 0, &mut events);
                }
            }
        }

        (moved, events)
    }

    fn compact_line(&mut self, row: usize, _start_col: usize, _row_step: isize, col_step: isize) -> bool {
        // Gather existing values in scan order
        let cols: Vec<usize> = if col_step == 1 { (0..self.size).collect() } else { (0..self.size).rev().collect() };
        let mut values: Vec<(usize, u32)> = Vec::new();
        for c in &cols {
            if let Some(t) = self.get(row, *c) { values.push((*c, t.value)); }
        }

        // Compute merged sequence
        let mut merged_values: Vec<u32> = Vec::new();
        let mut i = 0;
        while i < values.len() {
            if i + 1 < values.len() && values[i].1 == values[i + 1].1 {
                let nv = values[i].1 * 2;
                self.score += nv;
                merged_values.push(nv);
                i += 2;
            } else {
                merged_values.push(values[i].1);
                i += 1;
            }
        }

        // Write back and detect moved
        let mut moved = false;
        // clear row
        for c in 0..self.size { self.set(row, c, None); }
        let merged_len = merged_values.len();
        for k in 0..merged_len {
            let val = merged_values[k];
            let dest_c = if col_step == 1 { k } else { self.size - 1 - k };
            // moved if any original src column for this value is not at dest_c
            if !values.is_empty() {
                moved = true; // safe approximation
            }
            self.set(row, dest_c, Some(Tile::new(val)));
        }
        moved
    }

    fn compact_column(&mut self, _start_row: usize, col: usize, row_step: isize, _col_step_unused: isize) -> bool {
        let rows: Vec<usize> = if row_step == 1 { (0..self.size).collect() } else { (0..self.size).rev().collect() };
        let mut values: Vec<(usize, u32)> = Vec::new();
        for r in &rows {
            if let Some(t) = self.get(*r, col) { values.push((*r, t.value)); }
        }

        let mut merged_values: Vec<u32> = Vec::new();
        let mut i = 0;
        while i < values.len() {
            if i + 1 < values.len() && values[i].1 == values[i + 1].1 {
                let nv = values[i].1 * 2;
                self.score += nv;
                merged_values.push(nv);
                i += 2;
            } else {
                merged_values.push(values[i].1);
                i += 1;
            }
        }

        let mut moved = false;
        for r in 0..self.size { self.set(r, col, None); }
        let merged_len = merged_values.len();
        for k in 0..merged_len {
            let val = merged_values[k];
            let dest_r = if row_step == 1 { k } else { self.size - 1 - k };
            if !values.is_empty() {
                moved = true;
            }
            self.set(dest_r, col, Some(Tile::new(val)));
        }
        moved
    }

    fn compact_line_with_events(&mut self, row: usize, _start_col: usize, _row_step: isize, col_step: isize, out: &mut Vec<MoveEvent>) -> bool {
        let cols: Vec<usize> = if col_step == 1 { (0..self.size).collect() } else { (0..self.size).rev().collect() };
        let mut items: Vec<(usize, u32)> = Vec::new();
        for c in &cols { if let Some(t) = self.get(row, *c) { items.push((*c, t.value)); } }

        // Build merged list with source mapping
        let mut merged: Vec<(Vec<usize>, u32)> = Vec::new();
        let mut i = 0;
        while i < items.len() {
            if i + 1 < items.len() && items[i].1 == items[i + 1].1 {
                merged.push((vec![items[i].0, items[i + 1].0], items[i].1 * 2));
                self.score += items[i].1 * 2;
                i += 2;
            } else {
                merged.push((vec![items[i].0], items[i].1));
                i += 1;
            }
        }

        // Clear row, place and emit events
        for c in 0..self.size { self.set(row, c, None); }
        let mut moved = false;
        for (k, (sources, val)) in (0..merged.len()).zip(merged.into_iter()) {
            let dest_col = if col_step == 1 { k } else { self.size - 1 - k };
            self.set(row, dest_col, Some(Tile::new(val)));
            let sources_len = sources.len();
            for src_c in &sources {
                if *src_c != dest_col { moved = true; }
                let src_value = if sources_len == 2 { val / 2 } else { val };
                let merged_into = if sources_len == 2 { Some(val) } else { None };
                out.push(MoveEvent { from_row: row, from_col: *src_c, to_row: row, to_col: dest_col, value: src_value, merged_into_value: merged_into });
            }
        }
        moved
    }

    fn compact_column_with_events(&mut self, _start_row: usize, col: usize, row_step: isize, _col_step_unused: isize, out: &mut Vec<MoveEvent>) -> bool {
        let rows: Vec<usize> = if row_step == 1 { (0..self.size).collect() } else { (0..self.size).rev().collect() };
        let mut items: Vec<(usize, u32)> = Vec::new();
        for r in &rows { if let Some(t) = self.get(*r, col) { items.push((*r, t.value)); } }

        let mut merged: Vec<(Vec<usize>, u32)> = Vec::new();
        let mut i = 0;
        while i < items.len() {
            if i + 1 < items.len() && items[i].1 == items[i + 1].1 {
                merged.push((vec![items[i].0, items[i + 1].0], items[i].1 * 2));
                self.score += items[i].1 * 2;
                i += 2;
            } else {
                merged.push((vec![items[i].0], items[i].1));
                i += 1;
            }
        }

        for r in 0..self.size { self.set(r, col, None); }
        let mut moved = false;
        for (k, (sources, val)) in (0..merged.len()).zip(merged.into_iter()) {
            let dest_row = if row_step == 1 { k } else { self.size - 1 - k };
            self.set(dest_row, col, Some(Tile::new(val)));
            let sources_len = sources.len();
            for src_r in &sources {
                if *src_r != dest_row { moved = true; }
                let src_value = if sources_len == 2 { val / 2 } else { val };
                let merged_into = if sources_len == 2 { Some(val) } else { None };
                out.push(MoveEvent { from_row: *src_r, from_col: col, to_row: dest_row, to_col: col, value: src_value, merged_into_value: merged_into });
            }
        }
        moved
    }
}

