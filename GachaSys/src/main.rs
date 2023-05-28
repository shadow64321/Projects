//todo list
//create a wish history file
//and add pitty system


use rand::prelude::*;
use std::io;

const BANNER:[ &str; 50 ] = [ "5-Star", "4-Star", "4-Star", "4-Star", "4-Star", "4-Star", "4-Star", "4-Star", "4-Star", "4-Star", "4-Star", "3-Star", "3-Star", "3-Star", "3-Star", "3-Star", "3-Star", "3-Star", "3-Star", "3-Star", "3-Star", "2-Star", "2-Star", "2-Star", "2-Star", "2-Star", "2-Star", "2-Star", "2-Star", "2-Star", "2-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star" ];

fn main() {
    println!("Gacha system test");
    println!("Wish (Y/n)");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n)=>{
            let input_buf = input;


            if input_buf.contains("y"){
                let gachaNum = rand::thread_rng().gen_range(0..BANNER.len());

                for i in 0..BANNER.len() {
                    if gachaNum == i {
                        let slice = BANNER[i];
                        println!("{:#?}", slice);
                    } 
                }
            } else {
                panic!("input is invalid");
            }
        }
        Err(error) => { 
            panic!("Error occured"); 
        }

    }
}
