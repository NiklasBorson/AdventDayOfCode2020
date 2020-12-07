use std::fs;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let rules = BagRules::new("day7-input.txt")?;

    println!("Num containing colors for shiny gold: {}", rules.count_containing_colors("shiny gold"));

    if let Some(count) = rules.get_min_children("shiny gold") {
        println!("Fewest bags within shiny gold: {}", count);
    }

    Ok(())
}

struct BagColor {
    name : String,
    first_child : i32 // -1 if none
}

struct ChildBag {
    count : u32,
    color_index : u32,
    next_sibling : i32 // -1 if none
}

struct BagRules {
    colors : Vec<BagColor>,
    list_nodes : Vec<ChildBag>
}

fn split2<'a>(s : &'a str, delim : &str) -> Option<(&'a str, &'a str)> {
    match s.find(delim) {
        Some(i) => Some((&s[..i], &s[i + delim.len()..])),
        None => None
    }
}

impl BagRules {
    fn new(path : &str) -> std::io::Result<BagRules> {
        let mut rules = BagRules{ colors : Vec::new(), list_nodes : Vec::new() };
        for line in BufReader::new(fs::File::open(path)?).lines() {
            rules.add_rule(&line?);
        }
        Ok(rules)
    }

    fn add_rule(&mut self, line : &str) {
        if let Some((color_name, mut tail)) = split2(line, " bags contain ") {

            let bag_color_index = self.get_color_index(color_name);

            while !tail.is_empty() {
                let content =
                    if let Some((left, right)) = split2(tail, ", ") {
                        tail = right;
                        left
                    }
                    else if let Some(end) = tail.find('.') {
                        let left = &tail[..end];
                        tail = "";
                        left
                    }
                    else {
                        break;
                    };

                if let Some((number, color)) = split2(content, " ") {
                    if let Ok(n) = number.parse::<u32>() {
                       if let Some(end) = color.find(" bag") {
                            let child_color_index = self.get_color_index(&color[0..end]);
                            self.add_child_bag(bag_color_index, n, child_color_index);
                       } 
                    }
                }
            }
        }
    }

    fn find_color_index(&self, name : &str) -> Option<u32> {
        // Look for existing color using linear search.
        for i in 0..self.colors.len() {
            if self.colors[i].name == name {
                return Some(i as u32);
            }
        }
        None
    }

    fn get_color_index(&mut self, name : &str) -> u32 {
        if let Some(i) = self.find_color_index(name) {
            i
        }
        else {
            let i = self.colors.len() as u32;
            self.colors.push(BagColor{ name : String::from(name), first_child : -1 });
            i
        }
    }

    fn add_child_bag(&mut self, parent_color_index : u32, child_count : u32, child_color_index : u32) {
        let mut color = & mut(self.colors[parent_color_index as usize]);
        let new_child_index = self.list_nodes.len() as i32;
        self.list_nodes.push(ChildBag{ count : child_count, color_index : child_color_index, next_sibling : color.first_child });
        color.first_child = new_child_index;
    }

    fn count_containing_colors(&self, color_name : &str) -> u32 {
        let mut count = 0;
        if let Some(target_color) = self.find_color_index(color_name) {
            let mut visited = Vec::new();

            for bag_color_index in 0..self.colors.len() {
                visited.resize(self.colors.len(), false);
                if self.contains_color(&mut visited, bag_color_index as u32, target_color) {
                    count += 1;
                }
                visited.clear();
            }
        }
        count
    }

    fn contains_color(&self, visited : &mut[bool], bag_color_index : u32, target_color : u32) -> bool {
        let bag_color = &self.colors[bag_color_index as usize];
        let mut node_index = bag_color.first_child;
        while node_index >= 0 {
            let child = &self.list_nodes[node_index as usize];
            let child_color = child.color_index;
            if !visited[child_color as usize] {
                if child_color == target_color || self.contains_color(visited, child_color, target_color) {
                    return true;
                }
            }
            node_index = child.next_sibling;
        }
        false
    }

    fn get_min_children(&self, color_name : &str) -> Option<u32> {
        if let Some(color_index) = self.find_color_index(color_name) {
            let mut visited = Vec::new();
            return self.get_min_children2(&mut visited, color_index);
        }
        
        None
    }

    fn get_min_children2(&self, visited : &mut Vec<u32>, color_index : u32) -> Option<u32> {
        let bag = &self.colors[color_index as usize];
        if bag.first_child < 0
        {
            return Some(0);
        }

        visited.push(color_index);

        let mut best = None;

        // Iterate over the possible child bags.
        let mut node_index = bag.first_child;
        while node_index >= 0 {
            let child = &self.list_nodes[node_index as usize];

            // Make sure it's not a cycle.
            if !visited.contains(&child.color_index) {

                if let Some(count) = self.get_min_children2(visited, child.color_index) {
                    // The full count is the child bag itself plus the contents of the child bag,
                    // times the number of child bags.
                    let full_count = (1 + count) * child.count;

                    if let Some(best_count) = best {
                        if full_count < best_count {
                            best = Some(full_count);
                        }
                    }
                    else {
                        best = Some(full_count);
                    }
                }
            }

            node_index = child.next_sibling;
        }

        visited.pop();

        best
    }
}

