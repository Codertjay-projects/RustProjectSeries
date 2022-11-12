#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}


#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    // this method needs to be inside main() method
    // env::set_var("RUST_BACKTRACE", "1");
    let json = r#"
    {
        "article":"How to work with json in rust",
        "author":"Codertjay",
        "paragraph":[
        {
            "name":"Strarting Sentence",
        }, {
            "name":"Body of the paragraph",
        }, {
            "name":"ENd of the paragraph",
        }

        ]
    }"#;

    let parsed: Article = read_json_typed(json);
    println!("\n\n The name of the first paragraph is :{}", parsed.paragraph[0].name);
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}
