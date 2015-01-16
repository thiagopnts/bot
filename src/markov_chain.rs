
use std::rand::random;
use std::collections::HashMap;
use regex::Regex;

pub struct MarkovChainer {
    order: usize,
    beginnings: Vec<Vec<String>>,
    freq: HashMap<(String, String), Vec<String>>,
}

impl MarkovChainer {
    pub fn new(order: usize) -> MarkovChainer {
        MarkovChainer {
            order: order,
            beginnings: vec!(vec!()),
            freq: HashMap::new(),
        }
    }

    pub fn add_sentence(&mut self, string: String, terminator: String) {
        let mut words: Vec<String> = string.split(' ').collect::<Vec<&str>>().iter().map(|s| s.to_string()).collect();
        if words.len() > self.order {
            words.push(terminator);
            let mut vec = vec!();
            vec.push_all(words.slice(0, self.order));
            self.beginnings.push(vec);
        }

        let mut buf: Vec<String> = vec!();
        for word in words.iter() {
            buf.push(word.to_string());
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

    pub fn add_text(&mut self, text: String) {
        let re = Regex::new(r"\n").unwrap();
        let without_newline = re.replace_all(text.as_slice(), ".");

        let seps = Regex::new("([.!?;:])").unwrap();
        let mut pieces = seps.split(without_newline.as_slice());

        let mut sentence = "".to_string();
        for piece in pieces {
            if piece != "" {
                sentence = match seps.find(piece) {
                    Some(_) => {
                        self.add_sentence(sentence, piece.to_string());
                        "".to_string()
                    },
                    None => piece.to_string(),
                };
            }
        }
    }

    fn generate_sentence(&mut self) -> Option<String> {
        let mut res = self.beginnings[random::<usize>() % self.beginnings.len()].clone();
        if res.len() == self.order {
            let mut nw = false;
            loop {
                let restup = (res[res.len() - 2].clone(), res[res.len() - 1].clone());
                match self.next_word_for(restup) {
                    Some(word) => res.push(word),
                    None => continue,
                }
            }
            let mut new_res = res.slice(0, res.len() - 2);
            //TODO: check if is title, capitalize etc
            let mut sentence = "".to_string();
            for word in new_res.iter() {
                sentence.push_str(word.as_slice());
                sentence.push(' ');
            }

            sentence.push_str(res[res.len() - 2].clone().as_slice());
            sentence.push_str(res[res.len() - 1].clone().as_slice());
            Some(sentence)
        } else {
            None
        }
    }

    fn next_word_for(&mut self, words: (String, String)) -> Option<String> {
        match self.freq.get(&words) {
            Some(array) => Some(array[random::<usize>() % array.len()].clone()),
            None => None,
        }
    }
}


