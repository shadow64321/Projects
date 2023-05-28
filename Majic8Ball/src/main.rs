use rand::prelude::*;
use std::io;


const FATE:[ &str; 8 ] = [ "Yes", "No", "Maybe", "Just Don't", "Definately", "Not the best idea you've ever had", "One of Your finer ideas", "" ];

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    println!("Math will now predict your fate!");
    let math = rand::thread_rng().gen_range(0..FATE.len());
    for i in 0..FATE.len() {
        if i == math {
            let slice = FATE[i];
            println!("{:#?}", slice);
        }
    }
}
