use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return;
    }

    match args[1].as_str() {
        "init" => init_main(),
        "left" => generate_left_side(),
        "right" => generate_right_side(),
        _ => {}
    }
}

fn init_main() {}

fn generate_left_side() {}

fn generate_right_side() {}