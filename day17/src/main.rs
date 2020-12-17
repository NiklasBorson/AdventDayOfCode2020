use std::fs;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    const MAX_FRAME_COUNT : usize = 6;

    // Read the initial cells.
    let (width, cells) = read_file("input.txt")?;
    let height = cells.len() / width;

    // Allocate a 3d grid with enough margin around it that we won't
    // reach its boundaries in MAX_FRAME_COUNT frames.
    let margin = MAX_FRAME_COUNT + 1;
    let mut grid = Grid3::new(
        width + margin * 2,
        height + margin * 2,
        1 + margin * 2
    );

    // Set the initial active cells in the grid.
    let mut x = 0;
    let mut y = 0;
    for cell in cells {
        if cell {
            grid.set(x + margin, y + margin, margin, true);
        }

        // Advance (x,y) coordinate.
        x += 1;
        if x == width {
            x = 0;
            y += 1;
        }
    }

    // Execute the cellular automaton.
    for _i in 0..MAX_FRAME_COUNT {
        grid.next_frame();
    }

    // Output the final number of active cells.
    println!("{} active cells.", grid.count_active_cells());

    Ok(())
}

struct Grid3 {
    cells : Vec<bool>,
    next_cells : Vec<bool>,
    width : usize,
    height : usize,
    depth : usize
}

impl Grid3 {
    fn new(width : usize, height : usize, depth : usize) -> Grid3 {
        let mut cells = Vec::new();
        cells.resize(width * height * depth, false);
        let next_cells = cells.clone();
        Grid3{ cells, next_cells, width, height, depth }
    }

    fn index(&self, x : usize, y : usize, z : usize) -> usize {
        (((z * self.height) + y) * self.width) + x
    }

    fn set(&mut self, x : usize, y : usize, z : usize, value : bool) {
        let i = self.index(x, y, z);
        self.cells[i] = value;
    }

    fn test(&self, x : usize, y: usize, z : usize) -> bool {
        let i = self.index(x, y, z);
        self.cells[i]
    }

    fn next_frame(&mut self) {
        for z in 1..self.depth - 1 {
            for y in 1..self.height - 1 {
                for x in 1..self.width - 1 {
                    let i = self.index(x, y, z);
                    let is_active = self.cells[i];
                    let count = self.count_neighbors_and_self(x, y, z) - (is_active as usize);
                    self.next_cells[i] = match is_active {
                        true => count == 2 || count == 3,
                        false => count == 3
                    };
                }
            }
        }
        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    fn count_neighbors_and_self(&self, x : usize, y : usize, z : usize) -> usize {
        let mut count = 0;
        for z in z-1..z+2 {
            for y in y-1..y+2 {
                for x in x-1..x+2 {
                    count += self.test(x, y, z) as usize;
                }
            }
        }
        count
    }

    fn count_active_cells(&self) -> usize {
        let mut count = 0;
        for &cell in &self.cells {
            count += cell as usize;
        }
        count
    }
}

fn read_file(path: &str) -> std::io::Result<(usize, Vec::<bool>)> {
    let mut width = 0;
    let mut v = Vec::new();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        let s = line?;
        if s.is_empty() {
            break;
        }
        let mut col_width = 0;
        for ch in s.chars() {
            v.push(match ch {
                '#' => true,
                _ => false
            });
            col_width += 1;
        }
        if width == 0 {
            width = col_width;
        }
        else {
            assert!(col_width == width);
        }
    }
    Ok((width, v))
}
