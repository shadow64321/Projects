//use std::io::Read;
use std::process::exit;
use std::time;
use std::{io::stdin, thread::sleep};
//use rand::prelude::*;


static mut DOING_STUFF: bool = false;
// static mut inventory: Vec = vec::new();

fn main() {
    let mut main_menu_req_load = false;
    let mut weather_cycle_step_val = 0.0;
    let mut wildlife_migration_cycle_step_val = 0.0;
    let mut season = String::new();
    let mut season_cycle_step = 0;

    println!("Welcome to WolfDen");
    main_menu_req_load = true;
    if main_menu_req_load == true {
        main_menu_load();
        main_menu_req_load = false;
    }
}

fn main_menu_load() {
    let mut input = String::new();
    let opt1 = "1".to_string();
    let opt2 = "2".to_string();

    println!("WolfDen\n1|Start Game\n2|Caracter Customisation\n");
    stdin().read_line(&mut input).expect("where your input??");
    
    match input {
        opt1 => start_game(),
        opt2 => character_customiser(),
        _ => exit(1),
    }
}

fn start_game() {
    println!("Game Loading");
    game_loop();
}

fn character_customiser() {
    println!("Character Customiser Loading");
}

fn game_loop() {
    let ten_millis = time::Duration::from_millis(10);
    let mut input = String::new();
    let options: [String; 5] = ["1| Story".to_owned(), "2| Eat".to_owned(), "3| Sleep".to_owned(), "4| Explore".to_owned(), "5| Cultivate".to_owned()];

    if unsafe { DOING_STUFF } == false {
        sleep(ten_millis);
        println!("What do you want to do?");
        println!("{:#?}", options);
        stdin().read_line(&mut input).expect("Where input??");


        //switch stament doesn't work here so forced to do this the slow way
        if input == "1".to_string() {
            story();
        } 
        
        if input == "2".to_string() {
            eat();
        } 
        
        if input == "3".to_string() {
            sleepy_time();
        } 
        
        if input == "4".to_string() {
            explore();
        } 
        
        if input == "5".to_string() {
            cultivate();
        }
    }
}

fn story() {

}

fn eat() {

}

fn sleepy_time() {
    let mut days_spent_in_game = 0;
    let seven_seconds = time::Duration::from_millis(7000);
    sleep(seven_seconds);
    println!("You slept calmy");
    days_spent_in_game = days_spent_in_game + 1;
    println!("Day {:#?}", days_spent_in_game);
}

fn explore() {

}

fn cultivate() {

}