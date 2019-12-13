use std::io::{self, BufRead};
use json;

fn main() {
    let stdin = io::stdin();
    let mut coords: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.starts_with("\"checksum") {
            let v: Vec<&str> = line.split(' ').collect();
            let name = String::from(v[1]);
            let ver = String::from(v[2]);
            coords.push(format!("crate/cratesio/-/{}/{}", name, ver));
        }
    }
    println!("{}", json::stringify(coords));
}