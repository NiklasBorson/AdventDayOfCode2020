use std::fs;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {

    // Read the file into a vector of i32.
    let mut numbers = read_numbers("day10-input.txt")?;
    numbers.sort_unstable();

    // Solve part 1.
    count_intervals(&numbers);

    // Solve part2.
    println!("{} arrangements", count_arrangements(&numbers));

    Ok(())
}

fn count_intervals(numbers : &[i32]) {
    let mut diffs1 = 0;
    let mut diffs3 = 0;
    let mut last_jolt = 0;
    for jolt in numbers {
        match *jolt - last_jolt {
            1 => { diffs1 += 1; }
            3 => { diffs3 += 1; }
            _ => {}
        }
        last_jolt = *jolt;
    }

    // Add the 3-jolt step to the device's built-in adapter
    diffs3 += 1;

    println!("{} * {} = {}", diffs1, diffs3, diffs1 * diffs3);
}

fn count_arrangements(numbers : &[i32]) -> usize {

    // Starting with a sequence of sorted unique numbers, we can selectivly
    // elide numbers to produce other sequences. This gives up to pow(2,N)
    // combinations, except that only some combinations are valid.
    //
    // In a valid combination, no two consecutive numbers differ by more
    // than three. There's an implicit 0 before the first number. There's
    // an implicit (last + 3) after the last number.
    //
    // From the above it can be deduced that the last number can never
    // be elided and no more than two consecutive numbers can be elided.
    //
    // We can compute the number or valid combinations (arrangements) in
    // one pass by keeping track of three possible states:
    //
    //      State 0: we kept the last value.
    //      State 1: we elided last one value.
    //      State 2: we elided the last two values.
    //
    // For each state, we keep track of the number of valid combinations
    // so far for that state, and the most recent number for that state.
    // For example, consider the input:
    //
    //      1 2 5 6 7 8 9 12 15 18
    //
    // The following table shows the recent and counts values for each
    // state at the end of each iteration. The first row shows the
    // initial values after the implied input value 0.
    //
    //      n       r0  c0      r1  c1      r2  c2
    //      0        0   1       0   0       0   0
    //      1        1   1       0   1       0   0
    //      2        2   2       1   1       0   1
    //      5        5   2       2   2       1   1
    //      6        6   2       5   2       2   2
    //      7        7   4       6   2       5   2
    //      8        8   8       7   4       6   2
    //      9        9  14       8   8       7   4
    //     12       12  14       9  14       8   8
    //     15       15  14      12  14       9  14
    //     18       18  14      15  14      12  14

    // Most recent value for each state.
    let mut recent = [0, 0, 0];

    // Number of combinations so far for each state. Initially, only
    // state 0 has a valid combination because nothing can have been
    // elided up to this point.
    let mut counts = [1, 0, 0];     

    for i in 0..numbers.len() {
        let n = numbers[i];
        let [r0, r1, r2] = recent;
        let [c0, c1, c2] = counts;

        // Assume we're entering state 0 (keeping i).
        recent[0] = n;
        counts[0] = 0;
        if n - r0 <= 3 {
            counts[0] = c0;
            if n - r1 <= 3 {
                counts[0] += c1;
                if n - r2 <= 3 {
                    counts[0] += c2;
                }
            }
        }

        // Assume we're entering state 1 (eliding i but keeping i-1).
        recent[1] = r0;
        counts[1] = c0;

        // Assume we're entering state 2 (eliding i and i-1).
        recent[2] = r1;
        counts[2] = c1;
    }

    counts[0]
}

fn read_numbers(path: &str) -> std::io::Result<Vec::<i32>> {
    let mut v = Vec::<i32>::new();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        let s = line?;
        if let Ok(n) = s.parse::<i32>() {
            v.push(n);
        }
    }
    Ok(v)
}
