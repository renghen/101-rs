//! Adapted from https://github.com/ferrous-systems/teaching-material/blob/main/assignments/serde-lifetimes.adoc

use serde::{Deserialize, Serialize};
use serde_json::Result;

/// pretend that we call an API and get a JSON String back
fn fetch_data() -> String {
    String::from(
        r#"
            {
                "id": 1,
                "title": "Hello, Rust"
            }
        "#,
    )
}

#[derive(Debug,Deserialize,Serialize)]
struct BlogPost {
    id: u32,
    title: String,
}


fn main() -> Result<()> {
    let post: BlogPost = {
        let data = fetch_data();
        serde_json::from_str(data.as_str())?
    };
    println!("deserialized = {:?}", post);

    let post_json: String = serde_json::to_string(&post)?;
    println!("serialized = {:?}", post_json);

    Ok(())
}
