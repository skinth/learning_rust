use std::io::{prelude::*};
use std::iter::FromIterator;

fn main() {
    println!("Anagram app!");

    let mut word = String::new();

    println!("Your word: ");
    std::io::stdin().read_line(&mut word).expect("Failed to read the word!");
    println!("You typed: {}", word);
    let mut sorted_word: Vec<char> = word.trim().chars().collect();
    sorted_word.sort_by(|a,b| a.cmp(b));
    word = String::from_iter(sorted_word);

    //unwrap(): not explicitly handling error
    let _f = std::fs::File::open("src/bin/dictionary.txt").unwrap();
    let lines = std::io::BufReader::new(_f).lines();
    let mut anagrams: Vec<String> = Vec::new();
    for line in lines {
        let l = line.unwrap();
        let mut chs: Vec<char> = l.chars().collect();
        chs.sort_by(|a, b| a.cmp(b));
        let s = String::from_iter(chs);
        if s==word {
            anagrams.push(l);
        }
    }

    println!("{:?}", anagrams);
}
