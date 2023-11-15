use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::{Hash, Hasher};
const DIRS: [[i8; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
#[derive(Eq, Clone)]
struct State {
    t_cost: i32,
    board: Vec<Vec<i8>>,
    cost: i32,
    man_cost: i32,
    parent: Option<Box<State>>,
}

impl State {
    // constructor
    fn new(board: Vec<Vec<i8>>) -> State {
        State {
            board: board.clone(),
            cost: 0,
            man_cost: manhatten_cost(&board),
            t_cost: manhatten_cost(&board),
            parent: None,
        }
    }
}

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

fn is_finished(s: &State) -> bool {
    let goal = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]];
    s.board == goal
}

fn manhatten_cost(board: &Vec<Vec<i8>>) -> i32 {
    let goal: Vec<Vec<i8>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]];
    let mut cost = 0;
    for i in 0..3 {
        for j in 0..3 {
            'inner: for k in 0..3 {
                for l in 0..3 {
                    if board[i][j] == goal[k][l] {
                        cost += (i as i32 - k as i32).abs() + (j as i32 - l as i32).abs();
                        break 'inner;
                    }
                }
            }
        }
    }
    cost
}

fn print(s: &State) {
    for i in 0..3 {
        for j in 0..3 {
            print!("{} ", s.board[i][j]);
        }
        println!();
    }
    println!();
}

fn path(s: &State) {
    if let Some(ref p) = s.parent {
        path(p);
    }
    print(s);
}

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
                    new_state.cost = curr.cost + 1 + new_state.man_cost;

                    new_state.parent = Some(Box::new(curr.clone()));
                    if !seen.contains(&new_state) {
                        heap.push(new_state);
                    }
                }
            }
        }
    }
}

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
    let start: State = State::new(starting_board);
    let mut seen: HashSet<State> = HashSet::new();

    heap.push(start);
    let start_time = std::time::Instant::now();
    while !heap.is_empty() {
        let mut curr = heap.pop().unwrap();
        if is_finished(&curr) {
            path(&curr);
            let end_time = std::time::Instant::now();
            println!("Time taken: {}ms", (end_time - start_time).as_millis());
            break;
        } else {
            expand(&mut heap, &mut seen, &mut curr);
        }
    }
    println!("Total states explored: {}", seen.len());
}
