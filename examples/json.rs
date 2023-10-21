use serde_json::{Result, Value};
use ijson::{IValue};

fn main() {

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let v: Value = serde_json::from_str(data).unwrap();

    println!("name: {}, number: {}", v["name"], v["phones"][0]);

    let iv: IValue = serde_json::from_str(data).unwrap();

    println!("ijson name: {:?}, number: {:?}", iv["name"], iv["phones"][0]);

    println!("ijson value is object: {}", iv.is_object());
}