#[derive(Clone)]
pub struct Command {
    pub name: String,
    pub action: fn(),
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }
}

impl Command {
    pub fn new(name: String, action: fn()) -> Self {
        Command { name, action }
    }
}

pub fn build_commands_list() -> Vec<Command> {
    vec![
        Command::new("foo".to_string(), foo),
        Command::new("bar".to_string(), bar),
    ]
}

fn foo() {
    println!("foo command called");
}

fn bar() {
    println!("bar command called");
}
