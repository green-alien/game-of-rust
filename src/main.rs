use itertools::Itertools;
use std::io;

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
            Cell{x: xmod(-1), y: self.y  }, /*         origin         */  Cell{x: xmod(1), y: self.y },
            Cell{x: xmod(-1), y: ymod( 1)}, Cell{x: self.x, y: ymod( 1)}, Cell{x: xmod(1), y: ymod( 1)},
        ]
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
            print!(" candidate alive : {:?}", canidate);
            let count = count_alive_neigbors(*canidate);
            println!(" {count}");
            if count == 2 || count == 3 { next_generation.push(*canidate); }
        }

        println!();
    
        // dead cell rule
        for canidate in rel_neighbors.iter() {
            print!(" candidate dead : {:?}", canidate);
            let count = count_alive_neigbors(*canidate);
            println!(" {count}");
            if count == 3 { next_generation.push(*canidate) }
        }
        println!();
        
        Life(next_generation)
    }
}

fn main() {
    
    let mut blinker = Life(
        vec!(
            Cell{x: -1, y: 0}, Cell{x: 0, y: 0}, Cell{x: 1, y: 0}
        )
    );

    println!("{:?}", blinker.0);

    loop {
        blinker = blinker.eval();
        println!("{:?}", blinker.0);

        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("failed to readline");
    }
}