use serde::{Deserialize, Serialize};
use serde_json;

use neuro_arg::open_file;

#[derive(Debug, Deserialize, Serialize)]
struct Signatures {
    bytes: String,
    desc: Vec<Extension>
}

#[derive(Debug, Deserialize, Serialize)]
struct Extension {
    extension: String,
    description: String
}

fn main() {
    _get_extension();
}

fn _get_extension() {
    let contents = open_file("others/signatures.json");
    let mut bytes = String::new();
    match contents {
        Ok(c) => bytes = c,
        Err(e) => {
            println!("File not found, or corrupted contents. Error: {}", e)
        }
    };

    let json_vec: Vec<Signatures> = Vec::new();
    for i in bytes.lines() {
        let sig: Signatures;
        if i.contains('[') {
            let (byte, _) = i.split_at(i.len() - 4);
            let (_, byte) = byte.split_at(5);
            println!("{}", byte);
            continue;
        }
        if i.contains("extension") {
            let (_, extension) = i.split_at(26);
            let (extension, _) = extension.split_at(extension.len() - 2);
            println!("{}", extension);
            continue;
        }
        if i.contains("description") {
            let (_, description) = i.split_at(28);
            let (description, _) = description.split_at(description.len() - 1);
            println!("{}", description);
            continue;
        }
    }
}

fn _filter_hash() {
    let contents = open_file("lyrics/numbers");
    let mut lyrics = String::new();
    match contents {
        Ok(c) => lyrics = c,
        Err(e) => {
            println!("File not found, or corrupted contents. Error: {}", e)
        }
    };

    let contents = open_file("encrypted/numbers_desc");
    let mut hash_desc = String::new();
    match contents {
        Ok(c) => hash_desc = c,
        Err(e) => {
            println!("File not found, or corrupted contents. Error: {}", e)
        }
    };


    let soundcloud = "c291bmRjbG91ZAo=";
    let mut s = String::new();

    for i in soundcloud.chars() {
        s.push(i);
        println!("{}", lyrics.contains(&s));
    }

    let (_before, start_by_2) = hash_desc.split_at(129);

    println!("{}", start_by_2);

    for i in start_by_2.as_bytes() {
        if i.to_string().contains("5") || i.to_string().contains("7") || i.to_string().contains("2") || i.to_string().contains("9") ||
           i.to_string().contains("4") || i.to_string().contains("3") {
            print!("{}", *i as char);
        }
    }
}
