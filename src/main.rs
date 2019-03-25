extern crate regex;
mod humanizer;
use std::io;
use std::str::{self, FromStr};
use regex::Regex;

fn main() {
    let digits_only_validator = Regex::new(r"^\d+,?\d{0,2}$")
        .unwrap();
    let terminator = String::from("q");
    loop {
        println!("Input a number:");
        let mut current = String::default();
        io::stdin().read_line(&mut current)
            .expect("failed to read input...");
        current = current.trim_end_matches("\n").to_owned();
        if current.eq(&terminator){
            println!("Bye!");
            break;
        }
        if !digits_only_validator.is_match(&current) 
        {
            println!("Wrong input!");
            continue;
        }
        if current.contains(",")
        {
            let split: Vec<&str> = current.split(",").collect();
            let dollars = split[0];
            let cent = split[1];
            println!("{} dollars and {} cents",
                humanizer::convert_into_text(usize::from_str(dollars).unwrap_or(0)),
                humanizer::convert_into_text(usize::from_str(cent).unwrap_or(0))
            );
        } else {
            println!("{} dollars",
            humanizer::convert_into_text(current.parse::<usize>().unwrap()));
        }
    }
}
