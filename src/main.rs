use std::env;
use cmd::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (command, rest) = (&args[1], &args[2..]);

    let mut config = Config::new();

    config.set_config(command, rest);
    config.print();

}
