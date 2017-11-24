extern crate rand;

use std::fs::File;
use std::io::Read;
use std::env;

use rand::distributions::{IndependentSample, Range};

struct MarkovModel<'a> {
    starts: Vec<&'a str>,
    pairs: Vec<(&'a str, &'a str)>,
}

fn train(text: &String) -> MarkovModel {
    let mut pairs = Vec::new();
    let mut starts = Vec::new();

    for line in text.lines() {
        let iter1 = line.split_whitespace();
        let iter2 = line.split_whitespace().skip(1);
        let word_pairs = iter1.zip(iter2);
        for (i, (first, next)) in word_pairs.enumerate() {
            if i == 0 {
                starts.push(first);
            }
            pairs.push((first, next));
        }
    }

    MarkovModel {
        starts: starts,
        pairs: pairs,
    }
}

fn get_random_word<'a>(words: &Vec<&'a str>, rng: &mut rand::ThreadRng) -> &'a str {
    let idx_range = Range::new(0, words.len());
    words[idx_range.ind_sample(rng)]
}

fn generate(model: &MarkovModel, n_words: u32) -> String {
    let mut result = String::new();
    let mut rng = rand::thread_rng();

    let mut curr_word = get_random_word(&model.starts, &mut rng);
    result.push_str(curr_word);
    result.push(' ');

    let mut options: Vec<&str> = Vec::new();
    for _ in 0..n_words-1 {
        for pair in &model.pairs {
            if pair.0 == curr_word {
                options.push(pair.1);
            }
        }
        if options.len() == 0 {
            break;
        } else {
            curr_word = get_random_word(&options, &mut rng);
            options.truncate(0);
        }
        result.push_str(curr_word);
        result.push(' ');
    }

    result
}

fn main() {
    let mut buf = String::new();
    // Usage: markov <filename1> [<filename2> ...]
    for arg in env::args().skip(1) {
        let mut file = File::open(arg).expect("could not open file");
        file.read_to_string(&mut buf).expect("could not read {}");
    }
    let model = train(&buf);
    println!("{}", generate(&model, 20));
}
