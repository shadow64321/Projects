use std::{io::{self, Read}, clone};
use rand::prelude::*;

//always have a default(0) to help with the paralisis part
const PARALISIS_TABLE:[ &str; 3 ] = [ "Time_Cafe", "Identity_Cafe", "Food_Cafe" ];
const FOOD_CAFE:[ &str; 4 ] = [ "Veggie Hut", "Tachi Veggie", "Dupling Kitchen", "GYG" ];
const TIME_CAFE:[ &str; 6 ] = [ "0) Japanese", "1) Programming", "2) Music", "3) Cooking", "4) Gaming backlog clearing", "5) Art" ];
const JAPANESE_CAFE:[ &str; 6 ] = [ "0) Immersion", "1) Vocab and Kanji", "2) Writing", "3) Reading", "4) Speaking", "5) Listening" ];
const GAME_BACKLOG_CAFE:[ &str; 9 ] = [ "Nioh 2", "Wo Long: Fallen Dynasty", "Honkai: Star Rail", "Genshin Impact", "Tales of Arise", "Naraka Bladepoint", "Mirrors Edge", "Dark Souls 3", "Metal Gear Rising: Revengence" ];
const MUSIC_CAFE:[ &str; 4] = [ "0) Lesson song practice", "1) Ensemble song practice", "2) song Writing", "Technique Practice" ];
const COOKING_CAFE:[ &str; 1] = [ "0) Mochi" ];
const IDENTITY_CAFE:[ &str; 6] = [ "0) Yor Forger", "1) Inosuke Hashibira", "2) Jungwon", "3) Dongfang Qingcang", "4) Battosai", "5) Jing Yuan" ];
const ART_CAFE:[ &str; 4] = [ "1) Drawing", "2) 3D Modeling", "3) anatomy practice", "4) Hand Drawing Practice" ];
const PROGRAMMING_CAFE:[ &str; 6 ] = [ "Learn Zig programming language", "learn Assembly programming language", "learn Lobster programming language", "learn Monkey programming language", "Make a 3D Cellular Automata", "Program A JRPG" ];

fn main() {
    println!("Please remember capitals at the start of every instruction.\n");
    println!("Capital letters are also required for catagories!!!\n");
    
    println!("{:#?}\n", PARALISIS_TABLE);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            //tests
            //println!("{n} bytes read");
            //println!("{input}");
            let input_data_buf: String = String::from(input.clone());
            //trim input_data_buf by 2 bytes from the end to remove '\r\n'
            let input_data_semi_buf = &input_data_buf[0..input_data_buf.len()-2];
            let input_data: &str = &input_data_semi_buf;
            //tests
            //println!("{:#?}", input_data_semi_buf);
            //println!("{:#?}", input_data_buf);
            

            for i in 0..PARALISIS_TABLE.len() {
                let slice = PARALISIS_TABLE[i];
                let sv: String = String::from(slice);
                let iv: String = String::from(input_data);
                
                //Test
                //println!("{:#?}\n", sv.eq_ignore_ascii_case(&iv));
                
                if sv == iv {
                    if iv  == "Time_Cafe" {
                        println!("Would you like a random catagory chosen?");
                        let mut task_chosen_query = String::new();
                        match io::stdin().read_line(&mut task_chosen_query) {
                            Ok(n) => {
                                let task_chosen_query_buf = task_chosen_query;
                                if task_chosen_query_buf.contains("Yes") {
                                    let time_num = rand::thread_rng().gen_range(0..TIME_CAFE.len());
                                    for i in 0..TIME_CAFE.len() { 
                                        if time_num == i {
                                            let time_slice = TIME_CAFE[i];
                                            println!("{:#?}\n", time_slice);
                                            println!("Would you like to expand this catagory?");
                                            let mut catagory_expansion_query = String::new();
                                            match io::stdin().read_line(&mut catagory_expansion_query) {
                                                Ok(n) => {
                                                    let catagory_expansion_query_buf = catagory_expansion_query;
                                                    if catagory_expansion_query_buf.contains("Yes") {
                                                        if time_slice == "0) Japanese" {
                                                            println!("Would you like a random task chosen?");
                                                            let mut rand_catagory_chosen_query = String::new();
                                                            match io::stdin().read_line(&mut rand_catagory_chosen_query) {
                                                                Ok(n) => {
                                                                    let japanese_number = rand::thread_rng().gen_range(0..JAPANESE_CAFE.len());
                                                                    if rand_catagory_chosen_query.contains("Yes") {
                                                                        for i in 0..JAPANESE_CAFE.len() {
                                                                            if i == japanese_number {
                                                                                let japanese_slice = JAPANESE_CAFE[i];
                                                                                println!("{:#?}", japanese_slice);
                                                                                break;
                                                                            } else {
                                                                                println!("Error in random num gen");
                                                                                println!("{:#?}", JAPANESE_CAFE);
                                                                                break;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(error) => println!("error: {error}\n"),
                                                            } 
                                                        }

                                                        if time_slice == "1) Programming" {
                                                            println!("Would you like a random task chosen?");
                                                            let mut rand_catagory_chosen_query = String::new();
                                                            match io::stdin().read_line(&mut rand_catagory_chosen_query) {
                                                                Ok(n) => {
                                                                    let programming_number = rand::thread_rng().gen_range(0..PROGRAMMING_CAFE.len());
                                                                    if rand_catagory_chosen_query.contains("Yes") {
                                                                        for i in 0..PROGRAMMING_CAFE.len() {
                                                                            if i == programming_number {
                                                                                let programming_slice = PROGRAMMING_CAFE[i];
                                                                                println!("{:#?}", programming_slice);
                                                                                break;
                                                                            } else {
                                                                                println!("Error in random num gen");
                                                                                println!("{:#?}", PROGRAMMING_CAFE);
                                                                                break;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(error) => println!("error: {error}\n"),
                                                            }
                                                        }

                                                        if time_slice == "2) Music" {
                                                            println!("Would you like a random task chosen?");
                                                            let mut rand_catagory_chosen_query = String::new();
                                                            match io::stdin().read_line(&mut rand_catagory_chosen_query) {
                                                                Ok(n) => {
                                                                    let music_number = rand::thread_rng().gen_range(0..MUSIC_CAFE.len());
                                                                    if rand_catagory_chosen_query.contains("Yes") {
                                                                        for i in 0..MUSIC_CAFE.len() {
                                                                            if i == music_number {
                                                                                let music_slice = MUSIC_CAFE[i];
                                                                                println!("{:#?}", music_slice);
                                                                                break;
                                                                            } else {
                                                                                println!("Error in random num gen");
                                                                                println!("{:#?}", MUSIC_CAFE);
                                                                                break;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(error) => println!("error: {error}\n"),
                                                            }
                                                        }

                                                        if time_slice == "3) Cooking" {
                                                            println!("Would you like a random task chosen?");
                                                            let mut rand_catagory_chosen_query = String::new();
                                                            match io::stdin().read_line(&mut rand_catagory_chosen_query) {
                                                                Ok(n) => {
                                                                    let cooking_number = rand::thread_rng().gen_range(0..COOKING_CAFE.len());
                                                                    if rand_catagory_chosen_query.contains("Yes") {
                                                                        for i in 0..COOKING_CAFE.len() {
                                                                            if i == cooking_number {
                                                                                let cooking_slice = COOKING_CAFE[i];
                                                                                println!("{:#?}", cooking_slice);
                                                                                break;
                                                                            } else {
                                                                                println!("Error in random num gen");
                                                                                println!("{:#?}", COOKING_CAFE);
                                                                                break;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(error) => println!("error: {error}\n"),
                                                            }
                                                        }

                                                        if time_slice == "4) Gaming backlog clearing" {
                                                            println!("Would you like a random task chosen?");
                                                            let mut rand_catagory_chosen_query = String::new();
                                                            match io::stdin().read_line(&mut rand_catagory_chosen_query) {
                                                                Ok(n) => {
                                                                    let gaming_backlog_number = rand::thread_rng().gen_range(0..GAME_BACKLOG_CAFE.len());
                                                                    if rand_catagory_chosen_query.contains("Yes") {
                                                                        for i in 0..GAME_BACKLOG_CAFE.len() {
                                                                            if i == gaming_backlog_number {
                                                                                let gaming_backlog_slice = GAME_BACKLOG_CAFE[i];
                                                                                println!("{:#?}", gaming_backlog_slice);
                                                                                break;
                                                                            } else {
                                                                                println!("Error in random num gen");
                                                                                println!("{:#?}", GAME_BACKLOG_CAFE);
                                                                                break;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(error) => println!("error: {error}\n"),
                                                            }
                                                        }

                                                        if time_slice == "5) Art" {
                                                            println!("Would you like a random task chosen?");
                                                            let mut rand_catagory_chosen_query = String::new();
                                                            match io::stdin().read_line(&mut rand_catagory_chosen_query) {
                                                                Ok(n) => {
                                                                    let art_number = rand::thread_rng().gen_range(0..ART_CAFE.len());
                                                                    if rand_catagory_chosen_query.contains("Yes") {
                                                                        for i in 0..ART_CAFE.len() {
                                                                            if i == art_number {
                                                                                let art_slice = ART_CAFE[i];
                                                                                println!("{:#?}", art_slice);
                                                                                break;
                                                                            } else {
                                                                                println!("Error in random num gen");
                                                                                println!("{:#?}", ART_CAFE);
                                                                                break;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(error) => println!("error: {error}\n"),
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(error) => println!("error: {error}\n"),
                                            }
                                        }
                                    }
                                } else {
                                    println!("Error in random num gen");
                                    println!("{:#?}\n", TIME_CAFE);
                                }
                            }
                            Err(error) => println!("error: {error}\n"),
                        }
                        break;
                    } else if iv == "Identity_Cafe" {
                        println!("Would you like a random task chosen?");
                        let mut task_chosen_query = String::new();
                        match io::stdin().read_line(&mut task_chosen_query) {
                            Ok(n) => {
                                let task_chosen_query_buf = task_chosen_query;
                                if task_chosen_query_buf.contains("Yes") {
                                    let identity_num = rand::thread_rng().gen_range(0..IDENTITY_CAFE.len());
                                    for i in 0..IDENTITY_CAFE.len() { 
                                        if identity_num == i {
                                            let identity_slice = IDENTITY_CAFE[i];
                                            println!("{:#?}", identity_slice);
                                            break;
                                        }
                                    }
                                } else {
                                    println!("Error in random num gen");
                                    println!("{:#?}\n", IDENTITY_CAFE);
                                }
                            }
                            Err(error) => println!("error: {error}\n"),
                        }
                        break;
                    } else if iv == "Food_Cafe" {
                        println!("Would you like a random task chosen?");
                        let mut task_chosen_query = String::new();
                        match io::stdin().read_line(&mut task_chosen_query) {
                            Ok(n) => {
                                let task_chosen_query_buf = task_chosen_query;
                                if task_chosen_query_buf.contains("Yes") {
                                    let food_num = rand::thread_rng().gen_range(0..FOOD_CAFE.len());
                                    for i in 0..FOOD_CAFE.len() { 
                                        if food_num == i {
                                            let food_slice = FOOD_CAFE[i];
                                            println!("{:#?}", food_slice);
                                            break;
                                        }
                                    }
                                } else {
                                    println!("Error in random num gen");
                                    println!("{:#?}\n", FOOD_CAFE);
                                }
                            }
                            Err(error) => println!("error: {error}\n"),
                        }
                        break;
                    }
                } 
            }
        }
        Err(error) => println!("error: {error}\n"),
    }
}