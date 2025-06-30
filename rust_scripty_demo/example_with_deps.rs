//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // 使用 serde 进行 JSON 序列化/反序列化
    let person = Person {
        name: "张三".to_string(),
        age: 25,
    };
    
    // 序列化为 JSON
    let json = serde_json::to_string(&person).unwrap();
    println!("JSON: {}", json);
    
    // 反序列化
    let parsed: Person = serde_json::from_str(&json).unwrap();
    println!("解析结果: {:?}", parsed);
} 