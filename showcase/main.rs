//! Standalone Rust template. No dependencies, secrets, or network access.

struct Assistant {
    name: String,
}

impl Assistant {
    fn new(name: &str) -> Self {
        Self { name: name.to_owned() }
    }

    fn greeting(&self) -> String {
        format!("Hello from {}!", self.name)
    }
}

fn main() {
    let assistant = Assistant::new("Lucy");
    println!("{}", assistant.greeting());
}
