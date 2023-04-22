use shortcut_gpt::{run, Config};
use std::process;

fn main() {
    let config = Config::load_config();

    if let Err(e) = run(&config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
