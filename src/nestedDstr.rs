fn main {
// Nested Data structures

#[derive(Serialize, Deserialize, Debug)]
struct Employee {
    name: String,
    #[serde(flatten)] // flatten address fields
    address: Address,
}

// fn main()...

let employee = Employee {
    name: "Bob".to_string(),
    address: Address {
        street: "011 Jhb Rust lane".to_string(),
        city: "Kernelville".to_string(),
    },
};

let json = serde_json::to_string(&employee).unwrap();
println!("Serialized: {}", json);

}