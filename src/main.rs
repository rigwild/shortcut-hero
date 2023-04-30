use std::process;

use shortcut_hero::{run, Config};

fn main() {
    let config = Config::load_config();

    if let Err(e) = run(&config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
