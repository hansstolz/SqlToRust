use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn remove_size(str:&str) -> String {
    str
        .chars()
        .filter(|x| x.is_alphabetic())
        .collect()
}


pub fn load_file_as_lines(file_name: &str) -> Vec<String> {
    let file = match File::open(file_name) {
        Err(why) => panic!("couldn't open {}: {}", file_name, why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let mut tables: Vec<String> = vec![];
    for line in reader.lines() {
        tables.push(line.unwrap());
    }

    tables
}

pub fn write_struct_to_file(path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}


pub fn table_name_to_struct_name(table_name:&str) -> String {
    let mut struct_name = table_name.to_string();
    struct_name = struct_name.replace("_", " ");
    struct_name = struct_name.to_lowercase();
    struct_name = struct_name
        .split_whitespace()
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(c).collect(),
            }
        })
        .collect();
    struct_name
}