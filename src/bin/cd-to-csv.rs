use std::io::{self, BufRead, Read};
use json;
use reqwest;

fn print_lic(coord: String) {
    let url = format!("{}{}",
        "https://api.clearlydefined.io/definitions/",
        coord);
    let mut res = reqwest::get(&url)
        .expect("Get ok");
    let mut body = String::new();
    res.read_to_string(&mut body)
        .expect("Body ok");
    let def = json::parse(&body)
        .expect("parse ok");
    println!("{},{},{}",
        def["coordinates"]["name"],
        def["coordinates"]["revision"],
        def["licensed"]["declared"]);
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input)
        .expect("Read ok");
    let mut parsed = json::parse(&input)
        .expect("Parse ok");
    println!("name,revision,license");
    for line in parsed.members_mut() {
        print_lic(line.take_string().expect("string ok"));
    }
}