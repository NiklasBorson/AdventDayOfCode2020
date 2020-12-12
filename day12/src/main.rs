use std::fs;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {

    // Read the file into a vector of i32.
    let directions = read_directions("input.txt")?;

    let mut ship = Ship::new();
    let mut ship2 = Ship2::new();

    for dir in directions {
        ship.navigate(dir);
        ship2.navigate(dir);
    }

    println!("Ship is at {} E, {} S, Manhattan distance = {}.", 
        ship.position.x, ship.position.y, distance(ship.position)
    );

    println!("Ship2 is at {} E, {} S, Manhattan distance = {}.", 
        ship2.position.x, ship2.position.y, distance(ship2.position)
    );

    Ok(())
}

#[derive(Copy, Clone)]
enum Direction {
    Translate { dx : i32, dy : i32 },
    Rotate { degrees : i32 },
    Forward { distance : i32 }
}

#[derive(Copy, Clone)]
struct Point {
    x : i32,
    y : i32
}

struct Ship {
    position : Point,
    heading : i32   // 0 = East, 90 = South, etc.
}

impl Ship {
    fn new() -> Ship {
        Ship{ position : Point{ x: 0, y: 0 }, heading : 0 }
    }

    fn navigate(&mut self, dir : Direction) {
        match dir {
            Direction::Translate{ dx, dy } => {
                self.position.x += dx;
                self.position.y += dy;
            },
            Direction::Rotate{ degrees} => {
                self.heading = normalize_angle(self.heading + degrees);
            },
            Direction::Forward{ distance} => {
                match self.heading {
                    0 => {
                        self.position.x += distance;
                    },
                    90 => {
                        self.position.y += distance;
                    },
                    180 => {
                        self.position.x -= distance;
                    },
                    270 => {
                        self.position.y -= distance;
                    },
                    _ => {
                        println!("Error: invalid heading: {}.", self.heading);
                    }
                }
            }
        };
    }
}

struct Ship2 {
    position : Point,
    waypoint : Point
}

impl Ship2 {
    fn new() -> Ship2 {
        Ship2{ 
            position : Point{ x : 0, y : 0 },
            waypoint : Point{ x : 10, y : -1 }
        }
    }

    fn navigate(&mut self, dir : Direction) {
        match dir {
            Direction::Translate{ dx, dy } => {
                self.waypoint.x += dx;
                self.waypoint.y += dy;
            },
            Direction::Rotate{ degrees} => {
                //
                // Rotate the waypoint around the origin.
                // Compute a rotation matrix from the angle.
                //
                let (sin, cos) = match degrees {
                    0 => (0, 1),
                    90 => (1, 0),
                    180 => (0, -1),
                    270 => (-1, 0),
                    _ => {
                        println!("Error: invalid angle: {}.", degrees);
                        (0, 1)
                    }
                };
                let m11 = cos;
                let m12 = sin;
                let m21 = -sin;
                let m22 = cos;

                //
                // Multiply (x, y) by the rotation matrix.
                //
                let Point{ x, y} = self.waypoint;
                self.waypoint.x = m11 * x + m21 * y;
                self.waypoint.y = m12 * x + m22 * y;
            },
            Direction::Forward{ distance} => {
                self.position.x += distance * self.waypoint.x;
                self.position.y += distance * self.waypoint.y;
            }
        };
    }
}

fn distance(pt : Point) -> i32 {
    pt.x.abs() + pt.y.abs()
}

fn normalize_angle(angle : i32) -> i32 {
    let a = angle % 360;
    if a >= 0 { a } else { a + 360 }
}

fn parse_int(iter : &mut std::str::Chars) -> Option<i32> {
    let mut n = iter.next()?.to_digit(10)?;
    while let Some(ch) = iter.next() {
        n = (n * 10) + ch.to_digit(10)?;
    }
    Some(n as i32)    
}

fn parse_direction(line: &str) -> Option<Direction> {
    let mut iter = line.chars();
    let code = iter.next()?;
    let arg = parse_int(&mut iter)?;

    match code {
        'N' => Some(Direction::Translate{ dx : 0, dy : -arg }),
        'S' => Some(Direction::Translate{ dx : 0, dy : arg }),
        'E' => Some(Direction::Translate{ dx : arg, dy : 0 }),
        'W' => Some(Direction::Translate{ dx : -arg, dy : 0 }),
        'L' => Some(Direction::Rotate{ degrees : normalize_angle(-arg) }),
        'R' => Some(Direction::Rotate{ degrees : normalize_angle(arg) }),
        'F' => Some(Direction::Forward{ distance : arg }),
        _ => None
    }
}

fn read_directions(path: &str) -> std::io::Result<Vec::<Direction>> {
    let mut v = Vec::new();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        if let Some(dir) = parse_direction(&line?) {
            v.push(dir);
        }
    }
    Ok(v)
}
