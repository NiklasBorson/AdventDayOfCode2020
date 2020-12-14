use std::fs;
use std::io::{prelude::*, BufReader, BufWriter};

fn main() -> std::io::Result<()> {
    let (start_time, periods) = read_file("input.txt")?;

    find_best_period(start_time, &periods);

    write_periods(&periods)?;

    find_special_time(&periods)?;

    Ok(())
}

fn find_best_period(start_time : usize, periods : &[(usize,usize)]) {
    let mut best_period = 0;
    let mut best_wait = usize::MAX;

    for &(_i, p) in periods {
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

fn write_periods(periods : &[(usize,usize)]) -> std::io::Result<()> {
    let mut writer = BufWriter::new(fs::File::create("periods.txt")?);

    for &(i, p) in periods {
        let line = format!("{},{}\n", i, p);
        writer.write(line.as_bytes())?;
    }

    Ok(())
}

struct Bus {
    i : usize,
    p : usize,
    n : usize,
    t : usize    // (p * n) - i
}

impl Bus {
    fn compute_nt(i : usize, p : usize, min_t : usize) -> (usize, usize) {
        // Compute the smallest n such that (p * n) - i >= min_t.
        //   p * n >= min_t + i
        //   n >= (min_t + i) / p
        let mut n = (min_t + i) / p;
        
        while p * n < min_t + i {
            n += 1;
        }

        let t = (p * n) - i;

        assert!(t >= min_t && t < min_t + p);

        (n, t)
    }

    fn new(i : usize, p : usize) -> Bus {
        let (n, t) = Bus::compute_nt(i, p, 0);
        Bus { i : i, p : p, n : n, t : t }
    }

    fn next(&mut self, min_t : usize) -> usize {
        let (n, t) = Bus::compute_nt(self.i, self.p, min_t);
        self.n = n;
        self.t = t;
        t
    }
}

fn least_common_multiple(v : &[(usize,usize)]) -> usize {
    let mut factors = factor(v[0].1);

    for &(_i, p) in &v[1..] {
        factors = get_common_factors(&factors, &factor(p));
    }

    get_product(&factors)
}

fn get_product(v : &[usize]) -> usize {
    v.iter().fold(1, |accum,x| accum * x)
}

fn get_common_factors(a : &[usize], b : &[usize]) -> Vec::<usize> {
    let mut v = Vec::new();

    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        let an = a[i];
        let bn = b[j];
        if an <= bn {
            v.push(an);
            i += 1;
            if an == bn {
                j += 1;
            }
        }
        else {
            v.push(bn);
            j += 1;
        }
    }

    if i < a.len() {
        v.extend_from_slice(&a[i..]);
    }

    if j < b.len() {
        v.extend_from_slice(&b[j..]);
    }

    v
}

fn factor(mut n : usize) -> Vec::<usize> {
    let mut v = Vec::new();

    while (n & 1) == 0 {
        v.push(2);
        n >>= 1;
    }

    for f in (3..).step_by(2) {
        if f * f > n {
            break;
        }

        while (n % f) == 0 {
            v.push(f);
            n /= f;
        }
    }

    if n > 1 {
        v.push(n);
    }

    v
}

fn find_special_time(periods : &[(usize,usize)]) -> std::io::Result<()> {
    let lcm = least_common_multiple(&periods);
    println!("least common multiple = {}", lcm);

    let mut v = Vec::new();
    for &(i, p) in periods {
        v.push(Bus::new(i, p));
    }

    let mut writer = BufWriter::new(fs::File::create("times.csv")?);

    loop {
        // Compute the minimum and maximum t for all the buses.
        let mut min_t = v.iter()
            .map(|bus| bus.t)
            .fold(usize::MAX, |accum,x| std::cmp::min(accum, x));

        let mut max_t = v.iter()
            .map(|bus| bus.t)
            .fold(0, |accum,x| std::cmp::max(accum, x));
        
        for bus in &v {
            if bus.t < min_t { min_t = bus.t; }
            if bus.t > max_t { max_t = bus.t; }

            let field = format!("{} * {} - {} = {},", bus.p, bus.n, bus.i, bus.t);
            writer.write(field.as_bytes())?;
        }
        let newline = [10];
        writer.write(&newline)?;

        // If all the times are the same then we're done.
        if min_t == max_t {
            println!("The magic time is {}", min_t);
            break;            
        }

        for bus in &mut v {
            max_t = bus.next(max_t);
        }

        if max_t >= lcm {
            break;
        }
    }

    Ok(())
}

fn compute_wait(start_time : usize, p : usize) -> usize {
    p - (start_time % p)
}


fn read_file(path: &str) -> std::io::Result<(usize,Vec::<(usize,usize)>)> {
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
            if let Ok(n) = field.parse::<usize>() {
                periods.push((i,n));
            }
            i += 1;
        }
    }

    Ok((start_time, periods))
}
