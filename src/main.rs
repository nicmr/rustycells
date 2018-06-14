extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::{Uniform};


fn main() {
    
    let mut grid = random_grid(10);

    println!["{:?}", grid];
}

//currently one-dimensional
//consider using ndarray
fn random_grid(len: u32) -> Vec<u8>{

    let roll_range = Uniform::new_inclusive(0,1);
    thread_rng().sample_iter(&roll_range).take(10).collect()
}
