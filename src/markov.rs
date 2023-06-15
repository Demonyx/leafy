use serde::{Deserialize, Serialize};
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

pub fn train(filenames: Vec<String>) -> Result<(), Error> {
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
    let mut file = File::create("../corpus/model")?;
    let model_json = serde_json::to_string(&model).unwrap();
    file.write_all(model_json.as_bytes())
}

pub fn generate(num: i32) {
    // temporary until db is set up
    let file = File::open("../corpus/model").expect("No such file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Could not read file");

    print!("{}", contents);
}
