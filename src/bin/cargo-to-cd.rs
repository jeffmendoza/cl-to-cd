// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use json;
use cargo_metadata;

fn main() {
    let mut coords: Vec<String> = Vec::new();
    let mut cmd = cargo_metadata::MetadataCommand::new();
    let metadata = cmd.manifest_path("./Cargo.toml").exec()
        .expect("to parse");
    for package in metadata.packages {
            coords.push(format!("crate/cratesio/-/{}/{}", package.name, package.version));
    }
    println!("{}", json::stringify(coords));
}
