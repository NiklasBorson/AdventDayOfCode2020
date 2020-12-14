use std::fs;
use std::io::{prelude::*, BufReader};

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Bus(/*p*/usize, /*i*/usize);

impl Bus {
    fn period(&self) -> usize { self.0 }
    fn index(&self) -> usize { self.1 }
}

fn main() -> std::io::Result<()> {
    let (start_time, periods) = read_file("input.txt")?;

    // Part 1
    find_best_period(start_time, &periods);

    // Part 2
    find_special_time(&periods);

    Ok(())
}

fn find_best_period(start_time : usize, periods : &[Bus]) {
    let mut best_period = 0;
    let mut best_wait = usize::MAX;

    for &Bus(p, _i) in periods {
        let wait = compute_wait(start_time, p);
        if wait < best_wait {
            best_period = p;
            best_wait = wait;
        }
    }

    println!("best_period = {}", best_period);
    println!("best_wait = {}", best_wait);
    println!("product = {}", best_period * best_wait);
}

fn is_prime(n: usize) -> bool {
    if (n & 1) == 0
    {
        return false;
    }
    for i in (3..).step_by(2) {
        if n % i == 0 {
            return false;
        }
        if i * i > n {
            break;
        }
    }
    true
}

struct Bus2 {
    p : usize,
    t : usize
}

impl Bus2 {
    // Find the first matching time for this bus.
    fn first_time(bus : & Bus) -> usize {
        assert!(is_prime(bus.period()));

        let p = bus.period() as isize;
        let i = bus.index() as isize;

        // Time t matches if the bus departs at (t + i) seconds. Bus departure 
        // times are integer multiples of p, so matching times are:
        // 
        //      t = (p * N) - i
        //      where N = any integer
        //
        // Compute t for N = 0.
        let mut t = p - i;

        // Find the first non-negative t.
        while t < 0 { t += p; }

        t as usize
    }

    fn new(bus : &Bus) -> Bus2 {
        Bus2{ p : bus.period(), t : Bus2::first_time(bus) }
    }

    fn match_time(&mut self, t : usize) -> bool {
        assert!(self.t < t);
        self.t += round_up_to_multiple(t - self.t, self.p);
        self.t == t
    }
}

fn round_up_to_multiple(n : usize, k : usize) -> usize {
    let m = n % k;
    if m > 0 { n - m + k } else { n - m }
}

fn find_special_time(periods : &[Bus]) {
    if periods.is_empty() {
        return;
    }

    // Get the last bus, which has the longest period.
    let last_index = periods.len() - 1;
    let last = &periods[last_index];

    // Create Bus2 objects for the other, faster buses.
    let mut others = Vec::new();
    for bus in &periods[..last_index] {
        others.push(Bus2::new(bus));
    }

    // The match time must be less than the product of all the
    // periods because at that point we begin a new cycle where
    // all the buses depart together, like at t = 0.
    let &limit = &periods.iter()
        .map(|bus| bus.period())
        .fold(1, |accum,x| accum * x);

    // Start at the first possible match for the slowest bus.
    let mut t= Bus2::first_time(last);
    while t < limit {
        let mut step = last.period();
        let mut all_match = true;

        // Iterate over the faster buses.
        for bus in &mut others {
            if !bus.match_time(t) {

                // Time t is not a match for bus.
                all_match = false;
            }
            else {

                // If two or more buses match at time t then the smallest possible
                // interval to the next match is the product of their periods.
                step *= bus.p;
            }
        }

        if all_match {
            println!("Matching time is {}.", t);
            break;
        }

        t += step;
    }
}

fn compute_wait(start_time : usize, p : usize) -> usize {
    p - (start_time % p)
}

fn read_file(path: &str) -> std::io::Result<(usize,Vec::<Bus>)> {
    let mut start_time = 0;
    let mut periods = Vec::new();
    let mut lines = BufReader::new(fs::File::open(path)?).lines();

    if let Some(line) = lines.next() {
        let s = line?;
        if let Ok(n) = s.parse::<usize>() {
            start_time = n;
        }
    }

    if let Some(line) = lines.next() {
        let s = line?;
        let mut i = 0;
        for field in s.split(',') {
            if let Ok(p) = field.parse::<usize>() {
                periods.push(Bus(p, i));
            }
            i += 1;
        }
    }

    periods.sort_unstable();

    Ok((start_time, periods))
}
