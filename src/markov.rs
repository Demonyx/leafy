use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader, Error},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn train(filenames: Vec<String>) {
    let mut corpus = Vec::new();
    let mut model: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for file in filenames {
        corpus.extend(lines_from_file(file));
    }
    for name in corpus {
        let mut previous_char = "START".to_string();
        for chars in name.chars() {
            model
                .entry(previous_char)
                .and_modify(|counts| {
                    counts
                        .entry(chars.to_string())
                        .and_modify(|count| {
                            *count += 1;
                        })
                        .or_insert(1);
                })
                .or_insert(HashMap::from([(chars.to_string(), 1)]));
            previous_char = chars.to_string();
        }
        model
            .entry(previous_char)
            .and_modify(|counts| {
                counts
                    .entry("END".to_string())
                    .and_modify(|count| {
                        *count += 1;
                    })
                    .or_insert(1);
            })
            .or_insert(HashMap::from([("END".to_string(), 1)]));
    }
    // temporary until db is set up
    let mut file = File::create("../corpus/model").expect("Cound not open file");
    let model_json = serde_json::to_string(&model).unwrap();
    file.write_all(model_json.as_bytes()).unwrap();
}

fn find_next_token(counts: HashMap<String, i32>) -> String {
    let mut options = Vec::new();
    for token in counts.keys() {
        let mut i = *counts.get(token).unwrap();
        while i > 0 {
            options.push(token);
            i -= 1;
        }
    }
    options.choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn generate(num: i32) -> Vec<String> {
    // temporary until db is set up
    let file = File::open("../corpus/model").expect("No such file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Could not read file");

    let data: HashMap<String, HashMap<String, i32>> =
        serde_json::from_str(&contents).expect("Error converting to JSON");

    let mut names = Vec::new();
    for n in 0..num {
        let mut current_token = "START".to_string();
        let mut new_name = String::new();
        while current_token != "END" {
            new_name.push_str(&current_token);
            current_token = find_next_token(data.get(&current_token).unwrap().clone());
        }
        new_name = new_name[5..].to_string();
        // println!("{}", new_name);
        names.push(new_name);
    }
    names
}
