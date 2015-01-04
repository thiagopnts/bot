#![feature(phase)]
extern crate regex;

#[phase(plugin)]
extern crate regex_macros;

use std::collections::HashMap;

mod markov_chain;

#[test]
fn it_works() {
    let text = "\nola\nmeu nome é thiago\nyeah!";
    let re = regex!(r"\n");
    let returned = re.replace_all(text, ".");
    println!("{}", returned);
    let seps = regex!("([.!?;:])");
    let mut splits: Vec<&str> = seps.split(returned.as_slice()).collect();
    println!("\n");
    println!("{}", splits.len());
    println!("{}", splits);
}
