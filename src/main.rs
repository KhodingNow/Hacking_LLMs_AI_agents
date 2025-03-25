fn main() {
// Serde's key feature and functionality

use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct Person {
        name: String,
        age: u8,
        is_active: bool,
    }

    let alice = Person {
        name: "Alice".to_string(),
        age: 31,
        is_active: true,
    };


    let bonga = serde_json::to_string(&alice).unwrap();
    println!("Serialized: {}", bonga);

    // Output: {"name": "bonga", "age": 31, "is active": true } 

    // Deserialize back to a Person
    let deserialized: Person = serde_json::from_str(&bonga).unwrap();
    println!("Deserialized: {:?}", deserialized);

    println!("Hello, world!, here is where I am explore the Hacking of LLMs, AI agents with Serde...");


// Customizing structs with Serde attributes
// - Field Renaming and Skipping

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct  User {
    #[serde(rename = "id")] // rename a single field
    user_id: u64,
    email: String,
    #[serde(skip_serializing)] // skip field during serialization
    password_hash: String,
}

let user = User {
    user_id: 123,
    email: "bona@example.com".to_string(),
    password_hash: "hashed".to_string(),
};

let json = serde_json::to_string(&user).unwrap();
println!("Serialized: {}", json);


// Working with enums

// - Basic enums with Variants

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")] // enum type, nmed 'type'
enum Event {
    Login { user_id: u64, timestamp: String },
    Logout { user_id: u64},
    Error(String),
}

//fn main()...
let login_event = Event::Login {
    user_id: 456,
    timestamp: "2025-18-01T12:00:00Z".to_string(),
};

let json = serde_json::to_string(&login_event).unwrap();
println!("Serialized: {}", json);


// Adjacently tagged enum

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t", content = "c")]
enum Message {
    Text(String),
    Image { url: String, width: u32, height: u32 },

}

// fn main()...

let msg = Message::Image {
    url: "http://example.com/cat.jpg".to_string(),
    width: 600,
    height:500,
};

let json = serde_json::to_string(&msg).unwrap();
println!("Serialized: {}", json);


}


