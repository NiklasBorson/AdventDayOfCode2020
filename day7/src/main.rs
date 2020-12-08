mod bag_rules;

fn main() -> std::io::Result<()> {

    // Load the rule definitions.
    let rules = bag_rules::RuleList::new("day7-input.txt")?;
    println!("{} colors are defined.", rules.count());

    // Try getting the color definition for shiny gold.
    if let Some(color) = rules.find_color("shiny gold") {

        // Determine how many other colors contain this color.
        let mut contains_count = 0;
        for index in 0..rules.count() {
            if rules.get_color(index).contains_color(color.index()) {
                contains_count += 1;
            }
        }
        println!("{} colors contain {}.", contains_count, color.name());

        // Recursively count the child bags of this color and its contents.
        println!("A {} bag contains at least {} other bags.", color.name(), count_children(&color));

        // List the immediate children of this color.
        println!("Contents of {} ({}):", color.name(), color.index());
        for child in color.children() {
            let child_color = child.get_color();
            println!("    {} * {} ({})", child.get_count(), child_color.name(), child_color.index());
        }
    }

    Ok(())
}

fn count_children(color : &bag_rules::BagColor) -> u32 {
    let mut count = 0;
    for child in color.children() {

        // For each child bag, count the bag itself plus all the bags it contains.
        let per_child_count = 1 + count_children(&child.get_color());

        // Muliply this by the number of child bags.
        count += per_child_count * child.get_count();
    }
    count
}
