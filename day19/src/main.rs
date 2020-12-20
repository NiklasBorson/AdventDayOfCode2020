mod grammar;
mod nfa;

fn main() -> std::io::Result<()> {

    // Part 1
    test_input("input.txt")?;

    // Part 2 - contains cycles
    //test_input("input2.txt")?;

    Ok(())
}

fn test_input(path : &str) -> std::io::Result<()> {
    let (rules, input) = grammar::read_file(path)?;

    let nfa = nfa::Nfa::new(&rules);
    nfa.write_transitions("transitions.txt")?;

    let mut match_count = 0;
    for line in &input[..] {
        let is_match = nfa.is_match(line);
        //println!("{} -> {}", line, is_match);
        if is_match { match_count += 1; }
    }
    println!("Matched {} of {}", match_count, input.len());
 
    Ok(())
}