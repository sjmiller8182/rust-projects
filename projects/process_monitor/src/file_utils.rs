
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Error;
use std::io::{Lines, BufReader, BufRead};

pub fn read_file_to_string(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}

pub fn read_n_lines(path: &str, n: u32) -> Result<Vec<String>, Error> {
    let mut lines: Vec<String> = Vec::new();
    let mut buf = String::new();
    
    let file = File::open(path)?;
    let mut read_buffer = BufReader::new(file);

    for _ in 0..n {
        buf.clear();
        let _buf = read_buffer.read_line(&mut buf)?;
        lines.push(buf.trim().to_string())
    }

    Ok(lines)
}

pub fn iter_lines(path: &str) -> Result<Lines<BufReader<File>>, Error> {
    //
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

pub fn read_as_hashmap(path: &str, delimiter: &str) -> Result<HashMap<String, String>, Error>{
    //
    let mut hm = HashMap::new();
    let lines = iter_lines(path)?;
    for line in lines {
        let line = line?;
        let split_loc = line.find(delimiter).unwrap();
        let (key, value) = line.split_at(split_loc);

        let key = key.to_string();
        let value = value
            .replace(delimiter, "")
            .trim()
            .to_string();
        
        hm.entry(key).or_insert(value);
    }
    Ok(hm)
}
