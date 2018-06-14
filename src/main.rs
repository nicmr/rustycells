extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::{Uniform};


fn main() {
    
    // let mut grid = random_grid(10);
    let mut grid = vec![0,0,0,0,1,0,0,0,0]; //Sierpiński triangle
    let generations = 10;
    println!["{:?}", grid];


    for gen in 0..generations {
        grid = next_gen(grid);
        println!["{:?}", grid];
    }
}



//currently one-dimensional
//consider using ndarray
fn random_grid(len: u32) -> Vec<u8>{

    let roll_range = Uniform::new_inclusive(0,1);
    thread_rng().sample_iter(&roll_range).take(10).collect()
}

fn next_gen(old: Vec<u8>)->Vec<u8>{

    let mut new = old.to_vec();

    //border operations
    new[0] = procreate(0, old[0], old[1]).translate();
    new[old.len()-1] = procreate(old[old.len()-2], old[old.len()-1], 0).translate();

    //regular operations
    for cell_ind in 1..(old.len()-1) {
        let genome = procreate(old[cell_ind-1], old[cell_ind], old[cell_ind+1]);
        // println!["{}'s genome is {}", cell_ind, genome.code]; //just for debugging
        new[cell_ind] = genome.translate();
    }
    new
}


fn procreate(a: u8, b: u8, c: u8)->Genome{
    Genome{code: 1*a + 2*b + 4*c}
}

struct Genome{
    code: u8,
}



impl Genome{
    fn translate(&self)->u8{
        match self.code{
            0 => 0,
            1 => 1,
            2 => 0,
            3 => 1,
            4 => 1,
            5 => 0,
            6 => 1,
            7 => 0,
            _ => {panic!["you messed up"];}
        }
    }
}

