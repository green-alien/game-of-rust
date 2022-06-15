use std::io;
use std::fmt;
use std::{thread, time};
extern crate termsize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Cell{x: i32, y: i32}

//const ORIGIN: Cell = Cell{x: 0, y: 0};

impl Cell {

    // return a cells neighbors
    pub fn get_neighbors(&self) -> [Cell; 8] {

        let xmod = |i| {self.x.wrapping_add(i)};
        let ymod = |i| {self.y.wrapping_add(i)};

        [
            Cell{x: xmod(-1), y: ymod(-1)}, Cell{x: self.x, y: ymod(-1)}, Cell{x: xmod(1), y: ymod(-1)},
            Cell{x: xmod(-1), y: self.y  }, /*         origin         */  Cell{x: xmod(1), y: self.y  },
            Cell{x: xmod(-1), y: ymod( 1)}, Cell{x: self.x, y: ymod( 1)}, Cell{x: xmod(1), y: ymod( 1)},
        ]
    }

    pub fn xy(&self) -> [i32; 2] {
        [self.x , self.y]
    }
}


struct Life(Vec<Cell>);

impl Life {

    // return each cell neighboring a live cell
    fn gen_relevent_cells(&self) -> Vec<Cell> {

        // map get neighbors to each cell that is alive
        let all = self.0.iter().map(|x| { x.get_neighbors() });

        // fold lists of neighbors into vec 
        let mut rel = all.fold(vec!(), |a, b| { [a, b.to_vec()].concat()});

        // remove duplicate cells
        rel.sort();
        rel.dedup();

        rel
    }

    // return the next state of life
    fn eval(&self) -> Life {
        
        // borrow current state
        let alive = &self.0;

        // get all cells who nieghbor a live cell
        let mut rel_neighbors = self.gen_relevent_cells();

        // retain them if not already in alive
        for a in alive.iter() {
            rel_neighbors.retain(|v| { *v != *a })
        }
        
        // count the number of neigbors in the set alive
        let count_alive_neigbors = |c: Cell| {

            let neighbors = c.get_neighbors();

            neighbors.iter().fold(
                0,
                |c, n| {
                    if alive.contains(n) {c + 1}
                    else {c}
                }
            )
        };

        // cells to be alive in next state iteration
        let mut next_generation: Vec<Cell> = vec!();

        // alive cell rules
        for canidate in alive.iter() {
            let count = count_alive_neigbors(*canidate);
            if count == 2 || count == 3 { next_generation.push(*canidate); }
        }
    
        // dead cell rule
        for canidate in rel_neighbors.iter() {
            let count = count_alive_neigbors(*canidate);
            if count == 3 { next_generation.push(*canidate) }
        }
        
        Life(next_generation)
    }
}

impl fmt::Display for Life { // or bike shedding, code edition

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        // find the terminal dimentions
        let [mut rows, mut cols] = [0; 2];
        termsize::get().map(|size| { 
            rows = size.rows;
            cols = size.cols / 2;
        });

        let rows = rows as usize;
        let cols = cols as usize;

        // center of the board
        let origin_x = (cols/2) as i32;
        let origin_y = (rows/2) as i32;
        
        let mut board = vec![vec!["  "; cols]; rows];
        
        // push live cells here
        for live_cell in self.0.iter() {

            let [xcor, ycor] = live_cell.xy();
            let [xcor, ycor] = [xcor + origin_x, ycor + origin_y];

            let is_on_board = |cor, edge| {0 <= cor && cor < edge as i32};

            if !(is_on_board(xcor, cols) && is_on_board(ycor, rows)) {continue};
            
            board[ycor as usize][xcor as usize] = "\x1b[30;107m  \x1b[0m";
        }

        // finally, collect board into a string, and write
        let mut str_board = "".to_owned();
        for row in board {

            let mut line = "".to_owned();
            for square in row {
                line.push_str(square);
            }

            str_board.push_str(&format!("{line}\n"));
        }

        write!(f, "{str_board}")
    }
}

fn main() {
    
    // "why dont i have a social life?" asks the terrible programer
    let mut gosper = Life(
        vec!(
            Cell{x:0, y:0},   Cell{x:-1, y:0},   Cell{x:-1, y:-1},
            Cell{x:-1, y:1},  Cell{x:-2, y:-2},  Cell{x:-2, y: 2},
            Cell{x:-3, y:0},  Cell{x:-4, y:-3},  Cell{x:-4, y:3},
            Cell{x:-5, y:-3}, Cell{x:-5, y:3},   Cell{x:-6, y:-2},
            Cell{x:-6, y:2},  Cell{x:-7, y:-1},  Cell{x:-7, y:0},
            Cell{x:-7, y:1},  Cell{x:3, y:-1},   Cell{x:3, y:-2}, 
            Cell{x:3, y:-3},  Cell{x:4, y:-1},   Cell{x:4, y:-2}, 
            Cell{x:4, y:-3},  Cell{x:5, y:0},    Cell{x:5, y: -4},
            Cell{x:7, y:0},   Cell{x:7, y:1},    Cell{x:7, y:-4}, 
            Cell{x:7, y:-5},  Cell{x:-16, y:0},  Cell{x:-16, y:-1},
            Cell{x:-17, y:0}, Cell{x:-17, y:-1}, Cell{x:17, y:-2},
            Cell{x:17, y:-3}, Cell{x:18, y:-2},  Cell{x:18, y:-3},
        )
    );

    //let mut gosper = Life(vec!(Cell{x:0, y:0}));

    println!("{gosper}");

    let mut _n = String::new();
    io::stdin().read_line(&mut _n).expect("failed to readline");


    loop {

        let one_sec = time::Duration::from_millis(250);
        thread::sleep(one_sec);
        
        // remove previous print?

        gosper = gosper.eval();
        println!("{gosper}"); 
    }

}