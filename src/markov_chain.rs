
use std::rand::random;
use std::collections::HashMap;
use regex;

pub struct MarkovChainer<'a> {
    order: uint,
    beginnings: Vec<Vec<&'a str>>,
    freq: HashMap<(&'a str, &'a str), Vec<&'a str>>,
}

impl<'a> MarkovChainer<'a> {
    pub fn new(order: uint) -> MarkovChainer<'a> {
        MarkovChainer {
            order: order,
            beginnings: vec!(vec!()),
            freq: HashMap::new(),
        }
    }

    pub fn add_sentence(&mut self, string: &'a str, terminator: &'a str) {
        let mut words: Vec<&str> = string.split(' ').collect();
        if words.len() > self.order {
            words.push(terminator);
            let mut vec = vec!();
            vec.push_all(words.slice(0, self.order));
            self.beginnings.push(vec);
        }

        let mut buf: Vec<&str> = vec!();
        for word in words.iter() {
            buf.push(word.clone());
            let size = buf.len();
            if size == self.order + 1 {
                let key = (buf[0].clone(), buf[size - 2].clone());
                if let Some(value) = self.freq.clone().get_mut(&key) {
                    value.push(buf[size - 1].clone());
                } else {
                    self.freq.insert(key, vec!(buf[size - 1].clone()));
                }
                buf.remove(0);
            } else {
                continue
            }
        }
        
    }

}


