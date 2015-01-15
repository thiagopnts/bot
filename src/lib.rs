extern crate regex;

use std::collections::HashMap;
use regex::Regex;

mod markov_chain;

#[test]
fn it_works() {
    let text = "\nola\nmeu nome é thiago\nyeah!";
    let re = Regex::new(r"\n");
    let returned = re.replace_all(text, ".");
    println!("{}", returned);
    let seps = Regex::new("([.!?;:])");
    let mut splits: Vec<&str> = seps.split(returned.as_slice()).collect();
    println!("\n");
    println!("{}", splits.len());
    println!("{}", splits);
}
