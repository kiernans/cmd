pub struct Config {
    command: String,
    flags: Vec<String>
}

impl Config {
    pub fn new() -> Self {
        Self {
            command: String::new(),
            flags: vec![String::new()]
        }
    }

    pub fn set_config(&mut self, command: &str, flags: &[String]) {
        self.command = String::from(command);
        self.flags = flags.iter().map(|x| x.to_string()).collect();
    }

    pub fn print(&self) {
        println!("Command: {}", self.command);
        println!("Flags: {:?}", self.flags);
    }
}