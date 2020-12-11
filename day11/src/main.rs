use std::fs;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Space,
    Empty,
    Full
}

fn main() -> std::io::Result<()> {

    // Read the file into a vector of i32.
    let (input, width) = read_cells("day11-input.txt")?;

    println!("Pass 1 count = {}", run(&input, width, next_frame1));
    println!("Pass 2 count = {}", run(&input, width, next_frame2));

    Ok(())
}

fn run<T>(input: &[Cell], width: usize, func: T) -> usize
where T: Fn(&[Cell], usize, usize, &mut[Cell]) {
    let height = input.len() / width;
    let mut current = Vec::from(input);
    let mut next = current.clone();

    // Compute the initial frame + 1.
    func(&current, width, height, &mut next);

    while next != current {
        std::mem::swap(&mut current, &mut next);
        func(&current, width, height, &mut next);
    }

    count_full_cells(&current)    
}

fn count_full_cells(v : &[Cell]) -> usize {
    v.iter()
        .filter(|&cell| *cell == Cell::Full)
        .count()
}

fn next_frame1(v : &[Cell], width : usize, height : usize, next : &mut[Cell]) {
    assert_eq!(width * height, v.len());
    for y in 0..height {
        // Array index at start of this row.
        let i = y * width;

        // Slices representing the current row and the rows above and below.
        // Above and below are empty if this is the top or bottom row.
        let row = &v[i..i + width];
        let above = if y > 0 { &v[i - width..i] } else { &v[0..0] };
        let below = if y + 1 < height { &v[i + width..i + width + width] } else { &v[0..0] };

        // Slice representing the current destination row.
        let dest = &mut next[i..i + width];

        // For each cell in the row.
        for x in 0..width {

            // Range of x coordinates including the current cell and its
            // immediate neighbors, clamped to the row boundary.
            let left = if x > 0 { x - 1 } else { x };
            let right = if x + 1 < width { x + 2 } else { x + 1 };

            // Count the full cells.
            // Note: if the current cell is full, it is counted as well.
            let mut count = count_full_cells(&row[left..right]);
            if !above.is_empty() {
                count += count_full_cells(&above[left..right]);
            }
            if !below.is_empty() {
                count += count_full_cells(&below[left..right]);
            }

            // Apply the rule to determine the next value.
            // Note, if the current cell is full, count count is the
            // number of full neighbors plus one.
            dest[x] = match row[x] {
                Cell::Space => Cell::Space,
                Cell::Empty => if count == 0 { Cell::Full } else { Cell::Empty },
                Cell::Full => if count < 5 { Cell::Full } else { Cell::Empty }
            };
        }
    }
}

fn get_cell_index(width : usize, x : isize, y : isize) -> usize {
    (y as usize) * width + (x as usize)
}

fn next_frame2(v : &[Cell], width : usize, height : usize, next : &mut[Cell]) {
    assert_eq!(width * height, v.len());

    // Define 2D vectors for each of the eight directions.
    let vecs = [ 
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1)
    ];

    // Signed width and height.
    let w = width as isize;
    let h = height as isize;

    // Iterate over all the cells by row and then by column.
    for y in 0..h {
        for x in 0..w {
            let mut count = 0;
            for &(dx, dy) in &vecs {
                let mut x1 = x + dx;
                let mut y1 = y + dy;
                while x1 >= 0 && x1 < w && y1 >= 0 && y1 < h {
                    let cell = v[get_cell_index(width, x1, y1)];
                    if cell != Cell::Space {
                        if cell == Cell::Full { count += 1; }
                        break;
                    }
                    x1 += dx;
                    y1 += dy;
                }
            }
            let i = get_cell_index(width, x, y);

            next[i] = match v[i] {
                Cell::Space => Cell::Space,
                Cell::Empty => if count == 0 { Cell::Full } else { Cell::Empty },
                Cell::Full => if count < 4 { Cell::Full } else { Cell::Empty }
            };
        }
    }
}

fn make_error(message: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, message)
} 

fn read_cells(path: &str) -> std::io::Result<(Vec::<Cell>, usize)> {
    let mut v = Vec::new();
    let mut width = 0;
    for line in BufReader::new(fs::File::open(path)?).lines() {
        let size = v.len();

        for ch in line?.chars() {
            v.push(match ch {
                '.' => Cell::Space,
                'L' => Cell::Empty,
                '#' => Cell::Full,
                _ => { return Err(make_error("invalid character.")); }
            });
        }

        if width == 0 {
            width = v.len();
        }
        else if v.len() != size && v.len() != size + width {
            return Err(make_error("mismatched line widths."));
        }
    }
    Ok((v, width))
}
