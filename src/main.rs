use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::{Hash, Hasher};

/* Constants */
const DIRS: [[i8; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
const GOAL: [[i8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 0]];
const GOAL_POS: [(i8, i8); 8] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (1, 0),
    (1, 1),
    (1, 2),
    (2, 0),
    (2, 1),
];
const LIMIT: usize = 100000;

/* State struct for 8-puzzle */
#[derive(Eq, Clone)]
struct State {
    t_cost: i32,
    board: Vec<Vec<i8>>,
    cost: i32,
    man_cost: i8,
    parent: Option<Box<State>>,
}

impl State {
    // constructor
    fn new(board: Vec<Vec<i8>>) -> State {
        State {
            board: board.clone(),
            cost: 0,
            man_cost: manhatten_cost(&board),
            t_cost: 0,
            parent: None,
        }
    }
}

/* Trait implementations for struct State */
impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.t_cost == other.t_cost
    }
}

// for min-heap behabior
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.t_cost.cmp(&self.t_cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for row in &self.board {
            for &val in row {
                val.hash(state);
            }
        }
    }
}
/* Check is state reached goal */
fn is_finished(s: &State) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if s.board[i][j] != GOAL[i][j] {
                return false;
            }
        }
    }
    true
}
/* Calculate the manhatten cost for a board. */
fn manhatten_cost(board: &Vec<Vec<i8>>) -> i8 {
    let mut cost = 0;
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] != 0 {
                let (x, y) = GOAL_POS[(board[i][j] - 1) as usize];
                cost += (i as i8 - x).abs() + (j as i8 - y).abs();
            }
        }
    }
    cost
}

/* Prints the board */
fn print(s: &State) {
    for i in 0..3 {
        for j in 0..3 {
            print!("{} ", s.board[i][j]);
        }
        println!();
    }
    println!();
}

/* Traces down the path and prints each state. */
fn path(s: &State) -> i32 {
    let mut count = 0;
    let mut curr = s;
    while let Some(ref p) = curr.parent {
        print(curr);
        curr = p;
        count += 1;
    }
    count
}

/* Expanding the BinaryHeap and HashSet in a DFS. */
fn expand(heap: &mut BinaryHeap<State>, seen: &mut HashSet<State>, curr: &mut State) {
    seen.insert(curr.clone());
    for i in 0..3 {
        for j in 0..3 {
            if curr.board[i][j] == 0 {
                for [x, y] in DIRS {
                    let xx = x + i as i8;
                    let yy = y + j as i8;
                    // swap 0 with the number in the direction
                    if xx < 0 || xx > 2 || yy < 0 || yy > 2 {
                        continue;
                    }
                    let mut a = curr.board[i][j];
                    let mut b = curr.board[xx as usize][yy as usize];
                    std::mem::swap(&mut a, &mut b);
                    let mut new_state = State::new(curr.board.clone());
                    new_state.board[i][j] = a;
                    new_state.board[xx as usize][yy as usize] = b;
                    new_state.man_cost = manhatten_cost(&new_state.board);
                    new_state.cost = 1 + curr.cost;
                    new_state.t_cost = new_state.cost + new_state.man_cost as i32;
                    new_state.parent = Some(Box::new(curr.clone()));
                    if !seen.contains(&new_state) {
                        heap.push(new_state);
                    }
                }
            }
        }
    }
}

/* Run the A* algorithm to attempt to find a solution within LIMIT states */
fn main() {
    let mut starting_board = vec![vec![0; 3]; 3];
    let mut numbers: HashSet<i8> = HashSet::new();
    println!("Enter the starting board in order (0 for blank): ");
    for i in 0..3 {
        for j in 0..3 {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().parse::<i8>().unwrap();
            if input < 0 || input > 8 {
                panic!("Invalid input");
            }
            if numbers.contains(&input) {
                panic!("Duplicate number");
            }
            numbers.insert(input);
            starting_board[i][j] = input;
        }
    }

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut start: State = State::new(starting_board);
    start.t_cost = start.cost + start.man_cost as i32;

    let mut seen: HashSet<State> = HashSet::new();
    println!("-----Starting Search-----");

    heap.push(start);
    let start_time = std::time::Instant::now();
    while !heap.is_empty() {
        let mut curr = heap.pop().unwrap();
        if is_finished(&curr) {
            let count = path(&curr);
            let end_time = std::time::Instant::now();
            println!("Time taken: {}ms", (end_time - start_time).as_millis());
            println!("Total moves: {}", count);
            break;
        } else if seen.len() > LIMIT {
            println!("No solution found in {} states", LIMIT);
            break;
        } else {
            expand(&mut heap, &mut seen, &mut curr);
        }
    }
    println!("Total states explored: {}", if seen.len() == 0 { 0 } else {seen.len() - 1});
}
