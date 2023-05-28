//todo list
//and add pitty system:
//iterate through file and check if there are 50 pulls (each pull is separated by a comma)
//if there is then garentee a 5 star


use rand::prelude::*;
use std::io;
use std::io::Read;
use std::io::Write;
use std::thread;
use std::time;
use std::fs::File;
use std::fs::OpenOptions;

//const FILE = File::open("WishHistory.txt");
const BANNER:[ &str; 50 ] = [ "5-Star", "4-Star", "4-Star", "4-Star", "4-Star", "4-Star", "4-Star", "4-Star", "4-Star", "4-Star", "4-Star", "3-Star", "3-Star", "3-Star", "3-Star", "3-Star", "3-Star", "3-Star", "3-Star", "3-Star", "3-Star", "2-Star", "2-Star", "2-Star", "2-Star", "2-Star", "2-Star", "2-Star", "2-Star", "2-Star", "2-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star", "1-Star" ];

fn main() {
    let file_existance_test = File::open("WishHistory.txt");
    let data_file_exist = match file_existance_test {
        Ok(file) => {
            let mut data_file = File::open("WishHistory.txt").unwrap();
            let mut file_content = String::new();
            data_file.read_to_string(&mut file_content).unwrap();
        },
        Err (error) => {
            let mut data_file = File::create("WishHistory.txt").expect("Gacha history file creation failed");
        }
    };

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
                        print!("Wishing...");
                        thread::sleep(time::Duration::from_secs(5)); 
                        
                        let mut data_file = OpenOptions::new()
                            .append(true)
                            .open("WishHistory.txt")
                            .expect("cannot open file");
                            
                        let slice_as_bytes = slice.as_bytes();
                        data_file.write(slice_as_bytes).expect("Gacha History write failed");    
                        data_file.write(", ".as_bytes()).expect("Gacha History write failed");   

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
