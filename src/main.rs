use std::io::stdin;
#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}
impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_lowercase(),
        }
    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}
fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
    .read_line(&mut your_name)
    .expect("failed to read line");
    your_name
        .trim()
        .to_lowercase()
    

}
fn main() {
    let mut visitor_list = vec![
        Visitor::new("enzo", "Hello enzo, enjoy your treehouse "),
        Visitor::new("alessandro", "Hi alessandro, your cheese is in the fridge "),
        Visitor::new("emma", "Why is emma here?"),
        Visitor::new("toto", "New friend"),
    ];
    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {    // (1)
                    break;  // (2)
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, "New friend"));
                }
            }
        }
    }
}


