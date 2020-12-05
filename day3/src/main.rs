use std::fs;
use std::io::{prelude::*, BufReader};

const MAP_WIDTH : u32 = 31;

fn main() -> std::io::Result<()> {
    let map = read_map("day3-input.txt")?;
    let inputs = [ (1,1), (3,1), (5,1), (7,1), (1,2) ];

    let mut product = 1;

    for (x, y) in &inputs {
        let tree_count = count_trees(&map, *x, *y);
        println!("right = {}, down = {}, trees = {}", x, y, tree_count);
        product *= tree_count;
    }

    println!("product = {}", product);

    Ok(())
}

fn count_trees(map: &[u32], dx : u32, dy : u32) -> i32 {
    let mut x : u32 = 0;
    let mut y : usize = 0;
    let mut tree_count = 0;

    while y < map.len() {
        if (map[y] & (1u32 << x)) != 0 {
            tree_count += 1;
        }
        x = (x + dx) % MAP_WIDTH;
        y += dy as usize;
    }
    tree_count
}

fn read_map(path: &str) -> std::io::Result<Vec::<u32>> {
    let mut v = Vec::<u32>::new();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        let mut bits : u32 = 0;
        let mut x = 0;
        for ch in line?.chars() {
            match ch {
                '#' => {
                    bits |= 1u32 << x;
                    x += 1;
                }
                '.' => {
                    x += 1;
                }
                _ => {
                    println!("Warning: unexpected character.");
                }
            }
        }
        if x != MAP_WIDTH {
            println!("Warning: line has width {} ({} expected).", x, MAP_WIDTH);
        }
        v.push(bits);
    }
    Ok(v)
}
