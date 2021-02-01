#[macro_use]
extern crate serde_json;

use serde::{self, Serialize};
use format::json::JsonPrettyFormatter;


fn main() {
    let obj = json!({"name": "ryan",
                            "age": 100.5,
                            "man": true,
                            "female": false,
                            "address": {
                                "street": "10 Downing Street",
                                "city": ["New York", "California"]
                            },
                            "phones": [
                                "+44 1234567",
                                "+44 2345678"
                            ]
                        });

    let buf = Vec::new();
    let formatter = JsonPrettyFormatter::new();

    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    obj.serialize(&mut ser).unwrap();

    println!("{}", String::from_utf8(ser.into_inner()).unwrap());
}