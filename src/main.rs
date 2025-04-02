use serde::{Serialize, Deserialize};

fn main() {
    // Serde's key feature and functionality
    
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

    // Output: {"name": "bonga", "age": 31, "is_active": true } 

    // Deserialize back to a Person
    let deserialized: Person = serde_json::from_str(&bonga).unwrap();
    println!("Deserialized: {:?}", deserialized);

    println!("Hello, world! Here is where I explore hacking LLMs, AI agents with Serde...");

   // Customizing structs with Serde attributes
   // - Field Renaming and Skipping
    
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "snake_case")]
    struct User {
        #[serde(rename = "id")] // Rename a single field
        user_id: u64,
        email: String,
        #[serde(skip_serializing)] // Skip field during serialization
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

    // SECURE pattern to prevent invalid USER ID


    #[derive(Serialize, Deserialize, Debug)]

    #[serde(tag = "type")]

    enum Event {
    Login {
        #[serde(deserialize_with = "strict_u64")]
        user_id: u64,
        timestamp: String,
    },

    Logout {
        #[serde(deserialize_with = "strict_u64")]
        user_id: u64,
    },
    
    Error(String),
}

fn strict_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let num: u64 = Deserialize::deserialize(deserializer)?;
    if num > 999_999 { // Example limit
        return Err(serde::de::Error::custom("user_id too large!"));
    }
    Ok(num)
}


    // Adjacently tagged enum
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag = "t", content = "c")]
    enum Message {
        Text(String),
        Image { url: String, width: u32, height: u32 },
    }

    let msg = Message::Image {
        url: "http://example.com/cat.jpg".to_string(),
        width: 600,
        height: 500,
    };

    let json = serde_json::to_string(&msg).unwrap();
    println!("Serialized: {}", json);

    // Deserialization with error handling
    let invalid_json = r#"{"name": "Tobu", "age": "Twenty"}"#; // "age" is a string, not a u8

  
}