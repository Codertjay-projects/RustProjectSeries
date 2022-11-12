use serde::{Deserialize, Serialize};

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
        },

        ]
    }"#;

    let parsed: Article = read_json_typed(json);
    println!("\n\n The name of the first paragraph is :{}", parsed.paragraph[0].name);


    println!("Hello, world!");
}
