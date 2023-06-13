use std::{fs::File, io::{BufReader, Read}};

pub fn open_file(file_path: &str) -> std::io::Result<String> {
    let file = File::open(file_path)?;
    let mut buff_reader = BufReader::new(file);
    let mut contents = String::new();
    buff_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
