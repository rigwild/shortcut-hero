use shortcut_hero::{run, Config};

fn main() {
    let config = Config::load_config();
    run(&config);
}
