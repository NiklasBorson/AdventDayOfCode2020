use std::fs;
use std::io::{prelude::*, BufReader};

const INVALID_INDEX : u32 = 0xFFFFFFFF;

// Internal representation of a color definition.
struct ColorDef {
    color_name : String,
    first_child : u32
}

// Public wrapper for a color definition.
pub struct BagColor<'a> {
    rule_list : &'a RuleList,
    color_def : &'a ColorDef,
    index : u32
}

impl<'a> BagColor<'a> {
    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn name(&self) -> &'a str {
        &self.color_def.color_name
    }

    pub fn children(&self) -> ChildBagIterator<'a> {
        ChildBagIterator{ 
            rule_list : self.rule_list,
            child_id : self.color_def.first_child
        }
    }

    pub fn contains_color(&self, other_index : u32) -> bool {
        for child in self.children() {
            let color = child.get_color();
            if color.index() == other_index || color.contains_color(other_index) {
                return true;
            }
        }
        false
    }
}

// Internal representation of a node in a linked list of child bags.
struct ChildNode {
    next_child : u32,
    pub child_count : u32,
    child_color : u32
}

// Public wrapper for a child bag.
pub struct ChildBag<'a> {
    rule_list : &'a RuleList,
    node : &'a ChildNode
}

impl<'a> ChildBag<'a> {
    pub fn get_count(&self) -> u32 {
        self.node.child_count
    }

    pub fn get_color(&self) -> BagColor<'a> {
        let index = self.node.child_color;
        BagColor{
            rule_list : &self.rule_list,
            color_def : &self.rule_list.color_defs[index as usize],
            index : index
        }
    }
}

// Iterator for linked list of child bags.
pub struct ChildBagIterator<'a> {
    rule_list : &'a RuleList,
    child_id : u32
}

impl<'a> Iterator for ChildBagIterator<'a> {
    type Item = ChildBag<'a>;
    fn next(&mut self) -> Option<ChildBag<'a>> {
        if self.child_id == INVALID_INDEX {
            None
        }
        else {
            let node = &self.rule_list.child_nodes[self.child_id as usize];
            self.child_id = node.next_child;
            Some(ChildBag{ rule_list : self.rule_list, node : &node })
        }
    }
}

pub struct RuleList {
    color_defs : Vec<ColorDef>,
    child_nodes : Vec<ChildNode>
}

impl RuleList {
    pub fn new(path : &str) -> std::io::Result<RuleList> {
        let mut rules = RuleList{ color_defs : Vec::new(), child_nodes : Vec::new() };
        for line in BufReader::new(fs::File::open(path)?).lines() {
            rules.add_rule(&line?);
        }
        Ok(rules)
    }

    pub fn find_color(&self, color_name : &str) -> Option<BagColor> {
        if let Some(index) = self.find_color_id(color_name) {
            Some(self.get_color(index))
        }
        else {
            None
        }
    }

    pub fn count(&self) -> u32 {
        self.color_defs.len() as u32
    }

    pub fn get_color(&self, index : u32) -> BagColor {
        BagColor{ 
            rule_list : self, 
            color_def : &self.color_defs[index as usize], 
            index : index 
        }
    }

    fn find_color_id(&self, color_name : &str) -> Option<u32> {
        for i in 0..self.color_defs.len() {
            if self.color_defs[i].color_name == color_name {
                return Some(i as u32);
            }
        }
        None
    }

    fn get_color_id(&mut self, color_name : &str) -> u32 {
        if let Some(id) = self.find_color_id(color_name) {
            id
        }
        else {
            let index = self.color_defs.len();
            self.color_defs.push(ColorDef{ color_name : String::from(color_name), first_child : INVALID_INDEX });
            index as u32
        }
    }

    fn add_rule(&mut self, line : &str) {
        // Parse a rule with the following BNF:
        //
        //      <rule> = <color_name> " bags contain " <tail>
        //      <tail> = <content> ( ", " <content> )* "."
        //      <content> = <number> " " <color_name> " bag" ["s"] | "no other bags"
        //
        // First separate <color_name> from <tail>.
        if let Some((color_name, mut tail)) = split2(line, " bags contain ") {
            let color_id = self.get_color_id(color_name);

            // Remove trailing period from the tail.
            if let Some(i) = tail.find('.') {
                tail = &tail[0..i];
            }
            
            // Split the tail into comma-delimited content productions.
            for content in tail.split(", ") {
                
                // Split the first word (number) from the color name.
                if let Some((number, child_color_name)) = split2(content, " ") {

                    // Parse the number; this will fail in the case of "no other bags".
                    if let Ok(child_count) = number.parse::<u32>() {

                        // Insert a new ChildNode at the head of the list for this ColorDef.
                        let child_id = self.child_nodes.len() as u32;
                        let child_color_id = self.get_color_id(trim_from(child_color_name, " bag"));
                        let color_def = &mut self.color_defs[color_id as usize];
                        self.child_nodes.push(ChildNode{ next_child : color_def.first_child, child_count : child_count, child_color : child_color_id });
                        color_def.first_child = child_id;
                    }
                }
            }
        }
    }
}

fn trim_from<'a>(s : &'a str, suffix : &str) -> &'a str {
    match s.find(suffix) {
        Some(i) => &s[0..i],
        None => s
    }
}

fn split2<'a>(s : &'a str, delim : &str) -> Option<(&'a str, &'a str)> {
    match s.find(delim) {
        Some(i) => Some((&s[..i], &s[i + delim.len()..])),
        None => None
    }
}
