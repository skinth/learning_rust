use std::io::{prelude::*};
use std::io::Lines;
use std::io::Result;
use std::iter::FromIterator;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

fn main() {
    println!("Anagram app!");

    let mut word = String::new();

    print!("Your word: \n");
    std::io::stdin().read_line(&mut word).expect("Failed to read the word!");
    let mut sorted_word: Vec<char> = word.trim().chars().collect();
    sorted_word.sort_by(|a,b| a.cmp(b));
    word = String::from_iter(sorted_word);

    println!("{:?}", find_anagram(word));
}

fn find_anagram(w: String) -> Vec<String> {

    let mut anagrams: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("src/bin/dictionary.txt") {
        for line in lines {
            if let Ok(current_word) = line {
                let mut chs: Vec<char> = current_word.chars().collect();
                chs.sort_by(|a, b| a.cmp(b));
                let s = String::from_iter(chs);
                if s==w {
                    anagrams.push(current_word);
                }
            }
        }
    }
    anagrams
}

fn read_lines(path: &str) -> Result<Lines<BufReader<File>>> {
    //unwrap(): not explicitly handling error
    let mut _f = match File::open(path) {
        Err(e) => panic!("Error opening file {}: {}", path, e.description()),
        Ok(file) => file,
    };

    Ok(BufReader::new(_f).lines())
}
