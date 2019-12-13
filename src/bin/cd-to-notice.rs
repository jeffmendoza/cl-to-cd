// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::io::{self, BufRead, Read};
use json;
use reqwest;
use reqwest::header::{HeaderMap, CONTENT_TYPE};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input)
        .expect("Read ok");
    let parsed = json::parse(&input)
        .expect("Parse ok");
    let data = json::object!{
        "coordinates" => parsed,
        "options" => json::object!{},
    };

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    let client = reqwest::Client::new();
    let mut res = client.post("https://api.clearlydefined.io/notices")
        .headers(headers)
        .body(data.dump())
        .send()
        .expect("post ok");
    let mut body = String::new();
    res.read_to_string(&mut body)
        .expect("Body ok");
    let notice = json::parse(&body)
        .expect("parse ok");
    println!("{}", notice["content"]);
    eprintln!("{}", notice["summary"]);
}
