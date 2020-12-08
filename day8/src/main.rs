mod cpu;

fn main() -> std::io::Result<()> {
    let mut computer = cpu::Computer::load_program("day8-input.txt")?;

    // Part 1
    computer.run();
    if computer.in_bounds() {
        println!(
            "Infinite loop at {} with accumulator = {}",
            computer.get_instruction_index(),
            computer.get_accumulator()
        );
    }

    for index in 0..computer.get_instruction_count() {
        let old_op = computer.get_op_code(index);
        let new_op = match old_op {
            cpu::OpCode::Acc => cpu::OpCode::Acc,   // unchanged
            cpu::OpCode::Jmp => cpu::OpCode::Nop,
            cpu::OpCode::Nop => cpu::OpCode::Jmp
        };

        if new_op != old_op {
            computer.set_op_code(index, new_op);

            computer.reset();
            computer.run();

            if computer.get_instruction_index() == computer.get_instruction_count() {
                println!("Fixed program by changing instruction {}.", index);
                println!("Final accumulator value = {}.", computer.get_accumulator());
            }

            computer.set_op_code(index, old_op);
        }
    }

    Ok(())
}

