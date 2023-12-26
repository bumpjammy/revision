#![feature(fs_try_exists)]

use glob::{glob, GlobResult};
use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;
use std::io::{stdin, Write};
use std::path::PathBuf;
use std::{env, fs, io};

fn main() {
    let file_path = get_file_path_string();
    loop {
        let mut files: Vec<GlobResult> = glob(&*(file_path.to_owned() + "**/*.md"))
            .expect("Failed to read glob pattern")
            .collect();
        if files.is_empty() {
            panic!("This directory does not contain any markdown files! Is it correct?");
        }
        files.shuffle(&mut thread_rng());
        loop_files(&file_path, files);
    }
}

fn loop_files(dir_path: &String, files: Vec<GlobResult>) {
    files
        .into_iter()
        .filter_map(Result::ok)
        .for_each(|path| study_file(dir_path, path));
}

fn get_file_path_string() -> String {
    let default_path = "~/Documents/Obsidian/Example/";
    let args: Vec<String> = env::args().collect();
    let mut file_path = args.get(1).map_or(default_path, |path| path).to_string();
    if !file_path.ends_with('/') {
        file_path.push('/');
    }
    fs::read_dir(&file_path).expect("File path doesn't exist! Did you pass one in the argument?");
    file_path
}

fn study_file(file_path: &str, path: PathBuf) {
    let contents_string = fs::read_to_string(path.clone()).expect("Failed to read file");
    let mut words = get_words_between_stars(contents_string.as_str());

    while !&words.is_empty() {
        let current_text = replace_words(contents_string.clone(), &mut words);
        print_file(file_path, &path, current_text);
        let input = get_word_input();
        update_words(&mut words, input);
    }
}

fn get_word_input() -> String {
    println!("Enter a word: ");
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    format!("**{}**", line.trim())
}

fn update_words(words: &mut Vec<String>, input: String) {
    if words.contains(&input) {
        print!("\x07"); // Bell sound
        words.retain(|x| !x.eq(&input));
    }
}

fn replace_words(mut contents: String, words: &Vec<String>) -> String {
    for word in words {
        contents = contents.replace(word.as_str(), "____");
    }
    contents
}

fn print_file(file_path: &str, path: &PathBuf, line_with_keywords_removed: String) {
    let formatted_path = if let Some(path_str) = path.to_str() {
        path_str.replace(file_path, "")
    } else {
        String::new()
    };

    let esc = 27 as char;
    print!(
        "{esc}[2J{esc}[H{esc}[1mStudying: {:?}{esc}[22m\n\n{}{esc}[30;0H",
        formatted_path, line_with_keywords_removed,
    );
    io::stdout().flush().unwrap();
}

fn get_words_between_stars(str: &str) -> Vec<String> {
    let mut words = vec![];
    let re = Regex::new(r"\*\*(.*?)\*\*").unwrap();
    let result = re.find_iter(str);
    for word in result {
        words.push(word.as_str().to_string());
    }
    words
}
