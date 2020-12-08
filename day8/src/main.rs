mod cpu;

fn main() -> std::io::Result<()> {
    let mut computer = cpu::Computer::load_program("day8-input.txt")?;
    check_for_loop(&mut computer);
    Ok(())
}

fn check_for_loop(computer : &mut cpu::Computer) {
    let mut visited = Vec::new();
    visited.resize(computer.get_instruction_count(), false);

    while computer.in_bounds() && !visited[computer.get_instruction_index()] {
        visited[computer.get_instruction_index()] = true;
        computer.step();
    }

    if computer.in_bounds() {
        println!(
            "Infinite loop at {} with accumulator = {}",
            computer.get_instruction_index(),
            computer.get_accumulator()
        );
    }
}
