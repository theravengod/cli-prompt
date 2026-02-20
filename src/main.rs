use std::env;
use colored::{Color, Colorize};

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

fn init_main() {
    let script = r#"
        _my_prompt_precmd() {
            PROMPT="$(cli-prompt left) "
            RPROMPT="$(cli-prompt right)"
        }

        # Load ZSH's hook system and add our function
        autoload -Uz add-zsh-hook
        add-zsh-hook precmd _my_prompt_precmd

        # Optional: This removes the right prompt from previous lines after you press Enter,
        # keeping your terminal history clean.
        setopt transient_rprompt
    "#;
    println!("{}", script);
}

fn generate_left_side() {
    if let Ok(cwd) = env::current_dir() {
        let mut path_str = cwd.to_string_lossy().to_string();

        // Replace the home directory path with '~' for a cleaner look
        if let Ok(home) = env::var("HOME") {
            if path_str.starts_with(&home) {
                path_str = path_str.replacen(&home, "~", 1);
            }
        }

        //print!("%F{{blue}}{}%f", path_str);
        print!("{}", format_path(path_str, Color::BrightBlue, Color::BrightWhite))
    }
}

fn format_path(path_str: String, bg_color: Color, fg_color: Color) -> String {
    format!("{}{}{}{}",
            "█".color(bg_color),
            path_str.on_color(bg_color).color(fg_color),
            "█".color(bg_color),
            "".color(bg_color)
    )
}

fn generate_right_side() {
    let host = hostname::get()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    // Use ZSH prompt escapes for colors: %F{green} starts green, %f resets it
    print!("%F{{green}}{}%f", host);
}