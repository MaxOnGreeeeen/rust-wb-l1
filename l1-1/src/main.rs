trait Action {
    fn say(&self);
}

struct Person {
    name: String,
}
impl Person {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
impl Action for Person {
    fn say(&self) {
        println!("Hello {}", self.name);
    }
}

fn main() {
    Person::new("Test").say();
}
