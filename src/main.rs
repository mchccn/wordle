#![allow(dead_code, unused)]

use clearscreen::clear;
use colored::*;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::fs::{canonicalize, read_to_string};
use std::io::{self, BufRead, Lines, StdinLock};
use std::path::PathBuf;

type Wordle = [[char; 5]; 6];

fn words() -> Vec<String> {
    include_str!("../words.txt")
        .lines()
        .map(|x| String::from(x))
        .collect::<Vec<String>>()
}

fn random_word(words: Vec<String>) -> String {
    words.choose(&mut thread_rng()).unwrap().to_string()
}

fn print_wordle(wordle: Wordle, word: &String) {
    println!("┏━━━━━━━━━━━━━┓");
    println!("┃ {} ┃", "W O R D L E".bold());
    println!("┣━━━━━━━━━━━━━┫");
    println!("┃             ┃");

    for row in wordle.iter() {
        if row.iter().any(|&x| x != ' ') {
            let mut colored = vec![];

            for (i, c) in row.iter().enumerate() {
                let w = word.chars().nth(i).unwrap();

                if *c == w {
                    colored.push(c.to_string().green());
                } else if word.chars().any(|r| *c == r) {
                    colored.push(c.to_string().yellow());
                } else {
                    colored.push(c.to_string().white());
                }
            }

            println!(
                "┃  {} ┃",
                colored
                    .into_iter()
                    .map(|x| format!("{} ", x))
                    .collect::<Vec<String>>()
                    .join(""),
            );
        } else {
            println!("┃  _ _ _ _ _  ┃");
        }
    }

    println!("┃             ┃");
    println!("┗━━━━━━━━━━━━━┛");
}

fn get_guess(stdin: &mut Lines<StdinLock>, wordle: Wordle, word: &String) -> String {
    let mut guess = stdin.next().unwrap().unwrap();

    while !words().contains(&guess) {
        clear().expect("Failed to clear screen");

        print_wordle(wordle, &word);

        if guess.len() != 5 {
            println!("{}", "Please enter a 5-letter word.".red());
        } else {
            println!("{}", "Word is not in word list.".red());
        }

        guess = stdin.next().unwrap().unwrap();
    }

    guess
}

fn main() {
    clear().expect("Failed to clear screen");

    let word = random_word(words());

    let mut wordle = [[' '; 5]; 6];

    print_wordle(wordle, &word);

    let stdin = io::stdin();

    let mut iterator = stdin.lock().lines();

    let mut guess = "".to_string();

    let mut guesses = 0;

    while guess != word && guesses < 6 {
        if guesses > 0 {
            println!("{}", "Incorrect guess.".red());
        } else {
            println!();
        }

        guess = get_guess(&mut iterator, wordle, &word);

        clear().expect("Failed to clear screen");

        print_wordle(wordle, &word);

        let mut chars = guess.chars();

        wordle[guesses] = [
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
        ];

        clear().expect("Failed to clear screen");

        print_wordle(wordle, &word);

        guesses += 1;
    }

    if guesses == 1 {
        println!("{}", "Correct! How did you do it?".green());
    } else if guesses < 6 {
        println!(
            "{}",
            format!("Correct! You guessed the word in {} guesses.", guesses).green()
        );
    } else {
        println!(
            "{}",
            format!("You didn't guess the word in 6 attempts.\nThe word was \"{}\".\nBetter luck next time!", word).red()
        );
    }
}
