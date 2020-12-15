use std::collections::HashMap;

fn main() {
    let input = [ 0, 14, 1, 3, 7, 9 ];
    let mut game = GameState::new(&input);

    // Part 1
    game.advance_to(2020);
    println!("{}", game.last_number);

    // Part 2
    game.advance_to(30000000);
    println!("{}", game.last_number);
}

struct GameState {
    next_index : usize,
    last_number : usize,
    turn_map : HashMap<usize, usize>
}

impl GameState {
    fn new(input : &[usize]) -> GameState {
        let mut turn_map = HashMap::new();

        let next_index = input.len();
        let last_index = next_index - 1;
        let last_number = input[last_index as usize];

        for i in 0..last_index {
            let n = input[i];
            //println!("{}. {}", i + 1, n);
            turn_map.insert(n, i + 1);
        }
        //println!("{}. {}", next_index, last_number);

        GameState {
            next_index,
            last_number,
            turn_map
        }
    }

    fn next(&mut self) {
        let n = match self.turn_map.get(&self.last_number) {
            Some(index) => self.next_index - *index,
            None => 0
        };
        self.turn_map.insert(self.last_number, self.next_index);

        self.last_number = n;
        self.next_index += 1;

        //println!("{}. {}", self.next_index, self.last_number);
    }

    fn advance_to(&mut self, turn_index : usize) {
        while self.next_index < turn_index {
            self.next();
        }
    }
}
