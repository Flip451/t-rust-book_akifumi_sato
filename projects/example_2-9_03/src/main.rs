use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() -> Result<()> {
    let user = User {
        name: String::from("sato"),
        age: 30,
    };

    // `User` を JSON 文字列に変換
    let serialized = serde_json::to_string(&user)?;
    println!("serialized = {}", serialized);

    // JSON 文字列を `User` に変換
    let deserialized: User = serde_json::from_str(&serialized)?;
    println!("{:?}", deserialized);

    Ok(())
}
